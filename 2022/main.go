package main

import (
	"flag"
	"fmt"

	"github.com/drdaemos/advent-of-code/2022/day01"
	"github.com/drdaemos/advent-of-code/2022/day02"
	"github.com/drdaemos/advent-of-code/utils"
)

func main() {
	registry := map[string]func(){
		"day01": day01.Main,
		"day02": day02.Main,
		"day03": func() {
			out, _, _ := utils.ExecuteShell("php ./day03/solution.php")
			fmt.Print(out)
		},
		"day04": func() {
			out, _, _ := utils.ExecuteShell("(cd day04/ && clojure -M ./solution.clj)")
			fmt.Print(out)
		},
		"day05": func() {
			out, _, _ := utils.ExecuteShell("(cd day05/ && clojure -M ./solution.clj)")
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
