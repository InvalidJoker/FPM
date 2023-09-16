package commands

import "net"

func Start(conn net.Conn, args []string) {
	// return args
	conn.Write([]byte("START"))
	conn.Write([]byte(args[0]))
}
