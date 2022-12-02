package day02

import (
	"fmt"
	"github.com/drdaemos/advent-of-code/utils"
)

// Main https://adventofcode.com/2022/day/2
func Main() {
	input := utils.GetStrings(utils.GetPackageInput("day02"))

	fmt.Println("Part one:", PartOne(input))
	fmt.Println("Part two:", PartTwo(input))
}

func PartOne(input []string) int {
	var total int
	for _, line := range input {
		switch line {
		case "A X":
			total += 4
		case "A Y":
			total += 8
		case "A Z":
			total += 3
		case "B X":
			total += 1
		case "B Y":
			total += 5
		case "B Z":
			total += 9
		case "C X":
			total += 7
		case "C Y":
			total += 2
		case "C Z":
			total += 6
		}
	}

	return total
}

func PartTwo(input []string) int {
	var total int
	for _, line := range input {
		switch line {
		case "A X":
			total += 3
		case "A Y":
			total += 4
		case "A Z":
			total += 8
		case "B X":
			total += 1
		case "B Y":
			total += 5
		case "B Z":
			total += 9
		case "C X":
			total += 2
		case "C Y":
			total += 6
		case "C Z":
			total += 7
		}
	}

	return total
}
