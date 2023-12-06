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
			out, _, _ := utils.ExecuteShell("(cd ../ && cargo run --bin 2023 -- day03)")
			fmt.Print(out)
		},
		"day04": func() {
			out, _, _ := utils.ExecuteShell("(cd ../ && cargo run --bin 2023 -- day04)")
			fmt.Print(out)
		},
		"day05": func() {
			out, _, _ := utils.ExecuteShell("(cd ../ && cargo run --bin 2023 -- day05)")
			fmt.Print(out)
		},
		"day06": func() {
			out, _, _ := utils.ExecuteShell("(cd ../ && cargo run --bin 2023 -- day06)")
			fmt.Print(out)
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
