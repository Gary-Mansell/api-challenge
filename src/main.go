package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"

	"github.com/globalsign/mgo/bson"

	"math/rand"

	"github.com/globalsign/mgo"
	"github.com/gorilla/mux"
)

var peopleColl = "people"

type person struct {
	ID      uint64  `json:"id" bson:"id"`
	Name    string  `json:"name" bson:"fullname"`
	Age     uint8   `json:"age"`
	Email   string  `json:"email"`
	Address string  `json:"address"`
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

	// Assign random id after marshalling (ignore client id)
	person.ID = rand.Uint64() // TODO ensure unique

	handler.db.C(peopleColl).Insert(person)
	log.Printf("Created: %v", person)

	responseWriter.WriteHeader(http.StatusOK)
	fmt.Fprintf(responseWriter, "%v", person.ID)
}

func (handler *personHandler) get(responseWriter http.ResponseWriter, request *http.Request) {
	vars := mux.Vars(request)
	id := vars["person_id"]

	// TODO Type check/conversion for id

	person := new(person)
	handler.db.C(peopleColl).Find(bson.M{"id": id}).One(&person)

	if person != nil {
		log.Printf("Found person!")
	} else {
		log.Printf("Failed to find person!")
	}
	log.Printf("Found: %v", person.ID)

	responseWriter.WriteHeader(http.StatusOK)
	fmt.Fprintf(responseWriter, "Found %v!", person.ID)
}

func (handler *personHandler) delete(responseWriter http.ResponseWriter, request *http.Request) {
	vars := mux.Vars(request)
	id := vars["person_id"]

	count := handler.db.C(peopleColl).Remove(bson.M{"id": id})

	responseWriter.WriteHeader(http.StatusOK)
	fmt.Fprintf(responseWriter, "Deleted %v records!", count)
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
	dbName := "App_DB"

	log.Printf("Connecting to db...")
	session, err := mgo.Dial(fmt.Sprintf("%v:%v", dbHost, dbPort))
	if err != nil {
		log.Fatalf("Unable to connect to db! %v", err)
	}
	db := session.DB(dbName)

	// TODO remove this test!
	if err := db.C("test").Insert(bson.M{"a": 1}); err != nil {
		log.Fatalf("Test failed! %v", err)
	}

	personHandler := newHandler(db)

	router := mux.NewRouter().StrictSlash(true)
	router.NotFoundHandler = http.HandlerFunc(notFoundHandler)

	router.HandleFunc("/", defaultHandler).Methods("GET")
	router.HandleFunc("/", personHandler.post).Methods("POST")
	router.HandleFunc("/{person_id}", personHandler.get).Methods("GET")
	router.HandleFunc("/{person_id}", personHandler.delete).Methods("DELETE")

	address := fmt.Sprintf(":%v", port)
	log.Printf("Listening... %v", address)
	http.ListenAndServe(address, router)
}
