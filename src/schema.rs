// @generated automatically by Diesel CLI.

diesel::table! {
    users (user_id) {
        #[max_length = 25]
        user_id -> Varchar,
        #[max_length = 25]
        user_status -> Nullable<Varchar>,
        date_creat -> Nullable<Date>,
        #[max_length = 255]
        first_name -> Nullable<Varchar>,
        #[max_length = 255]
        second_name -> Nullable<Varchar>,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        #[max_length = 11]
        phone -> Nullable<Varchar>,
    }
}
