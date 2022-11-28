package main

import (
	"fmt"
	"github.com/drdaemos/advent-of-code/utils"
)

/*
https://adventofcode.com/2021/day/1
*/
func main() {
	input := utils.GetIntegers(utils.GetInputPath("day01.txt"))

	var previous int
	var current int
	var wentDeeper int

	for i, depth := range input {
		if i == 0 {
			previous = depth
			current = depth

			continue
		}

		current = depth

		if current > previous {
			wentDeeper += 1
		}

		previous = current
	}

	fmt.Println(wentDeeper)
}
