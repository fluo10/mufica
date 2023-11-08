use crate::{Result, Backend, MutexHistories, PlainHistory, MatrixConfig, MatrixHistory};


use std::{
    io::{self, Write},
    path::{Path, PathBuf},
};

use matrix_sdk::{
    config::SyncSettings,
    matrix_auth::MatrixSession,
    ruma::{
        api::client::filter::FilterDefinition,
        events::room::message::{MessageType, OriginalSyncRoomMessageEvent},
        OwnedRoomId,
        UInt,
    },
    Client,LoopCtrl,Room, RoomState,
    room::{Messages, MessagesOptions},
    deserialized_responses::TimelineEvent,
};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;
use std::ops::DerefMut;


fn timeline_items_to_history(timeline_items: Vec<TimelineEvent>) -> PlainHistory {
    todo!()
}




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
async fn login(data_dir: &Path, session_file: &Path) -> Result<Client> {
    println!("No previous session found, logging in…");

    let (client, client_session) = build_client(data_dir).await?;
    let matrix_auth = client.matrix_auth();

    loop {
        print!("\nUsername: ");
        io::stdout().flush().expect("Unable to write to stdout");
        let mut username = String::new();
        io::stdin().read_line(&mut username).expect("Unable to read user input");
        username = username.trim().to_owned();

        print!("Password: ");
        io::stdout().flush().expect("Unable to write to stdout");
        let mut password = String::new();
        io::stdin().read_line(&mut password).expect("Unable to read user input");
        password = password.trim().to_owned();

        match matrix_auth
            .login_username(&username, &password)
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
async fn sync(client: Client,initial_sync_token: Option<String>, session_file: &Path, history: Arc<Mutex<MatrixHistory>>,) -> Result<()> {
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

async fn sync_once(client: Client,initial_sync_token: Option<String>, session_file: &Path, history: Arc<Mutex<MatrixHistory>>,) -> Result<()> {
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

#[derive(Clone, Debug)]
pub struct MatrixWorker{
    pub host: String,
    pub session_file: PathBuf,
    pub client: Client,
    pub sync_token: Option<String>,
    pub history: Arc<Mutex<MatrixHistory>>,
    pub backend: Arc<Mutex<Backend>>,
}

impl MatrixWorker{
    pub async fn new(backend: &Arc<Mutex<Backend>>, config: MatrixConfig) -> Result<Self> {
        let data_dir = PathBuf::from(&config.data_dir);
        let session_file = data_dir.join("session");

        let (client, sync_token) = if session_file.exists() {
            restore_session(&session_file).await?
        } else {
            (login(&data_dir, &session_file).await?, None)
        };

        let mut worker =  Self {
            host: config.host.clone(),
            session_file: session_file,
            client: client,
            sync_token: sync_token,
            history: Arc::new(Mutex::new(MatrixHistory{inner: HashMap::new()})),
            backend: backend.clone(),
        };
        worker.reflesh_history().await?;
        Ok(worker)
    }


    pub async fn sync(self) -> Result<()> {
        sync(self.client, self.sync_token, self.session_file.as_path(), self.history).await
    }
    pub async fn sync_once(self) -> Result<()> {
        sync_once(self.client, self.sync_token, self.session_file.as_path(), self.history).await
    }

    pub async fn reflesh_history(&mut self) -> Result<()> {
        self.history.lock().await.reflesh(&self.client).await
    }
}
