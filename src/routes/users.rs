use actix_web::{web, HttpResponse, Responder};
use crate::models::user::{User, CreateUserRequest};
use std::sync::Mutex;
use uuid::Uuid;

// In-memory storage for the POC
pub struct AppState {
    pub users: Mutex<Vec<User>>,
}

pub async fn create_user(
    data: web::Data<AppState>,
    user_req: web::Json<CreateUserRequest>,
) -> impl Responder {
    let user = User::new(user_req.name.clone(), user_req.email.clone());
    let mut users = data.users.lock().unwrap();
    users.push(user.clone());
    HttpResponse::Created().json(user)
}

pub async fn get_users(data: web::Data<AppState>) -> impl Responder {
    let users = data.users.lock().unwrap();
    HttpResponse::Ok().json(users.to_vec())
}

pub async fn get_user(
    data: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let users = data.users.lock().unwrap();
    if let Some(user) = users.iter().find(|u| u.id == *id) {
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::NotFound().finish()
    }
}

pub async fn update_user(
    data: web::Data<AppState>,
    id: web::Path<Uuid>,
    user_req: web::Json<CreateUserRequest>,
) -> impl Responder {
    let mut users = data.users.lock().unwrap();
    if let Some(user) = users.iter_mut().find(|u| u.id == *id) {
        user.name = user_req.name.clone();
        user.email = user_req.email.clone();
        user.updated_at = chrono::Utc::now();
        HttpResponse::Ok().json(user) 
    } else {
        HttpResponse::NotFound().finish()
    }
}

pub async fn delete_user(
    data: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let mut users = data.users.lock().unwrap();
    if let Some(pos) = users.iter().position(|u| u.id == *id) {
        users.remove(pos);
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}