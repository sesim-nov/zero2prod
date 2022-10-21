use crate::domain::ListSubscriberEmail;
use reqwest::Client;

pub struct EmailMessage {
    pub recipient: ListSubscriberEmail,
    pub subject: String,
    pub body_text: String,
    pub body_html: String,
}

pub struct EmailClient {
    sender: ListSubscriberEmail,
    http_client: Client,
    api_url: String,
}

impl EmailClient {
    pub fn new(sender: ListSubscriberEmail, api_url: String) -> Self {
        Self {
            sender,
            http_client: Client::new(),
            api_url,
        }
    }
    pub fn get_sender(&self) -> &ListSubscriberEmail {
        &self.sender
    }
    pub async fn send_mail(_message: EmailMessage) -> Result<(), String> {
        todo!();
    }
}
