package main

import (
	"flag"
	"fmt"
	"github.com/drdaemos/advent-of-code/2022/day01"
)

func main() {
	registry := map[string]func(){
		"day01": day01.Main,
	}
	flag.Parse()

	day := flag.Arg(0)

	if _, ok := registry[day]; ok {
		registry[day]()
	} else {
		fmt.Println("Solution not found")
	}
}
