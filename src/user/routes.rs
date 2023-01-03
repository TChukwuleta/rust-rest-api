use crate::user::model::User;
use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use serde_json::json;

#[get("/users")]
async fn find_all() -> impl Responder{
    HttpResponse::Ok().json(vec![
        User{id: 1, email: "testone@yopmail.com".to_string()},
        User{id: 1, email: "testtwo@yopmail.com".to_string()}
    ])
}

#[get("/users/{id}")]
async fn find() -> impl Responder {
    HttpResponse::Ok().json(User {id: 1, email: "testthree@yopmail.com".to_string()})
}

#[post("/users")]
async fn create(user: web::Json<User>) -> impl Responder {
    HttpResponse::Created().json(user)
}

#[put("/users/{id}")]
async fn update(user: web::Json<User>) -> impl Responder {
    HttpResponse::Ok().json(user)
}

#[delete("/users/{id}")]
async fn delete() -> impl Responder {
    HttpResponse::Ok().json(json!({"message": "Deleted"}))
}

pub fn init_routes(cfg: &mut web::ServiceConfig){
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}