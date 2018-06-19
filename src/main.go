package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
    "net/http"
    "github.com/globalsign/mgo/bson"

	"github.com/globalsign/mgo"
	"github.com/gorilla/mux"
)

type person struct {
    Name  string `json:"name"`
    Age  uint8 `json:"age"`
    Email string `json:"email"`
    Balance float64 `json:"balance"`
}

type personHandler struct {
	db *mgo.Database
}

func newHandler(db *mgo.Database) *personHandler {
	return &personHandler{
		db: db,
	}
}

func (handler *personHandler) get(responseWriter http.ResponseWriter, request *http.Request) {
    vars := mux.Vars(request)
    id := vars["person_id"]

    // TODO Type check/conversion for id

    person := new(person)
    handler.db.C("people").Find(bson.M{"_id":id}).One(&person)

    if person != nil {
        log.Printf("Found person!")
    } else {
        log.Printf("Failed to find person!")
    }

	responseWriter.WriteHeader(http.StatusOK)
	fmt.Fprintf(responseWriter, "Recieved id %v!", id)
}

func (handler *personHandler) post(responseWriter http.ResponseWriter, request *http.Request) {
	body, err := ioutil.ReadAll(request.Body)
	if err != nil {
		log.Printf("Error reading request body: %v", err)
		http.Error(responseWriter, "Unable to read request body!", http.StatusBadRequest)
		return
	}

	person := new(person)
	if err := json.Unmarshal(body, &person); err != nil {
		log.Printf("Error parsing request body: %v", err)
		http.Error(responseWriter, "Unable to parse request body!", http.StatusBadRequest)
		return
    }

    handler.db.C("people").Insert(person)

	responseWriter.WriteHeader(http.StatusOK)
	fmt.Fprintf(responseWriter, "Hi %v", person.Name)
}

func (handler *personHandler) delete(responseWriter http.ResponseWriter, request *http.Request) {
	responseWriter.WriteHeader(http.StatusOK)
	fmt.Fprintf(responseWriter, "Deleted!")
}

func defaultHandler(responseWriter http.ResponseWriter, request *http.Request) {
	log.Printf("Sending default response...")
	responseWriter.WriteHeader(http.StatusOK)
	fmt.Fprintf(responseWriter, "Ready!")
}

func notFoundHandler(responseWriter http.ResponseWriter, request *http.Request) {
	log.Printf("Not found! %v", request.URL)
	responseWriter.WriteHeader(http.StatusNotFound)
	fmt.Fprintf(responseWriter, "Not Found!")
}

func main() {
	port := "8081"
	dbHost := "localhost"
	dbPort := "27017"
	dbName := "Flexera"

	log.Printf("Connecting to db...")
	session, err := mgo.Dial(fmt.Sprintf("%v:%v", dbHost, dbPort))
	if err != nil {
		log.Fatalf("Unable to connect to db! %v", err)
	}
	db := session.DB(dbName)

    personHandler := newHandler(db)
    // defer personHandler.db.Close()

	router := mux.NewRouter().StrictSlash(true)
	router.NotFoundHandler = http.HandlerFunc(notFoundHandler)

	router.HandleFunc("/", defaultHandler).Methods("GET")
	router.HandleFunc("/{person_id}", personHandler.get).Methods("GET")
	router.HandleFunc("/{person_id}", personHandler.post).Methods("POST")

	address := fmt.Sprintf(":%v", port)
	log.Printf("Listening... %v", address)
	http.ListenAndServe(address, router)
}
