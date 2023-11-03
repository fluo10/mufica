mod matrix;
mod text_generation_webui;

pub use matrix::MatrixHistory;
pub use text_generation_webui::TextGenerationWebuiHistory;

use tokio::sync::Mutex;
use std::sync::Arc;
use crate::{Result};

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

pub struct PlainHistory {
    sort_date: DateTime<Local>,
    inner: Vec<Message>,
}

pub struct PlainHistories {
    inner: Vec<PlainHistory>,
}

pub enum MutexHistory {
    TextGenerationWebui(Arc<Mutex<TextGenerationWebuiHistory>>),
    Matrix(Arc<Mutex<MatrixHistory>>),
}

impl MutexHistory{
    async fn to_histories(&self) -> PlainHistories {
        match *self {
            Self::TextGenerationWebui(ref x) => {
                todo!()
            },
            Self::Matrix(ref x) => {
                todo!()
            },
        }
    }
}

pub struct MutexHistories {
    inner: Vec<MutexHistory>,
}
impl MutexHistories {
    pub fn iter(&self) -> impl Iterator<Item=&MutexHistory> {
        self.inner.iter()
    }
    pub async fn to_histories(&self) -> PlainHistories {
        let mut v = Vec::new();
        for history in self.iter() {
            v.append(&mut history.to_histories().await.inner);
        }
        PlainHistories{inner: v}
    }
        
}
