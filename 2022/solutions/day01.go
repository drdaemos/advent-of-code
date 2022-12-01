package main

import (
	"fmt"
	"github.com/drdaemos/advent-of-code/utils"
	"sort"
)

/*
https://adventofcode.com/2022/day/1
*/
func main() {
	input := utils.GetStrings(utils.GetInputPath("day01.txt"))

	fmt.Println("Total calories:", partOne(input))

	top3 := partTwo(input)
	fmt.Println("Top 3 elves:", top3)
	fmt.Println("Top 3 Sum:", utils.Sum(top3))
}

func partOne(input []string) int {
	elfTotals := getTotals(input)
	_, max := utils.MinMax(elfTotals)

	return max
}

func partTwo(input []string) []int {
	elfTotals := getTotals(input)
	sort.Ints(elfTotals)

	return elfTotals[len(elfTotals)-3:]
}

func getTotals(input []string) []int {
	var elfTotals []int

	current := 0

	for _, amount := range input {
		if amount != "" {
			current += utils.ToInt(amount)
		} else {
			elfTotals = append(elfTotals, current)
			current = 0
		}
	}

	// last elf
	elfTotals = append(elfTotals, current)

	return elfTotals
}
