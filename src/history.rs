use chrono::{DateTime, Local};
use std::convert::From;

pub struct Message{
    pub text: String,
}

impl From<TimestampedMessage> for Message {
    fn from(m: TimestampedMessage) -> Message {
        Message{
            text: m.text
        }
    }
}

pub struct PairedMessages{
   pub user: Message,
    pub agent: Message,
}
impl From<PairedTimestampedMessages> for PairedMessages {
    fn from(m: PairedTimestampedMessages) -> PairedMessages {
        PairedMessages{
            user: m.user.into(),
            agent: m.agent.into(),
        }
    }
}

pub struct History {
    content: Vec<PairedMessages>,
}

impl From<TimestampedHistory> for History {
    fn from(h: TimestampedHistory) -> History {
        History{
            content:  h.content.into_iter().map(|x| PairedMessages::from(x)).collect()
        }
    }
}


        
pub struct TimestampedMessage{
    pub timestamp: DateTime<Local>,
    pub text: String,
}

pub struct PairedTimestampedMessages{
    pub user: TimestampedMessage,
    pub agent: TimestampedMessage,
}
    

pub struct TimestampedHistory {
    content: Vec<PairedTimestampedMessages>,
}

