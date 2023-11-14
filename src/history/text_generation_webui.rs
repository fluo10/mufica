use crate::history::{Message, MessageSender, PlainHistory, PlainHistories};
use text_generation_webui_api::History;

use chrono::{DateTime, Local};

use std::default::Default;


#[derive(Clone, Debug, PartialEq, Default)]
pub struct TextGenerationWebuiHistory {
    pub sort_date: Option<DateTime<Local>>,
    pub inner: History,
}

impl TextGenerationWebuiHistory {
    pub fn new() -> Self{
        Self::default()
    }
    pub fn push_message(&mut self, m: &Message) {
        match (m.sender.clone(), self.inner.pop()){
            (MessageSender::User(_), None) => {
                self.inner.push((&m.text, &String::new()));
            },
            (MessageSender::User(_), Some((x, y))) => {
                if y.is_empty() {
                    self.inner.push((&(x + &m.text), &y));
                } else {
                    self.inner.push((&x, &y));
                    self.inner.push((&m.text, &String::new()));
                }
            },
            (MessageSender::Agent, None) => {
                self.inner.push((&String::new(), &m.text));
            },
            (MessageSender::Agent, Some((x, y))) => {
                self.inner.push((&x, &(y + &m.text)));
            },
            (_, _) => panic!(),
        }
    }
}

impl From<History> for TextGenerationWebuiHistory {
    fn from(h: History) -> Self {
        Self {
            sort_date: None,
            inner: h,
        }
    }
}

impl From<PlainHistory> for TextGenerationWebuiHistory {
    fn from(h: PlainHistory) -> Self {
        let mut result = Self::new();
        for message in h.iter() {
            result.push_message(&message);
        }
        result
    }
}
impl From<PlainHistories> for TextGenerationWebuiHistory {
    fn from(h: PlainHistories) -> Self {
        let mut result = Self::new();
        for history in h.iter() {
            for message in history.iter() {
                result.push_message(&message);
            }
        }
        result 
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::history::{Message, MessageSender, PlainHistory};

    #[test]
    fn to_plain_history () {
        let date = chrono::Local::now();
        let text_generation_webui_history_str = r#"{ "internal" : [["How are you?", "I'm fine. And you?"]], "visible": [["How are you?", "I'm fine. And you?"]]}"#;
        let text_generation_webui_history_inner: History = serde_json::from_str(&text_generation_webui_history_str).unwrap();
        let text_generation_webui_history = TextGenerationWebuiHistory::from(text_generation_webui_history_inner);
        let plain_history = PlainHistory { 
            sort_date: date,
            inner: vec![
                Message { sender: MessageSender::User(None), text: "How are you?".to_string() },
                Message { sender: MessageSender::Agent, text: "I'm fine. And you?".to_string() },
            ],
        };
        assert_eq!(plain_history, PlainHistory::from(text_generation_webui_history))
    }

    #[test]
    fn from_plain_history() {
        let date = chrono::Local::now();
        let text_generation_webui_history_str = r#"{ "internal" : [["How are you?", "I'm fine. And you?"]], "visible": [["How are you?", "I'm fine. And you?"]]}"#;
        let text_generation_webui_history_inner: History = serde_json::from_str(&text_generation_webui_history_str).unwrap();
        let text_generation_webui_history = TextGenerationWebuiHistory::from(text_generation_webui_history_inner);
        let plain_history = PlainHistory { 
            sort_date: date,
            inner: vec![
                Message { sender: MessageSender::User(None), text: "How are you?".to_string() },
                Message { sender: MessageSender::Agent, text: "I'm fine. And you?".to_string() },
            ],
        };
        assert_eq!(text_generation_webui_history, TextGenerationWebuiHistory::from(plain_history));
    }
}


