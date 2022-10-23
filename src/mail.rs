//! Components related to sending e-mail traffic.
use crate::domain::ListSubscriberEmail;
use reqwest::{Client, Response};
use secrecy::{ExposeSecret, Secret};

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

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
struct EmailApiRequest {
    from: String,
    to: String,
    subject: String,
    text_body: String,
    html_body: String,
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
    /// The API token used to authenticate with the mail application
    auth_token: Secret<String>,
}

impl EmailClient {
    /// Construct a new Email Client
    ///
    /// # Arguments
    ///
    /// * `sender` - a ListSubscriberEmail object representing the sender's address
    /// * `api_url` - a String representing the base API url used to send emails
    pub fn new(sender: ListSubscriberEmail, api_url: String, auth_token: Secret<String>) -> Self {
        Self {
            sender,
            http_client: Client::new(),
            api_url,
            auth_token,
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
    pub async fn send_mail(&self, message: EmailMessage) -> Result<Response, reqwest::Error> {
        let client = &self.http_client;
        let url = format!("{}/email", self.api_url);
        let body = EmailApiRequest {
            to: message.recipient.as_ref().to_owned(),
            from: self.sender.as_ref().to_owned(),
            subject: message.subject,
            html_body: message.body_html,
            text_body: message.body_text,
        };
        client
            .post(url)
            .header("X-Postmark-Server-Token", self.auth_token.expose_secret())
            .json(&body)
            .send()
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::EmailApiRequest;
    use crate::domain::ListSubscriberEmail;
    use crate::mail::{EmailClient, EmailMessage};
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::en::{Paragraph, Sentence};
    use fake::Fake;
    use secrecy::Secret;
    use wiremock::matchers::{body_json_schema, header, header_exists};
    use wiremock::{Mock, MockServer, ResponseTemplate};
    #[tokio::test]
    async fn send_mail_delivers_correct_request() {
        // Arrange
        let mock_server = MockServer::start().await;
        let sender = ListSubscriberEmail::try_from(SafeEmail().fake::<String>()).unwrap();
        let token = Secret::new("token".into());
        let email_client = EmailClient::new(sender, mock_server.uri(), token);

        let message_body: String = Paragraph(1..4).fake();
        let message = EmailMessage {
            recipient: ListSubscriberEmail::try_from(SafeEmail().fake::<String>()).unwrap(),
            subject: Sentence(1..3).fake(),
            body_text: message_body.clone(),
            body_html: message_body,
        };

        Mock::given(body_json_schema::<EmailApiRequest>)
            .and(header_exists("X-Postmark-Server-Token"))
            .and(header("Content-Type", "application/json"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Act
        let _send_result = email_client.send_mail(message).await;
    }
}
