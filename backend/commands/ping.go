package commands

import "net"

func Ping(conn net.Conn) {
	conn.Write([]byte("PONG"))
}
