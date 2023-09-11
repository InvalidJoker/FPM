package main

import (
	"github.com/InvalidJokerDE/fpm/services"
	"github.com/InvalidJokerDE/fpm/utils"
	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/websocket/v2"
	"log"
)

func main() {
	log.Println("Starting FPM backend...")

	socketService := utils.Socket{
		Clients: map[string]*websocket.Conn{},
	}

	backendService := services.BackendService{
		Server: fiber.New(),
		Socket: &socketService,
	}

	backendService.Init()
}
