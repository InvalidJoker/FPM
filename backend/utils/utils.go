package utils

import (
	"os"
	"strconv"
)

// Exists checks if a file exists
func Exists(path string) (bool, error) {
	_, err := os.Stat(path)
	if err == nil {
		return true, nil
	}
	if os.IsNotExist(err) {
		return false, nil
	}
	return false, err
}

func IsOnlyInt(s string) bool {
	for _, v := range s {
		if v < '0' || v > '9' {
			return false
		}
	}

	return true
}

func GetProcessByID(id int) *Process {
	for _, v := range Processes {
		if v.ID == id {
			return &v
		}
	}

	return nil
}

func GetProcessByName(name string) *Process {
	for _, v := range Processes {
		if v.Name == name {
			return &v
		}
	}

	return nil
}

func DoesNextIndexExists(args []string, index int) bool {
	if len(args) > index {
		return true
	}

	return false
}

func IsLastIndex(args []string, index int) bool {
	if len(args) == index {
		return true
	}

	return false
}

func StringBoolToBool(s string) bool {
	if s == "true" {
		return true
	}

	return false
}

func StringIntToInt(s string) int {
	if IsOnlyInt(s) {
		i, _ := strconv.Atoi(s)

		return i
	}

	return 0
}

func ParseArgs(args []string) (Args, string) {
	var a Args
	skipNext := false

	for i, v := range args {
		if skipNext {
			skipNext = false
			continue
		}

		switch {
		case v == "-n":
			if DoesNextIndexExists(args, i+1) {
				a.Name = args[i+1]
				skipNext = true
			} else {
				return a, "NO NAME"
			}

			if IsLastIndex(args, i+1) {
				break
			}
		case v == "-s":
			if DoesNextIndexExists(args, i+1) {
				a.Autostart = StringBoolToBool(args[i+1])
				skipNext = true
			} else {
				return a, "NO AUTOSTART"
			}

			if IsLastIndex(args, i+1) {
				break
			}
		case v == "-m":
			if DoesNextIndexExists(args, i+1) {
				a.Maxmem = StringIntToInt(args[i+1])
				skipNext = true
			} else {
				return a, "NO MAXMEMORY"
			}

			if IsLastIndex(args, i+1) {
				break
			}
		case v == "-p":
			if DoesNextIndexExists(args, i+1) {
				a.Prerun = args[i+1]
				skipNext = true
			} else {
				return a, "NO PRERUN"
			}

			if IsLastIndex(args, i+1) {
				break
			}
		case v == "-c":
			if DoesNextIndexExists(args, i+1) {
				a.Command = args[i+1]
				skipNext = true
			} else {
				return a, "NO COMMAND"
			}

			if IsLastIndex(args, i+1) {
				break
			}
		case v == "-a":
			if DoesNextIndexExists(args, i+1) {
				a.After = args[i+1]
				skipNext = true
			} else {
				return a, "NO AFTER"
			}

			if IsLastIndex(args, i+1) {
				break
			}
		default:
			return a, "UNKNOWN ARGUMENT"
		}

	}

	return a, ""
}
