package main

import (
	"flag"
	"fmt"

	"github.com/drdaemos/advent-of-code/2023/day01"
	"github.com/drdaemos/advent-of-code/2023/day02"
)

func main() {
	registry := map[string]func(){
		"day01": day01.Main,
		"day02": day02.Main,
	}
	flag.Parse()

	day := flag.Arg(0)

	if _, ok := registry[day]; ok {
		registry[day]()
	} else {
		fmt.Println("Solution not found")
	}
}
