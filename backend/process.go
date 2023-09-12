package main

import (
	"encoding/json"
	"os"
)

type jsonProcess struct {
	Processes []Process `json:"processes"`
}

type Process struct {
	ID        int      `json:"id"`
	Name      string   `json:"name"`
	Args      []string `json:"args"`
	Runtime   string   `json:"runtime"`
	Mainfile  string   `json:"mainfile"`
	Rootdir   string   `json:"rootdir"`
	IsRunning bool     `json:"isRunning"`
}

var Processes map[int]Process

func loadProcesses() error {
	body, err := os.ReadFile("~/.fpm/processes.json")

	if err != nil {
		return err
	}

	var _json jsonProcess
	err2 := json.Unmarshal(body, &_json)

	if err2 != nil {
		return err
	}

	finalList := make(map[int]Process)

	for _, process := range _json.Processes {
		finalList[process.ID] = process
	}

	Processes = finalList

	return nil
}
