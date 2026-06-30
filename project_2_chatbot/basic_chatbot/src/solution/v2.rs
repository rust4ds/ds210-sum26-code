use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV2 {
    chat_session: Chat<Llama>,
}

impl ChatbotV2 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV2 {
        return ChatbotV2 {
            chat_session: model
                .chat()
                .with_system_prompt("The assistant will act like a pirate"),
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, message: String) -> String {
        let response_message = self.chat_session.add_message(message).await.unwrap();
        return response_message;
    }
}