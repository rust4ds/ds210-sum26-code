use std::num::NonZero;
use kalosm::language::*;
use file_chatbot::solution::file_library;
use fix::fixed_load_session;
use lru::LruCache;

pub struct ChatbotV5 {
    model: Llama,
    cache: LruCache<String, Chat<Llama>>,
}

impl ChatbotV5 {
    pub fn new(model: Llama) -> ChatbotV5 {
        return ChatbotV5 {
            model: model,
            cache: LruCache::new(NonZero::new(2).unwrap()),
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_mut(&username);

        match cached_chat {
            None => {
                println!("chat_with_user: {username} is not in the cache!");
                // The cache does not have the chat. What should you do?
                let mut chat_session = match file_library::load_chat_session_from_file(filename) {
                    None => {
                        self.model.chat().with_system_prompt("The assistant will act like a pirate")
                    }
                    Some(session) => {
                    let fresh_chat = self.model.chat().with_system_prompt("The assistant will act like a pirate");
                    fixed_load_session(fresh_chat, session)
                    }
                };
                let output = chat_session.add_message(message).await.unwrap();
                let chat_session_clone = chat_session.clone();
                let session = chat_session_clone.session().unwrap();
                file_library::save_chat_session_to_file(filename, &session);
                self.cache.put(username.clone(), chat_session);
                return output;
            }
            Some(chat_session) => {
                println!("chat_with_user: {username} is in the cache! Nice!");
                // The cache has this chat. What should you do?
                let output = chat_session.add_message(message).await.unwrap();
                let session = chat_session.session().unwrap();
                file_library::save_chat_session_to_file(filename, &session);
                return output;
            }
        }
    }

    pub fn get_history(&mut self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_mut(&username);

        match cached_chat {
            None => {
                println!("get_history: {username} is not in the cache!");
                // TODO: The cache does not have the chat. What should you do?
                // Your code goes here.
                match file_library::load_chat_session_from_file(filename) {
                    None => {
                        return Vec::new();
                    }
                    Some(session) => {
                        let history = session.history().iter().skip(1).map(|msg| msg.content().to_string()).collect();
                        let fresh_chat = self.model.chat().with_system_prompt("The assistant will act like a pirate");
                        let chat_session = fixed_load_session(fresh_chat, session);
                        self.cache.put(username.clone(), chat_session);
                        return history;
                    }
                }
            }
            Some(chat_session) => {
                println!("get_history: {username} is in the cache! Nice!");
                // TODO: The cache has this chat. What should you do?
                // Your code goes here.
                let session = chat_session.session().unwrap();
                return session.history().iter()
                            .filter(|msg| msg.role() != MessageType::SystemPrompt)
                            .map(|msg| msg.content().to_string())
                            .collect();   
            }
        }
    }
}
