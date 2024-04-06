# RUST_API_REST_USER_MNG
API_REST_USER_MNG(RUST, DIESEL, ACTIX)
PAAS POstgresql Elephant (see .env)

Works on localhost http://127.0.0.1:8082/keepalive
/keepalive  get metod for test server run

Find postman worksheet
https://www.postman.com/cryosat-geoscientist-76623780/workspace/api-users-management-rust-actix-disel/request/25644544-e2aa70ae-1928-4faf-b2ef-fb119034dbfd

POST /user {
	 "first_name": "Test",
      "second_name": "test_user",
      "email": "test_user@sky.net",
      "phone" : "12345"
} 

-=some cli comands =-
$clear && cargo build
$cargo run
For back trace set 
$export RUST_BACKTRACE=full



