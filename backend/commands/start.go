package commands

import (
	"github.com/InvalidJokerDE/fpm/utils"
	"net"
)

func Start(conn net.Conn, args []string) {
	conn.Write([]byte(utils.Processes[args[0]].Mainfile))
}
