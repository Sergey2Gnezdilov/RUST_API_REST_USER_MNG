# RUST_API_REST_USER_MNG
API_REST_USER_MNG(RUST, DIESEL, ACTIX)
PAAS POstgresql Elephant (see .env)

Works on localhost http://127.0.0.1:8082/keepalive
/keepalive  get metod for test server run

Find postman worksheet here ->  https://www.postman.com/cryosat-geoscientist-76623780/workspace/api-users-management-rust-actix-disel/request/25644544-e2aa70ae-1928-4faf-b2ef-fb119034dbfd
/n
Example for POST new user
curl -X POST -H "Content-Type: application/json" -d \ /n
'{"first_name": "John",  "second_name": "Connor","email": "chief_engineer@sky.net","phone" : "12345"}' \ /n
http://localhost:8082/api_usr_mng/users

Example GET List users
curl -s http://localhost:8082/api_usr_mng/users | jq 



-=some cli comands =- /n
$clear && cargo build /n
$cargo run /n
For back trace set  /n
$export RUST_BACKTRACE=full



