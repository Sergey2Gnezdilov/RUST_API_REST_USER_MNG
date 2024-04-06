# RUST_API_REST_USER_MNG
API_REST_USER_MNG(RUST, DIESEL, ACTIX)
PAAS POstgresql Elephant (see .env)

**Works on localhost**
http://127.0.0.1:8082/keepalive
/keepalive  get metod for test server run


**Find postman worksheet here -> ** 

https://www.postman.com/cryosat-geoscientist-76623780/workspace/api-users-management-rust-actix-disel/request/25644544-e2aa70ae-1928-4faf-b2ef-fb119034dbfd


**Example for POST new user**

curl -X POST -H "Content-Type: application/json" -d '{"first_name": "John",  "second_name": "Connor","email": "sr_engineer@sky.net","phone" : "12345"}' http://localhost:8082/api_usr_mng/users


**Example GET List users**
curl -s http://localhost:8082/api_usr_mng/users

**Example GET specified user by ID **
curl -s http://localhost:8082/api_usr_mng/users/{id}


**Example DELETE specified user by ID **
curl -s -X DELETE http://localhost:8082/api_usr_mng/users/{id}

**Example PUT specified user by ID **
curl -s -X PUT -H "Content-Type: application/json" -d '{"first_name": "John",  "second_name": "Connor","email": "chief_engineer@sky.net","phone" : "12345"}' http://localhost:8082/api_usr_mng/users/{id}

**Example activate and deActicate user**
http://localhost:8082/api_usr_mng/users/active_status/42feedbd-547f-478a-905c-799d2ce3a88f  
http://localhost:8082/api_usr_mng/users/deactive_status/42feedbd-547f-478a-905c-799d2ce3a88f  

**-=some cli comands =-**
$clear && cargo build 
$cargo run 
For back trace set 
$export RUST_BACKTRACE=full



