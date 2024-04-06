diesel::table! {
    users (id) {
        id -> Varchar,
        user_status -> Nullable<Text>,
        date_create -> Nullable<Timestamp>,
        date_last_update -> Nullable<Timestamp>,
        first_name -> Varchar,
        second_name -> Varchar,
        email -> Varchar,
        phone -> Varchar,
    }
}

/*insert into users (
    id,
    user_status,
    date_creat ,
    first_name,
    second_name,
    email,
    phone
  ) */