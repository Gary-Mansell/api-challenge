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
This requests returns a pseudo-randomly generated id for the user - note this is not _guaranteed_ to be unique but should be close enough :)

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
_where <user_id> is the id returned by posting a user e.g. `curl http://localhost:8081/5577006791947779410`_


Delete specific user:
```
curl -X DELETE \
  http://localhost:8081/people/<user_id>
```

Delete all users:
```
curl -X DELETE \
  http://localhost:8081/people
```
