use serde::{Serialize, Deserialize};
use text_generation_webui_api::{ChatApiRequest, ChatApiResponse, ModelApiRequest, ModelApiResponse, ModelApiResponseResult, Character, History};
use tokio::sync::Mutex;
use std::sync::Arc;

use crate::{Result, TextGenerationWebuiConfig};


#[derive(Debug)]
pub struct TextGenerationWebui {
    host: String,
    model: String,
    history: Arc<Mutex<History>>,
    character_name: Option<String>,
    character: Option<Character>,

}

impl TextGenerationWebui {
    async fn load_model(&mut self) -> Result<()> {
        // Request current model
        let info = ModelApiRequest::info().send(&self.host).await?;
        if let ModelApiResponseResult::Info(x) = info.result {
            if x.model_name != self.model {
                let model_list = ModelApiRequest::list().send(&self.host).await?;
                if let ModelApiResponseResult::List(x) = model_list.result {
                    if x.contains(&self.model) {
                        ModelApiRequest::load(&self.model).send(&self.host).await?;
                    }
                }
            }
        }
        Ok(())
    }
    async fn new(c: TextGenerationWebuiConfig) -> Result<TextGenerationWebui>{
        let history = c.get_history()?;
        let character:Option<Character> = Some(c.get_character()?);
        let mut service = Self {
            host: c.host,
            model: c.model,
            history: Arc::new(Mutex::new(history)),
            character_name: c.character,
            character: character,
        };
        service.load_model().await?;
        Ok(service)
    }
    async fn generate(&self, input: String, history: History) -> Result<String> {
        todo!();
        let response = match (self.character_name.as_ref(), self.character.as_ref()) {
            (Some(_), Some(x)) => ChatApiRequest::default().character(x).history(&history.into()).user_input(&input).send(&self.host).await?,
            (Some(x), None) => ChatApiRequest::default().character_name(x).history(&history.into()).user_input(&input).send(&self.host).await?,
            (None, _) => ChatApiRequest::default().history(&history.into()).user_input(&input).send(&self.host).await?,
        };
        todo!();
    }
}


