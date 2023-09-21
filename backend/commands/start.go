package commands

import (
	"github.com/InvalidJokerDE/fpm/utils"
	"net"
)

func Start(conn net.Conn, args utils.Args) error {
	if args.Name == "" {
		_, err := conn.Write([]byte("NO NAME"))
		if err != nil {
			return err
		}

		return nil
	}

	if utils.GetProcessByName(args.Name) != nil {
		_, err := conn.Write([]byte("NAME ALREADY EXISTS"))
		if err != nil {
			return err
		}

		return nil
	}

	id := utils.GetID()

	utils.Processes[args.Name] = utils.Process{
		ID:        id,
		Name:      args.Name,
		IsRunning: true,
		After:     args.After,
		Command:   args.Command,
		Cwd:       args.Cwd,
		Prerun:    args.Prerun,
		AutoStart: args.AutoStart,
		Maxmem:    args.Maxmem,
	}

	return nil
}
