package day01

import (
	"fmt"
	"github.com/drdaemos/advent-of-code/utils"
	"sort"
)

// Main https://adventofcode.com/2022/day/1
func Main() {
	input := utils.GetStrings(utils.GetPackageInput("day01"))

	fmt.Println("Max amount:", PartOne(input))

	sum, top3 := PartTwo(input)
	fmt.Println("Top 3 sum:", sum)
	fmt.Println("Top 3 amounts:", top3)
}

func PartOne(input []string) int {
	elfTotals := getTotals(input)
	_, max := utils.MinMax(elfTotals)

	return max
}

func PartTwo(input []string) (int, []int) {
	elfTotals := getTotals(input)
	sort.Ints(elfTotals)

	top3 := elfTotals[len(elfTotals)-3:]

	return utils.Sum(top3), top3
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
