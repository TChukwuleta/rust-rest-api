use crate::api_error::ApiError;
use chrono::NaiveDateTime;
use reqwest::Client;
use serde::{Serialize, Deserialize};
use dotenv::dotenv;
use mongodb::bson::oid::ObjectId;
use std::env;

// Contact of recipient 
#[derive(Serialize, Debug)]
pub struct Contact {
    pub email: String,
    pub name: Option<String>
}

impl Contact {
    pub fn new<T: Into<String>>(email: T, name: T) -> Self {
        Contact { email: email.into(), name: Some(name.into()) }
    }
}


#[derive(Debug, Serialize)]
pub struct Email {
    pub sender: Contact,
    #[serde(rename = "to")]
    pub recipients: Vec<Contact>,
    pub subject: String,
    #[serde(rename = "htmlContent")]
    pub html: Option<String>
}

impl Email {
    pub fn new(sender: Contact) -> Self{
        Email { sender, recipients: Vec::new(), subject: "".to_string(), html: None }
    }

    pub fn add_recipient<T: Into<Contact>>(mut self, recipient: T) -> Self {
        self.recipients.push(recipient.into());
        self
    }

    pub fn set_subject<T: Into<String>>(mut self, subject: T) -> Self {
        self.subject = subject.into();
        self
    }

    pub fn set_html<T: Into<String>>(mut self, html: T) -> Self {
        self.html = Some(html.into());
        self
    }

    pub async fn send(self) -> Result<String, ApiError> {
        dotenv().ok();
        let email_api_key = env::var("EMAIL_API_KEY").expect("Unable to retrieve API key for email platform");
        let email_platform = env::var("EMAIL_PLATFORM").expect("Unable to get email sender platform");
        let client = Client::new();
        let response = client.post(email_platform)
        .header("Accept", "application/json")
        .header("api-key", email_api_key)
        .json(&self)
        .send()
        .await
        .map_err(|e| ApiError::new(500, format!("Failed to send email: {}", e)));

        let status = response.unwrap().status().as_u16();

        match status {
            201 => Ok(format!("Message delivered successfully")),
            _ => {
                Err(ApiError::new(500, format!("Failed to send email")))
            }
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct EmailVerificationTokenMessage {
    pub id: Option<String>,
    pub email: String
}

#[derive(Serialize, Deserialize)]
pub struct EmailVerificationToken {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub email: String,
    pub expires_at: NaiveDateTime,
    pub created_at: NaiveDateTime
}

