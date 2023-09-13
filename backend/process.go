package main

import (
	"encoding/json"
	"fmt"
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

func getProcessFile() (string, error) {
	directory, err := Exists("~/.fpm")

	if err != nil {
		return "", err
	}

	if directory == false && err == nil {
		err2 := os.MkdirAll("~/.fpm", os.ModePerm)

		if err2 != nil {
			return "", err2
		}
	}

	file, err2 := Exists("~/.fpm/processes.json")

	if err2 != nil {
		return "", err2
	}

	fmt.Println(file)

	if file == false && err2 == nil {
		fmt.Println("Creating processes.json")
		f, err3 := os.Create("~/.fpm/processes.json")

		if err3 != nil {
			return "", err3
		}

		defer f.Close()

		_, err4 := f.Write([]byte("{\"processes\":[]}"))

		if err4 != nil {
			return "", err4
		}

		return "{\"processes\":[]}", nil
	} else {
		fmt.Println("Reading processes.json")
		body, err3 := os.ReadFile("~/.fpm/processes.json")

		if err3 != nil {
			return "", err3
		}

		return string(body), nil
	}
}

func LoadProcesses() error {
	body, err := getProcessFile()

	if err != nil {
		return err
	}

	var _json jsonProcess
	err2 := json.Unmarshal([]byte(body), &_json)

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
