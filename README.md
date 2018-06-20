# Api Challenge

## Current state
Given more time I updated this repo with a completed challenge. The original text is at the bottom of this readme.

### Run db
Run mongodb on your host:
`docker run --rm -p 27017:27017 mongo:latest`

## Run app
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









*_Original readme text below_*

---

## Submission (after 1 hour)
_`git checkout v1.0_go` or `git checkout v1.0_rust`_

So this went well :/

I initially attempted to create the api using Rust, sadly there happens to be a bug with the Rocket crate I was using which I was unable to resolve - it is essentially identical to [this issue](https://github.com/SergioBenitez/Rocket/issues/235).

**As a result there is no database interaction** and the api is incomplete/very basic.

After discovering the error I attempted to update my rust toolchain (I use nightly) which took about 15 minutes. It still did not work and after attempting the stable toochain it failed too. At this point I decided to cut my loses and write the api in golang.

To get (this does not interact with any db and simply returns ok):
```
curl http://localhost:8081
```

To post some basic data which should be marshalled using:
```
curl -X POST \
  http://localhost:8081 \
  -H 'Cache-Control: no-cache' \
  -H 'Content-Type: application/json' \
  -d '{
	"name": "test_name",
	"email": "test_email"
}'
```

For what it's worth: I am confident that with more time I would be able to complete this challenge fairly easily - I have created similar apis numerous times over the past few years.