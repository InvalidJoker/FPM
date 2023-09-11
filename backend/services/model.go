package services

import (
	"github.com/InvalidJokerDE/fpm/utils"
	"github.com/gofiber/fiber/v2"
)

type BackendService struct {
	Server *fiber.App
	Socket *utils.Socket
}
