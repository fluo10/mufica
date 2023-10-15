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

pub struct LocalHistories {
    content: Vec<LocalHistory>,
}
