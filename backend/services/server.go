package services

import (
	"fmt"
	"net"
	"os"
	"os/signal"
	"strings"
	"syscall"

	"github.com/InvalidJokerDE/fpm/commands"
)

func callback(conn net.Conn) error {
	defer conn.Close()

	buffer := make([]byte, 1024)

	n, err := conn.Read(buffer)
	if err != nil {
		return err
	}

	data := string(buffer[:n])
	command := strings.Split(data, " ")

	if len(command) < 1 {
		_, err2 := conn.Write([]byte("NO COMMAND"))
		if err2 != nil {
			return err2
		}

		return nil
	}

	switch strings.ToUpper(strings.TrimSpace(command[0])) {
	case "PING":
		commands.Ping(conn)
	case "START":
		commands.Start(conn, command)
	default:
		_, err2 := conn.Write([]byte("UNKNOWN COMMAND"))
		if err2 != nil {
			return err2
		}
	}

	return nil
}

func StartServer() error {
	path := fmt.Sprintf("/run/user/%d/fpm.sock", os.Getuid())
	server, err := net.Listen("unix", path)
	if err != nil {
		return err
	}

	c := make(chan os.Signal, 1)
	signal.Notify(c, os.Interrupt, syscall.SIGTERM)
	go func() {
		<-c
		err := os.Remove(path)
		if err != nil {
			fmt.Println(err)
		}
		os.Exit(0)
	}()

	for {
		conn, err := server.Accept()

		if err != nil {
			return err
		}

		go func() {
			err := callback(conn)
			if err != nil {
				fmt.Println(err)
			}
		}()
	}
}
