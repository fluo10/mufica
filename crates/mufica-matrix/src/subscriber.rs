use crate::{Result, MatrixConfig, MatrixTimeline};
use matrix_sdk_ui::timeline::{Message, TimelineItem,RoomExt, Timeline, TimelineItemKind, TimelineItemContent, EventTimelineItem};
use eyeball_im::VectorDiff;
use imbl::Vector;


use std::{
    io::{self, Write},
    path::{Path, PathBuf},
    borrow::Borrow,
};

use matrix_sdk::{
    config::SyncSettings,
    matrix_auth::MatrixSession,
    ruma::{
        api::client::filter::FilterDefinition,
        events::room::message::{MessageType, OriginalSyncRoomMessageEvent},
        OwnedRoomId,
        UInt, EventId, UserId, OwnedUserId, OwnedEventId,
    },
    Client,LoopCtrl,Room, RoomState,
    room::{Messages, MessagesOptions},
    deserialized_responses::TimelineEvent,
};
use ruma::{
    events::{
        MessageLikeEventContent,
        room::message::{ForwardThread, RoomMessageEventContentWithoutRelation, TextMessageEventContent, }
    }
};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;
use std::ops::DerefMut;
use futures_util::StreamExt;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct ClientSession {
    homeserver: String,
    db_path: PathBuf,
    passphrase: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct FullSession {
    client_session: ClientSession,
    user_session: MatrixSession,
    #[serde(skip_serializing_if = "Option::is_none")]
    sync_token: Option<String>,
}

/// Restore a previous session.
async fn restore_session(session_file: &Path) -> Result<(Client, Option<String>)> {
    println!("Previous session found in '{}'", session_file.to_string_lossy());

    // The session was serialized as JSON in a file.
    let serialized_session = fs::read_to_string(session_file).await?;
    let FullSession { client_session, user_session, sync_token } =
        serde_json::from_str(&serialized_session)?;

    // Build the client with the previous settings from the
    let client = Client::builder()
        .homeserver_url(client_session.homeserver)
        .sqlite_store(client_session.db_path, Some(&client_session.passphrase))
        .build()
        .await?;

    println!("Restoring session for {}…", user_session.meta.user_id);

    client.restore_session(user_session).await?;

    Ok((client, sync_token))
}

/// Login with a new device.
async fn login(data_dir: &Path, session_file: &Path, username: &str, password: &str) -> Result<Client> {
    println!("No previous session found, logging in…");

    let (client, client_session) = build_client(data_dir).await?;
    let matrix_auth = client.matrix_auth();

    loop {
        print!("\nUsername: ");
        io::stdout().flush().expect("Unable to write to stdout");
        match matrix_auth
            .login_username(username, password)
            .initial_device_display_name("persist-session client")
            .await {
                Ok(_) => {
                    println!("Logged in as {username}");
                    break;
                }
                Err(error) =>{
                    println!("Error logging in: {error}");
                    println!("Please try again\n");
                }
            }
    }

    let user_session = matrix_auth.session().expect("A logged-in client should have a session");
    let serialized_session =
        serde_json::to_string(&FullSession { client_session, user_session, sync_token: None })?;
    fs::write(session_file, serialized_session).await?;

    println!("Session persisted in {}", session_file.to_string_lossy());
    Ok(client)
}

async fn build_client(data_dir: &Path) -> Result<(Client, ClientSession)> {
    let mut rng = thread_rng();
    let db_subfolder: String =
        (&mut rng).sample_iter(Alphanumeric).take(7).map(char::from).collect();
    let db_path = data_dir.join(db_subfolder);
    let passphrase: String =
        (&mut rng).sample_iter(Alphanumeric).take(32).map(char::from).collect();
    loop {
        let mut homeserver = String::new();

        print!("Homeserver URL: ");
        io::stdout().flush().expect("Unable to write to stdout");
        io::stdin().read_line(&mut homeserver).expect("Unable to read user input");

        println!("\nChecking homeserver…");

        match Client::builder()
            .homeserver_url(&homeserver)
            .sqlite_store(&db_path, Some(&passphrase))
            .build()
            .await{
                Ok(client) => return Ok((client, ClientSession { homeserver, db_path, passphrase })),
                Err(error) => match &error {
                    matrix_sdk::ClientBuildError::AutoDiscovery(_)
                        | matrix_sdk::ClientBuildError::Url(_)
                        | matrix_sdk::ClientBuildError::Http(_) => {
                            println!("Error checking the homeserver: {error}");
                            println!("Please try again\n");
                        }
                    _ => {
                        return Err(error.into());
                    }
                },
            }
    }
}
async fn sync(client: Client,initial_sync_token: Option<String>, session_file: &Path, history: Arc<Mutex<MatrixTimeline>>,) -> Result<()> {
    println!("Launching a first sync to ignore past messages…");
    let filter = FilterDefinition::with_lazy_loading();

    let mut sync_settings = SyncSettings::default().filter(filter.into());
    if let Some(sync_token) = initial_sync_token {
        sync_settings = sync_settings.token(sync_token);
    }
    loop {
        match client.sync_once(sync_settings.clone()).await {
            Ok(response) => {
                sync_settings = sync_settings.token(response.next_batch.clone());
                persist_sync_token(session_file, response.next_batch).await?;
                break;
            }
            Err(error) => {
                println!("An error occurred during initial sync: {error}");
                println!("Trying again…");
            }
        }
    }
    println!("The client is ready! Listening to new messages…");

    client.add_event_handler(on_room_message);
    client.sync_with_result_callback(sync_settings, |sync_result| async move {
        let response = sync_result?;
        persist_sync_token(session_file, response.next_batch)
            .await
            .map_err(|err| matrix_sdk::Error::UnknownError(err.into()))?;

        Ok(LoopCtrl::Continue)
    })
    .await?;

    Ok(())
}

async fn sync_once(client: Client,initial_sync_token: Option<String>, session_file: &Path, history: Arc<Mutex<MatrixTimeline>>,) -> Result<()> {
    println!("Launching a first sync to ignore past messages…");
    let filter = FilterDefinition::with_lazy_loading();

    let mut sync_settings = SyncSettings::default().filter(filter.into());
    if let Some(sync_token) = initial_sync_token {
        sync_settings = sync_settings.token(sync_token);
    }
    loop {
        match client.sync_once(sync_settings.clone()).await {
            Ok(response) => {
                sync_settings = sync_settings.token(response.next_batch.clone());
                persist_sync_token(session_file, response.next_batch).await?;
                break;
            }
            Err(error) => {
                println!("An error occurred during initial sync: {error}");
                println!("Trying again…");
            }
        }
    }
    Ok(())
}
async fn persist_sync_token(session_file: &Path, sync_token: String) -> Result<()> {
    let serialized_session = fs::read_to_string(session_file).await?;
    let mut full_session: FullSession = serde_json::from_str(&serialized_session)?;

    full_session.sync_token = Some(sync_token);
    let serialized_session = serde_json::to_string(&full_session)?;
    fs::write(session_file, serialized_session).await?;

    Ok(())
}
async fn on_room_message(event: OriginalSyncRoomMessageEvent, room: Room) {
    if room.state() != RoomState::Joined {
        return;
    }
    let MessageType::Text(text_content) = &event.content.msgtype else { return };

    let room_name = match room.display_name().await {
        Ok(room_name) => room_name.to_string(),
        Err(error) => {
            println!("Error getting room display name: {error}");
            room.room_id().to_string()
        }
    };

    println!("[{room_name}] {}: {}", event.sender, text_content.body)
}

fn get_new_text_message(user_id: &UserId, event: &VectorDiff<Arc<TimelineItem>>) -> Option<(EventTimelineItem, String)>{
    if let VectorDiff::PushBack{value: timeline_item} = event {
        if let TimelineItemKind::Event(event_timeline_item) = timeline_item.kind() {
            let sender: OwnedUserId = event_timeline_item.sender().to_owned();
            let event_id: OwnedEventId = event_timeline_item.event_id()?.to_owned();
            if sender == user_id { return None }
            if let TimelineItemContent::Message(message) = event_timeline_item.content() {
                if let MessageType::Text(text_message_content) = message.msgtype() {
                    return  Some((event_timeline_item.clone(), text_message_content.body.clone()));
                }
            }
        }
    }
    None
}

async fn reply_text_message(timeline: &Timeline , message: &str, reply_item: &EventTimelineItem) {
    let event_content = RoomMessageEventContentWithoutRelation::new(MessageType::Text(TextMessageEventContent::markdown(message)));
    
    timeline.send_reply(event_content, reply_item, ForwardThread::Yes);
}


#[derive(Clone, Debug)]
pub struct MatrixClient<'a>{
   pub config: &'a MatrixConfig, 
    pub client: Client,
    pub sync_token: Option<String>,
}

