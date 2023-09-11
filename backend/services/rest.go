package services

import (
	_ "github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/cors"
	"github.com/gofiber/fiber/v2/middleware/recover"
	"github.com/gofiber/websocket/v2"
	"log"
)

// TODO: Add port to config
const Port = "8080"

func (s *BackendService) Init() {
	log.Println("Starting API...")
	s.Server.Use(recover.New())
	s.Server.Use(cors.New(cors.Config{
		AllowOrigins: "*",
		AllowMethods: "*",
		AllowHeaders: "*",
	}))
	s.Server.Get("/ws", websocket.New(s.Socket.Start, websocket.Config{
		Subprotocols: []string{"token", "identifier"},
		Origins:      []string{"*"},
	}))

	log.Println("Starting Rest Service on port", Port)
	log.Println(s.Server.Listen(":" + Port))

}
