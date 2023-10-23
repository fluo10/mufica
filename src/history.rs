use tokio::sync::Mutex;
use std::sync::Arc;
use crate::{MatrixHistory, Result};

use chrono::{DateTime, Local};
use std::convert::{From, Into};
use std::ops::Deref;

pub enum MessageSender {
    User(Option<String>),
    Agent,
    // System,
}

pub struct Message {
    text: String,
    sender: MessageSender,
}

pub struct History {
    start_date: DateTime<Local>,
    end_data: DateTime<Local>,
    content: Vec<Message>,
}

pub struct Histories {
    content: Vec<History>,
}

pub enum LocalHistory {
    TextGenerationWebui(Arc<Mutex<text_generation_webui_api::History>>),
    Matrix(Arc<Mutex<MatrixHistory>>),
}

impl LocalHistory{
    async fn to_histories(&self) -> Histories {
        match *self {
            LocalHistory::TextGenerationWebui(ref x) => {
                todo!()
            },
            LocalHistory::Matrix(ref x) => {
                todo!()
            },
        }
    }
}

pub struct LocalHistories {
    content: Vec<LocalHistory>,
}
impl LocalHistories {
    pub fn iter(&self) -> impl Iterator<Item=&LocalHistory> {
        self.content.iter()
    }
    pub async fn to_histories(&self) -> Histories {
        let mut v = Vec::new();
        for history in self.iter() {
            v.append(&mut history.to_histories().await.content);
        }
        Histories{content: v}
    }
        
}
