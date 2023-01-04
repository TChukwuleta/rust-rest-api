use crate::{user::model::User, api_error::ApiError};
use actix_web::{get, post, put, delete, web, HttpResponse};
use chrono::Utc;
use super::dbconn::MongoConnection;
use  crate::user::model::UserMessage;

#[get("/users")]
async fn find_all(db: web::Data<MongoConnection>) -> Result<HttpResponse, ApiError>{
    let users = db.getall();
    match users {
       Ok(people) => Ok(HttpResponse::Ok().json(people)), 
       Err(e) => Err(ApiError::new(1, format!("Error retrieving users: {}", e))) 
    }
}

#[get("/users/{id}")]
async fn find(db: web::Data<MongoConnection>, id: web::Path<String>) -> Result<HttpResponse, ApiError> {
    if id.is_empty() {
        return Err(ApiError::new(0, "UserId cannot be null or empty".to_string()));
    }
    let user = db.getone(&id);
    match user {
        Ok(v) => Ok(HttpResponse::Ok().json(v)),
        Err(e) => Err(ApiError::new(1, format!("An error occured while trying to retrieve user details: {}", e)))
    }
}

#[post("/register")]
async fn create(db: web::Data<MongoConnection>, user: web::Json<UserMessage>) -> Result<HttpResponse, ApiError> {
    let mut user_detail = User {
        id: None,
        email: user.email.to_owned(),
        salt: String::new(),
        password: user.password.to_owned(),
        created_at: Utc::now().naive_utc(),
        updated_at: Some(Utc::now().naive_utc())
    };
    let (hashed_password, password_salt) = user_detail.hash_password(&user.password, None);
    user_detail.password = hashed_password;
    user_detail.salt = password_salt;
    let new_user = db.createuser(user_detail);
    match new_user {
        Ok(v) => Ok(HttpResponse::Ok().json(v)),
        Err(e) => Err(ApiError::new(2, format!("User creation was not successful: {}",e)))
    }
}

#[put("/users/{id}")]
async fn update(db: web::Data<MongoConnection>, id: web::Path<String>, user: web::Json<UserMessage>) -> Result<HttpResponse, ApiError> {
    if id.is_empty() {
        return Err(ApiError::new(0, "UserId cannot be null or empty".to_string()));
    }

    let existing_user = db.getone(&id).unwrap();
    let data = User{
        id: existing_user.id,
        email: user.email.to_owned(),
        salt: existing_user.salt,
        password: user.password.to_owned(),
        created_at: Utc::now().naive_utc(),
        updated_at: Some(Utc::now().naive_utc())
    };
    let updated_user = db.updateuser(&id, data);
    match updated_user {
        Ok(updated) => {
            if updated.matched_count == 1 {
                let updated_user_detail = db.getone(&id);
                return match updated_user_detail {
                    Ok(user) => Ok(HttpResponse::Ok().json(user)),
                    Err(e) => Err(ApiError::new(1, format!("Error retrieving user: {}", e)))
                };
            }
            else {
                return Err(ApiError::new(1, format!("User not found.")));
            }
        },
        Err(e) => Err(ApiError::new(1, format!("Error updating user: {}", e)))
    }
}

#[delete("/users/{id}")]
async fn delete(db: web::Data<MongoConnection>, id: web::Path<String>) -> Result<HttpResponse, ApiError> {
    if id.is_empty() {
        return Err(ApiError::new(0, "UserId cannot be null or empty".to_string()));
    }
    let existing_user = db.deleteuser(&id);
    match existing_user {
        Ok(res) => {
            if res.deleted_count == 1 {
                return Ok(HttpResponse::Ok().body(format!{"User deleted successfully"}));
            }
            else {
                return Err(ApiError::new(2, format!{"User with the specified Id wasnot found"}));
            }
        },
        Err(e) => Err(ApiError::new(1, format!("Error deleting user: {}", e)))
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig){
    let db = MongoConnection::new();
    let db_data = web::Data::new(db);
    cfg.app_data(db_data);
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}