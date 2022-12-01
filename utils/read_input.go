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

func GetPackageInput(pkg string) string {
	path, err := os.Getwd()
	check(err)
	file := filepath.Join(path, pkg, "input.txt")
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
		result = append(result, ToInt(scan.Text()))
	}

	check(scan.Err())

	return result
}

func ToInt(value string) int {
	number, err := strconv.Atoi(value)
	check(err)
	return number
}
