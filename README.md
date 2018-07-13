# Api Challenge

### Run db
Run mongodb on your host:
`docker run --rm -p 27017:27017 mongo:latest`

## Run app
### Go
With Go toolchain installed:
`go run src/main.go`
The libraries must be installed via:
`go get -v github.com/gorilla/mux github.com/globalsign/mgo`

_Alternatively build the dockerfile in the repo and run as a container if you don't want to run on host._

### Rust
With Rust toolchain installed:
`cargo run`


## Making api requests
Creation:
```
curl -X POST \
  http://localhost:8081/people/ \
  -H 'Content-Type: application/json' \
  -d '{
	"name": "test_name_1",
	"age": 11,
	"email": "test_email_1",
	"address": "test_address_1",
	"balance": 1.1
}'
```
This requests returns the randomly generated user id for the user which can be used in subsequent requests.

Retrieve all users:
```
curl -X GET \
  http://localhost:8081/people
```

Retrieve specific user:
```
curl -X GET \
  http://localhost:8081/people/<user_id>
```
_where <user_id> is the generated user id e.g. `curl http://localhost:8081/5577006791947779410`_


Delete specific user:
```
curl -X DELETE \
  http://localhost:8081/people/<user_id>
```
_where <user_id> is the generated user id._

Delete all users:
```
curl -X DELETE \
  http://localhost:8081/people
```
