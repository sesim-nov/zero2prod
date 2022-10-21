use crate::domain::ListSubscriberEmail;

pub struct EmailMessage {
    pub recipient: ListSubscriberEmail,
    pub subject: String,
    pub body_text: String,
    pub body_html: String,
}

pub struct EmailClient {
    sender: ListSubscriberEmail,
}

impl EmailClient {
    pub fn new(sender: ListSubscriberEmail) -> Self {
        Self { sender }
    }
    pub fn get_sender(&self) -> &ListSubscriberEmail {
        &self.sender
    }
    pub async fn send_mail(_message: EmailMessage) -> Result<(), String> {
        todo!();
    }
}
