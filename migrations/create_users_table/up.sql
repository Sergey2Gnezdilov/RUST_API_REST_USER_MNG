CREATE TABLE users (
  id VARCHAR(255) PRIMARY KEY,
  user_status VARCHAR(25),
  date_create TIMESTAMP,
  date_last_update TIMESTAMP,
  first_name VARCHAR(255),
  second_name VARCHAR(255),
  email VARCHAR(255),
  phone VARCHAR(11)
);