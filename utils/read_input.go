package utils

import (
	"bufio"
	"os"
	"path/filepath"
	"strconv"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func GetInputPath(filename string) string {
	path, err := os.Getwd()
	check(err)
	file := filepath.Join(path, "inputs", filename)
	return file
}

func GetStrings(path string) []string {
	file, err := os.Open(path)
	check(err)
	defer func(file *os.File) {
		_ = file.Close()
	}(file)

	scan := bufio.NewScanner(file)

	var result []string

	for scan.Scan() {
		result = append(result, scan.Text())
	}

	check(scan.Err())

	return result
}

func GetIntegers(path string) []int {
	file, err := os.Open(path)
	check(err)
	defer func(file *os.File) {
		_ = file.Close()
	}(file)

	scan := bufio.NewScanner(file)

	var result []int

	for scan.Scan() {
		value, err := strconv.Atoi(scan.Text())
		check(err)
		result = append(result, value)
	}

	check(scan.Err())

	return result
}
