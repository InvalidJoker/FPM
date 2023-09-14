package utils

import (
	"fmt"

	"github.com/BurntSushi/toml"
)

type Config struct {
	Info           InfoSection
	Commands       CommandsSection
	Specifications SpecificationsSection
}

type InfoSection struct {
	Name string
}

type CommandsSection struct {
	Inital  string
	Prerun  string
	Startup string
	After   string
}

type SpecificationsSection struct {
	MaxMemory int
}

func parseConfig(path string) Config {
	var config Config
	if _, err := toml.DecodeFile(path, &config); err != nil {
		fmt.Println("Error parsing TOML:", err)
		return config
	}

	return config
}
