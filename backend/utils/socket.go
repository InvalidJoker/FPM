package utils

import (
	"encoding/json"
	"github.com/gofiber/websocket/v2"
	"log"
)

// TODO: Add token to config
const securityToken = "ABC123"

type Socket struct {
	Clients map[string]*websocket.Conn
}

func (sc *Socket) SendToAll(object interface{}) {
	for identifier := range sc.Clients {
		if object == nil {
			continue
		}
		log.Println("Sending message to client: " + identifier)
		sc.Send(identifier, object)
	}
}

func (sc *Socket) Send(identifier string, object interface{}) {
	jsonObject, err := json.Marshal(object)
	c := sc.Clients[identifier]
	if err == nil && c != nil {
		err = c.WriteMessage(websocket.TextMessage, jsonObject)
		if err != nil {
			log.Println("Could not send message to client: " + identifier)
		}
	}
}

func (sc *Socket) Start(c *websocket.Conn) {
	token := c.Query("token")
	identifier := c.Query("identifier")

	clientIp := c.RemoteAddr().String()
	clientIp = clientIp[:len(clientIp)-6]

	log.Printf("New Socket connection from %s with token %s", clientIp, token)

	if token != securityToken {
		log.Printf("%s tried to connect to websocket with invalid token: (%s)", clientIp, token)
		err := c.WriteMessage(websocket.TextMessage, []byte("Dashboard "+clientIp+" is not allowed to connect to this websocket"))
		if err != nil {
			log.Println("Could not send message to client: " + clientIp)
		}
		return
	}

	sc.Clients[identifier] = c
	log.Printf("%s has connected to websocket", identifier)

	var (
		mt  int
		msg []byte
		err error
	)
	for {
		if mt, msg, err = c.ReadMessage(); err != nil {
			break
		}

		log.Printf("Incoming Type: %d", mt)
		log.Printf("Message: %s", string(msg))
	}
}
