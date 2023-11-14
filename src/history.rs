#[cfg(feature="matrix")]
mod matrix;
#[cfg(feature="text-generation-webui")]
mod text_generation_webui;

#[cfg(feature="matrix")]
pub use matrix::MatrixHistory;
#[cfg(feature="text-generation-webui")]
pub use text_generation_webui::TextGenerationWebuiHistory;

use tokio::sync::Mutex;
use std::sync::Arc;
use crate::errors::Result;

use chrono::{DateTime, Local, TimeZone};
use std::convert::{From, Into};
use std::ops::Deref;
use std::iter::{Iterator, IntoIterator};
use std::slice::{Iter,IterMut};

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

impl From<PlainHistories> for PlainHistory {
    fn from(h: PlainHistories) -> Self {
        todo!()
    }
}

#[cfg(feature="text-generation-webui")]
impl From<TextGenerationWebuiHistory> for PlainHistory {
    fn from(h: TextGenerationWebuiHistory) -> Self {
        let mut messages: Vec<Message> = Vec::new();
        for (x, y) in h.inner.iter() {
            if !x.is_empty() {
                messages.push(Message{
                    sender: MessageSender::User(None),
                    text: x.clone()
                });
            }
            if !y.is_empty() {
                messages.push(Message{
                    sender: MessageSender::Agent,
                    text: y.clone(),
                });
            }
        }
            
        Self {
            sort_date: Local.timestamp_opt(0,0).unwrap(),
            inner: messages,
        }

    }
}

impl Deref for PlainHistory {
    type Target = [Message];

    fn deref(&self) -> &[Message] {
        self.inner.deref()
    }
}


#[derive(Clone, Debug)]
pub struct PlainHistories {
    inner: Vec<PlainHistory>,
}

#[cfg(feature="matrix")]
impl From<MatrixHistory> for PlainHistories {
    fn from(h: MatrixHistory) -> Self {
        todo!()
    }
}

impl Deref for PlainHistories {
    type Target = [PlainHistory];

    fn deref(&self) -> &[PlainHistory] {
        self.inner.deref()
    }
}

pub enum MutexHistory {
    #[cfg(feature="text-generation-webui")]
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

    #[cfg(feature="text-generation-webui")]
    async fn to_text_generation_webui_history(&self) -> TextGenerationWebuiHistory {
        self.to_plain_histories().await.into()
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

