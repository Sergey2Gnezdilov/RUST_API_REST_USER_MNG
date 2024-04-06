use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, AsChangeset};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::repository::schema::users)]
// structure for user as object
pub struct User {
    #[serde(default)]
    pub id: String,
    pub user_status: Option<String>,
    pub date_create: Option<chrono::NaiveDateTime>,
    pub date_last_update: Option<chrono::NaiveDateTime>,
    pub first_name: String,
    pub second_name: String,
    pub email: String,
    pub phone: String,
}


/*DB 
  user_id,
  user_status,
  date_creat ,
  first_name,
  second_name,
  email,
  phone 
  
  CREATE TABLE users (
  user_id INT PRIMARY KEY,
  user_status VARCHAR(25),
  date_creat DATE,
  first_name VARCHAR(255),
  second_name VARCHAR(255),
  email VARCHAR(255),
  phone VARCHAR(11)
);

pub struct User_struct {
    #[serde(default)]
    
}
*/