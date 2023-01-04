use chrono::Utc;
use mongodb::{sync::{Collection, Client}, bson::{extjson::de::Error, oid::ObjectId, doc}, results::{DeleteResult, InsertOneResult, UpdateResult}};
use super::{model::User, email_verification::{EmailVerificationToken, EmailVerificationTokenMessage}};
use dotenv::dotenv;
use std::env;


pub struct MongoConnection {
    col: Collection<User>,
    emailcol: Collection<EmailVerificationToken>
}

impl MongoConnection {
    pub fn new() -> Self {
        dotenv().ok();
        let url = match env::var("MongoUrl") {
            Ok(c) => c.to_string(),
            Err(_) => format!("Error loading environment variable")
        };
        let client = Client::with_uri_str(url).unwrap();
        let db = client.database("restapi");
        let col: Collection<User> = db.collection("User");
        let emailcol: Collection<EmailVerificationToken> = db.collection("EmailToken");
        MongoConnection { col, emailcol }
    }

    pub fn getall(&self) -> Result<Vec<User>, Error> {
        let cursors = self.col.find(None, None).expect("Error getting users list");
        let users = cursors.map(|doc| doc.unwrap()).collect();
        Ok(users)
    }

    pub fn getone(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user = self.col.find_one(filter, None).expect("Error retrieving user's detail");
        Ok(user.unwrap())
    }

    pub fn findbyemail(&self, email: &String) -> Result<User, Error> {
        let filter = doc! {"email": email};
        let user = self.col.find_one(filter, None).expect("Error retrieving user by email");
        Ok(user.unwrap())
    }

    pub fn createuser(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let user = User{
            id: None,
            email: new_user.email,
            salt: new_user.salt,
            password: new_user.password,
            created_at: new_user.created_at,
            updated_at: new_user.updated_at
        };
        let newuser = self.col.insert_one(user, None).expect("Error occured while creating a user");
        Ok(newuser)
    }

    pub fn updateuser(&self, id: &String, user_detail: User) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter =doc! {"_id": id};
        let new_doc = doc! {
            "$set": {
                "id": obj_id,
                "email": user_detail.email,
                "password": user_detail.password,
                //"created_at": user_detail.created_at,
                //"updated_at": Utc::now().naive_utc()
            }
        };
        let updated_user = self.col.update_one(filter, new_doc, None).expect("error updating user information");
        Ok(updated_user)
    }

    pub fn deleteuser(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user = self.col.delete_one(filter, None).expect("Error occured while deleting user");
        Ok(user)
    }

    pub fn createemailtoken(&self, emailtokenreq: EmailVerificationTokenMessage) -> Result<InsertOneResult, Error> {
        let new_guy = EmailVerificationToken{
            id: None,
            email: emailtokenreq.email,
            expires_at: Utc::now().naive_utc(),
            created_at: Utc::now().naive_utc()
        };
        let newemailtoken = self.emailcol.insert_one(new_guy, None).expect("Error occured wile creating email token");
        Ok(newemailtoken)
    }

    pub fn deleteemailtoken(&self, id: &String) -> Result<DeleteResult, Error> {
        let filter = doc! {"id": id};
        let token = self.emailcol.delete_one(filter, None).expect("Error deleting email token");
        Ok(token)
    }
}