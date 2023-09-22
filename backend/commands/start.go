package commands

import (
	"fmt"
	"github.com/InvalidJokerDE/fpm/utils"
	"net"
)

// TODO: Finish method
func runProcess(name string) {
	utils.Processes[name].IsRunning = true
}

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

	utils.Processes[args.Name] = &utils.Process{
		ID:        id,
		Name:      args.Name,
		IsRunning: false,
		After:     args.After,
		Command:   args.Command,
		Cwd:       args.Cwd,
		Prerun:    args.Prerun,
		AutoStart: args.AutoStart,
		Maxmem:    args.Maxmem,
	}

	go runProcess(args.Name)

	_, err := conn.Write([]byte(fmt.Sprintf("OK\nID:%d", id)))
	if err != nil {
		return err
	}

	return nil
}
