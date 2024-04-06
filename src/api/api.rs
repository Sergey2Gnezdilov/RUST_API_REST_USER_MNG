
use actix_web::{web, get, post, delete, put, HttpResponse};
use crate::{models::user::User, repository::database::Database};



/*#[get("/test")]
async fn test() -> impl Responder {
    let response = Response {
        message: "for test endpoint".to_string(),
    };
    HttpResponse::Ok().json(response)
}
*/

#[get("/users/{id}")]
pub async fn get_user_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let user = db.get_user_by_id(&id);

    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("user not found"),
    }
}

#[get("/users")]
pub async fn get_users(db: web::Data<Database>) -> HttpResponse {
    let users = db.get_users();
    HttpResponse::Ok().json(users)
}

#[post("/users")]
pub async fn create_user(db: web::Data<Database>, new_user: web::Json<User>) -> HttpResponse {
    let user = db.create_user(new_user.into_inner());
    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/users/{id}")]
pub async fn update_user_by_id(db: web::Data<Database>, id: web::Path<String>, updated_user: web::Json<User>) -> HttpResponse {
    let user = db.update_user_by_id(&id, updated_user.into_inner());
    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("user not found"),
    }
}


#[put("/users/active_status/{id}")]
pub async fn active_user_by_id(db: web::Data<Database>, id: web::Path<String>, updated_user: web::Json<User>) -> HttpResponse {
    let user = db.active_user_by_id(&id, updated_user.into_inner());
    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("user not found"),
    }
}

#[put("/users/deactive_status/{id}")]
pub async fn deactive_user_by_id(db: web::Data<Database>, id: web::Path<String>, updated_user: web::Json<User>) -> HttpResponse {
    let user = db.deactive_user_by_id(&id, updated_user.into_inner());
    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("user not found"),
    }
}

#[delete("/users/{id}")]
pub async fn delete_user_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let user = db.delete_user_by_id(&id);
    match user {
        Some(_) => HttpResponse::Ok().finish(),
        None => HttpResponse::NotFound().body("user not found"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api_usr_mng")
            .service(create_user)
            //.service(test)
            .service(get_user_by_id)
            .service(get_users)
            .service(delete_user_by_id)
            .service(update_user_by_id)
            .service(active_user_by_id)
            .service(deactive_user_by_id)

    );
}
