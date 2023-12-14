package main

import (
	"flag"
	"fmt"

	"github.com/drdaemos/advent-of-code/2023/day01"
	"github.com/drdaemos/advent-of-code/2023/day02"
	"github.com/drdaemos/advent-of-code/utils"
)

func main() {
	registry := map[string]func(){
		"day01": day01.Main,
		"day02": day02.Main,
		"day03": func() {
			runRustBin("day03")
		},
		"day04": func() {
			runRustBin("day04")
		},
		"day05": func() {
			runRustBin("day05")
		},
		"day06": func() {
			runRustBin("day06")
		},
		"day07": func() {
			runRustBin("day07")
		},
		"day08": func() {
			runRustBin("day08")
		},
		"day09": func() {
			runRustBin("day09")
		},
		"day10": func() {
			runRustBin("day10")
		},
		"day11": func() {
			runRustBin("day11")
		},
	}
	flag.Parse()

	day := flag.Arg(0)

	if _, ok := registry[day]; ok {
		registry[day]()
	} else {
		fmt.Println("Solution not found")
	}
}

func runRustBin(day string) {
	out, _, _ := utils.ExecuteShell(fmt.Sprintf("(cd ../ && cargo run --bin 2023 -- %s)", day))
	fmt.Print(out)
}
