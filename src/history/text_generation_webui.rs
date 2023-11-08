use crate::history::{PlainHistory, PlainHistories};
use text_generation_webui_api::History;

use chrono::{DateTime, Local};


#[derive(Clone, Debug, PartialEq)]
pub struct TextGenerationWebuiHistory {
    sort_date: Option<DateTime<Local>>,
    inner: History,
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
        todo!()
    }
}
impl From<PlainHistories> for TextGenerationWebuiHistory {
    fn from(h: PlainHistories) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::history::{Message, MessageSender, PlainHistory};

    #[test]
    fn to_plain_history () {
        let date = chrono::Local::now();
        let text_generation_webui_history_str = r#"{ "internal" : [["How are you?", "I'm fine. And you?"]], "visible": [["How are you?", "Im fine. And you?"]]}"#;
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
        let text_generation_webui_history_str = r#"{ "internal" : [["How are you?", "I'm fine. And you?"]], "visible": [["How are you?", "Im fine. And you?"]]}"#;
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