impl<'a> MatrixClient<'a>{
    pub async fn new (config:&'a MatrixConfig) -> Result<Self> {
        let data_dir = PathBuf::from(&config.data_dir);
        let session_file = data_dir.join("session");

        let (client, sync_token) = if session_file.exists() {
            restore_session(&session_file).await?
        } else {
            (login(&data_dir, &session_file, &config.username, &config.password).await?, None)
        };
        let sync_settings = SyncSettings::default();

        client.sync_once(sync_settings.clone()).await?;

        let mut worker =  Self {
            config: config,
            client: client,
            sync_token: sync_token,
        };
        Ok(worker)
    }
    pub async fn timelines(&self) -> Result<Vec<Arc<Mutex<MatrixTimeline>>>>{
        let mut result = Vec::new();
        for room in self.client.rooms().iter() {
            let timeline = room.timeline().await;
            result.push( Arc::new(Mutex::new(timeline.items().await.into())));
        }
        Ok(result)

    }
    pub async fn subscribe(& self) -> Result<Vec<(Arc<Mutex<MatrixTimeline>>, MatrixSubscriber)>>{
        let mut result = Vec::new();
        for room in self.client.rooms().iter() {
            let timeline = room.timeline().await;
            let history = Arc::new(Mutex::new(MatrixTimeline::new()));
            result.push((
                    history.clone(),
                    MatrixSubscriber {
                        history:history.clone(), 
                        timeline: timeline,
                        room: room.clone(),
                    }
                    ));
        }
        Ok(result)
    }
}

#[derive(Debug)]
pub struct MatrixSubscriber{
    pub history: Arc<Mutex<MatrixTimeline>>,
    pub timeline: Timeline,
    pub room: Room,
}

impl MatrixSubscriber{

    pub async fn sync<F>(&mut self, f: F) -> Result<()> where
    F: Fn(&str) -> Result<String> {
        let (timeline_items, mut timeline_stream) = self.timeline.subscribe().await;
        let mut history = self.history.lock().await;
        history.deref_mut().append(timeline_items);

        while let Some(x) = timeline_stream.next().await {
            if let Some((event, text)) = get_new_text_message(self.user_id().borrow(), &x) {
                let reply_text = f(&text).unwrap();
                reply_text_message(&self.timeline, &reply_text, &event);
                    x.apply(self.history.lock().await.deref_mut());
            }
        }
            Ok(())

    }
    pub fn client(&self) -> Client {
        self.room.client()
    }
    pub fn user_id(&self) -> OwnedUserId {
        self.client().user_id().unwrap().to_owned()
    }
}
