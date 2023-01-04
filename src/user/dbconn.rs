use mongodb::{sync::{Collection, Client}, bson::{extjson::de::Error, oid::ObjectId, doc}, results::{DeleteResult, InsertOneResult, UpdateResult}};
use super::model::User;
use dotenv::dotenv;
use std::env;


pub struct MongoConnection {
    col: Collection<User>
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
        MongoConnection { col }
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
}