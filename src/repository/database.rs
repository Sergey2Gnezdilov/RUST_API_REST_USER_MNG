use std::fmt::Error;
use chrono::prelude::*;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

use crate::models::user::User;
use crate::repository::schema::users::dsl::*;

type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Database {
    pool: DBPool,
}

impl Database {

    pub fn new() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: DBPool = r2d2::Pool::builder()
            .max_size(4)
            .build(manager)
            .expect("Failed to create pool.");
        Database { pool }
    }

    pub fn get_users(&self) -> Vec<User> {
        users
            .load::<User>(&mut self.pool.get().unwrap())
            .expect("Error loading all users")
    }

    pub fn create_user(&self, user: User) -> Result<User, Error> {
        let user = User {
            id: uuid::Uuid::new_v4().to_string(),
            date_create: Some(Utc::now().naive_utc()),
            user_status: Some("new".to_string()), //defult value for new memebers
            ..user
        };
        diesel::insert_into(users)
            .values(&user)
            .execute(&mut self.pool.get().unwrap())
            .expect("Error creating new user");
        Ok(user)
    }

    pub fn get_user_by_id(&self, user_id: &str) -> Option<User> {
        let user = users
            .find(user_id)
            .get_result::<User>(&mut self.pool.get().unwrap())
            .expect("Error loading user by id");
        Some(user)
    }

    pub fn delete_user_by_id(&self, user_id: &str) -> Option<usize> {
        let count = diesel::delete(users.find(user_id))
            .execute(&mut self.pool.get().unwrap())
            .expect("Error deleting user by id");
        Some(count)
    }


    pub fn update_user_by_id(&self, user_id: &str, mut user: User) -> Option<User> {
        user.date_last_update = Some(Utc::now().naive_utc());
        let user = diesel::update(users.find(user_id))
            .set(&user)
            .get_result::<User>(&mut self.pool.get().unwrap())
            .expect("Error updating user by id");
        Some(user)
    }

    pub fn active_user_by_id(&self, user_id: &str, mut user: User) -> Option<User> {
        user.user_status = Some("active".to_string());
        let user = diesel::update(users.find(user_id))
            .set(&user)
            .get_result::<User>(&mut self.pool.get().unwrap())
            .ok();
        user
    }    
    pub fn deactive_user_by_id(&self, user_id: &str, mut user: User) -> Option<User> {
        user.user_status = Some("block".to_string());
        let user = diesel::update(users.find(user_id))
            .set(&user)
            .get_result::<User>(&mut self.pool.get().unwrap())
            .ok();
        user
    }   
}