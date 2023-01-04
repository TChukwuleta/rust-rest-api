use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use rand::{Rng, thread_rng, distributions::Alphanumeric};
use argon2::Config;


// User sign up model
#[derive(Serialize, Deserialize)]
pub struct UserMessage {
    pub email: String,
    pub password: String
}

// User Object
#[derive(Debug, Deserialize, Serialize)]
pub struct User{
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub email: String,
    pub salt: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>
}

impl User {
   pub fn match_password(&self, password: &String) -> bool {
       argon2::verify_encoded(&self.password, password.as_bytes()).unwrap()
   } 

   pub fn hash_password(&self, password: &String, salt: Option<&String>) -> (String, String) {
       let config = Config::default();
       //let test_salt: [u8; 32] = thread_rng().gen();
       let mut user_salt = String::new();
       if salt.is_none() {
        user_salt = thread_rng().sample_iter(Alphanumeric)
        .take(20).map(char::from).collect();
       }
       if salt.is_some() {
           user_salt = salt.unwrap().to_string();
       }
       let hashed_password = argon2::hash_encoded(password.as_bytes(), user_salt.as_bytes(), &config).unwrap();
       (hashed_password, user_salt)
   }
}