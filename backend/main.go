package main

import "log"

func main() {
	log.Println("Starting server")

	err := StartServer()

	if err != nil {
		log.Fatal(err)
	} else {
		log.Println("Server started")
	}
}
