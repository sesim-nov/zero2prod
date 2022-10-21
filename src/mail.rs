//! Components related to sending e-mail traffic. 
use crate::domain::ListSubscriberEmail;
use reqwest::Client;

/// Represents an e-mail message to be sent by an EmailClient. 
pub struct EmailMessage {
    /// The recipient of the email. 
    pub recipient: ListSubscriberEmail,
    /// The email subject
    pub subject: String,
    /// The body plaintext
    pub body_text: String,
    /// The HTML representation of the email body
    pub body_html: String,
}

/// An Email Client
///
/// This system is responsible for handling sending out email messages. 
///
pub struct EmailClient {
    /// The email used as the "From" address on emails being sent. 
    sender: ListSubscriberEmail,
    /// The HTTP client used to process the REST API requests to send emails. 
    http_client: Client,
    /// The base API url for the mail service used to send emails. 
    api_url: String,
}

impl EmailClient {
    /// Construct a new Email Client
    /// 
    /// # Arguments
    /// 
    /// * `sender` - a ListSubscriberEmail object representing the sender's address
    /// * `api_url` - a String representing the base API url used to send emails
    pub fn new(sender: ListSubscriberEmail, api_url: String) -> Self {
        Self {
            sender,
            http_client: Client::new(),
            api_url,
        }
    }
    /// Expose the sender email address. 
    pub fn get_sender(&self) -> &ListSubscriberEmail {
        &self.sender
    }
    /// Send out the given email
    ///
    /// # Arguments
    ///
    /// * `message`: an EmailMessage representing the email to be sent. 
    pub async fn send_mail(_message: EmailMessage) -> Result<(), String> {
        todo!();
    }
}
