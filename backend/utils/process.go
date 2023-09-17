package utils

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
	Cwd       string   `json:"cwd"`
	IsRunning bool     `json:"isRunning"`
}

var Processes map[string]Process

func getProcessFile() (string, error) {
	rootDir, errr := os.UserConfigDir()

	if errr != nil {
		return "", errr
	}

	directory, err := Exists(fmt.Sprintf("%s/fpm/", rootDir))

	if err != nil {
		return "", err
	}

	if !directory && err == nil {
		err2 := os.MkdirAll(fmt.Sprintf("%s/fpm/", rootDir), os.ModePerm)

		if err2 != nil {
			return "", err2
		}
	}

	file, err2 := Exists(fmt.Sprintf("%s/fpm/processes.json", rootDir))

	if err2 != nil {
		return "", err2
	}

	if !file && err2 == nil {
		f, err3 := os.Create(fmt.Sprintf("%s/fpm/processes.json", rootDir))

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
		body, err3 := os.ReadFile(fmt.Sprintf("%s/fpm/processes.json", rootDir))

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

	finalList := make(map[string]Process)

	for _, process := range _json.Processes {
		finalList[process.Name] = process
	}

	Processes = finalList

	return nil
}
