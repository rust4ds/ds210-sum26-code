use kalosm::language::*;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct ChatbotV3 {
    // What should you store inside your Chatbot type?
    // The model? The chat_session?
    // Storing a single chat session is not enough: it mixes messages from different users
    // together!
    // Need to store one chat session per user.
    // Think of some kind of data structure that can help you with this.
    model: Llama,
    chat_sessions: HashMap<String, Chat<Llama>>,

}

impl ChatbotV3 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV3 {
        return ChatbotV3 {
            // Make sure you initialize your struct members here
            model: model,
            chat_sessions: HashMap::new(),
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        // Add your code for chatting with the agent while keeping conversation history here.
        // Notice, you are given both the `message` and also the `username`.
        // Use this information to select the correct chat session for that user and keep it
        // separated from the sessions of other users.
        let chat_session = match self.chat_sessions.contains_key(&username) {
            false => {
                let session = self.model.chat().with_system_prompt("The assistant will act like a pirate");
                self.chat_sessions.insert(username.clone(), session);
                self.chat_sessions.get_mut(&username).unwrap()
            }
            true => {
                self.chat_sessions.get_mut(&username).unwrap()
            }
        };

        let output = chat_session.add_message(message).await.unwrap();
        return output;
    }

    #[allow(dead_code)]
    pub fn get_history(&self, username: String) -> Vec<String> {
        // Extract the chat message history for the given username
        // Hint: think of how you can retrieve the Chat object for that user, when you retrieve it
        // you may want to use https://docs.rs/kalosm/0.4.0/kalosm/language/struct.Chat.html#method.session
        // to then retrieve the history!
        let user_session = self.chat_sessions.get(&username).unwrap().session();
        let chat = user_session.unwrap();
        let chat_history = chat.history();
        return chat_history.iter().skip(1).map(|msg| msg.content().to_string()).collect();
    }
}