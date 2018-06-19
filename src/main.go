package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"

	"github.com/globalsign/mgo"
	"github.com/gorilla/mux"
)

type person struct {
	Name  string `json:"name"`
	Email string `json:"email"`
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
	responseWriter.WriteHeader(http.StatusOK)
	fmt.Fprintf(responseWriter, "Hello!")
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

	responseWriter.WriteHeader(http.StatusOK)
	fmt.Fprintf(responseWriter, "Hi {}", person.Name)
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

	router := mux.NewRouter().StrictSlash(true)
	router.NotFoundHandler = http.HandlerFunc(notFoundHandler)

	router.HandleFunc("/", defaultHandler).Methods("GET")
	router.HandleFunc("/", personHandler.get).Methods("GET")
	router.HandleFunc("/", personHandler.post).Methods("POST")

	address := fmt.Sprintf(":%v", port)
	log.Printf("Listening... %v", address)
	http.ListenAndServe(address, router)
}
