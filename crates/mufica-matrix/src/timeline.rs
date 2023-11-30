use std::collections::HashMap;

use crate::Result;

use matrix_sdk::{
    ruma::{
        OwnedRoomId,
        UInt,
    },
    room::{Messages, MessagesOptions},
    deserialized_responses::TimelineEvent,
    Client,
};

#[derive(Clone, Debug)]
pub struct MatrixTimeline{
    pub inner: HashMap<OwnedRoomId, Vec<TimelineEvent>>
}


impl MatrixTimeline {
    pub async fn reflesh(&mut self, client: &Client) -> Result<()>{
        let rooms = client.joined_rooms();
        for room in rooms {
            let room_id = room.room_id().to_owned();
            let mut options = MessagesOptions::forward();
            let mut events: Vec<TimelineEvent> = Vec::new();
            loop {
                let mut messages = room.messages(options).await?;
                if let Some(x) = messages.end {
                        options = MessagesOptions::forward();
                        options.from = Some(x);
                    } else {
                        break;
                    }
                    events.append(&mut messages.chunk);
                }
            self.inner.insert(room_id, events);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_history_to_plain() {
        todo!()
    }
}
