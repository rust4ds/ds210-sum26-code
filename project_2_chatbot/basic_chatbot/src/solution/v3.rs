use kalosm::language::*;
use std::collections::HashMap;

const SYSTEM_PROMPT: &str = "The assistant will act like a pirate";

#[allow(dead_code)]
pub struct ChatbotV3 {
    model: Llama,
    chat_sessions: HashMap<String, Chat<Llama>>,
}

impl ChatbotV3 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV3 {
        return ChatbotV3 {
            model,
            chat_sessions: HashMap::new(),
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        if !self.chat_sessions.contains_key(&username) {
            let chat_session = self
                .model
                .chat()
                .with_system_prompt(SYSTEM_PROMPT);
            self.chat_sessions.insert(username.clone(), chat_session);
        }

        let chat_session = self.chat_sessions.get_mut(&username).unwrap();
        let response_message = chat_session.add_message(message).await.unwrap();
        return response_message;
    }

    #[allow(dead_code)]
    pub fn get_history(&self, username: String) -> Vec<String> {
        if let Some(chat_session) = self.chat_sessions.get(&username) {
            let session = chat_session.session().unwrap();
            return session
                .history()
                .into_iter()
                .map(|message| message.content().to_string())
                .collect();
        }

        return Vec::new();
    }
}