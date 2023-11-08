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

#[derive(Clone, Debug, PartialEq,)]
pub enum MessageSender {
    User(Option<String>),
    Agent,
    // System,
}

#[derive(Clone, Debug, PartialEq,)]
pub struct Message {
    pub text: String,
    pub sender: MessageSender,
}

#[derive(Clone, Debug, PartialEq,)]
pub struct PlainHistory {
    sort_date: DateTime<Local>,
    inner: Vec<Message>,
}

impl PlainHistory {
    fn to_text_generation_webui_history(&self) -> TextGenerationWebuiHistory {
        todo!()
    }
}

impl From<PlainHistories> for PlainHistory {
    fn from(h: PlainHistories) -> Self {
        todo!()
    }
}

impl From<TextGenerationWebuiHistory> for PlainHistory {
    fn from(h: TextGenerationWebuiHistory) -> Self {
        todo!()
    }
}




#[derive(Clone, Debug)]
pub struct PlainHistories {
    inner: Vec<PlainHistory>,
}

impl PlainHistories {
    fn to_text_generation_webui_history(&self) -> TextGenerationWebuiHistory {
        PlainHistory::from(self.clone()).to_text_generation_webui_history()
    }
}

impl From<MatrixHistory> for PlainHistories {
    fn from(h: MatrixHistory) -> Self {
        todo!()
    }
}

pub enum MutexHistory {
    TextGenerationWebui(Arc<Mutex<TextGenerationWebuiHistory>>),
    Matrix(Arc<Mutex<MatrixHistory>>),
}

impl MutexHistory{
    async fn to_plain_histories(&self) -> PlainHistories {
        match *self {
            Self::TextGenerationWebui(ref x) => {
                PlainHistories{
                    inner: vec![PlainHistory::from(x.lock().await.clone())],
                }
            },
            Self::Matrix(ref x) => {
                x.lock().await.clone().into()
            },
        }
    }

    async fn to_text_generation_webui_history(&self) -> TextGenerationWebuiHistory {
        self.to_plain_histories().await.to_text_generation_webui_history()
    }

}

pub struct MutexHistories {
    inner: Vec<MutexHistory>,
}
impl MutexHistories {
    pub fn iter(&self) -> impl Iterator<Item=&MutexHistory> {
        self.inner.iter()
    }
    pub async fn to_plain_histories(&self) -> PlainHistories {
        let mut v = Vec::new();
        for history in self.iter() {
            v.append(&mut history.to_plain_histories().await.inner);
        }
        PlainHistories{inner: v}
    }
        
}

