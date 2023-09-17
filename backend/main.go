package main

import (
	"log"

	"github.com/InvalidJokerDE/fpm/utils"
)

func main() {

	if err := utils.LoadProcesses(); err != nil {
		panic(err)
	}

	utils.Processes["Hallo"] = utils.Process{
		ID:        0,
		Name:      "Hallo",
		Mainfile:  "/mnt/data/kotin/go/FPM/backend/main.go",
		Args:      []string{},
		IsRunning: false,
		Cwd:       "/mnt/data/kotin/go/FPM/backend/",
		Runtime:   "",
	}

	log.Println("Starting server")

	err2 := StartServer()

	if err2 != nil {
		log.Fatal(err2)
	} else {
		log.Println("Server closed peacefully")
	}
}
