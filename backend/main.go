package main

import (
	"log"

	"github.com/InvalidJokerDE/fpm/utils"
)

func main() {

	if err := utils.LoadProcesses(); err != nil {
		panic(err)
	}

	log.Println("Starting server")

	err2 := StartServer()

	if err2 != nil {
		log.Fatal(err2)
	} else {
		log.Println("Server closed peacefully")
	}
}
