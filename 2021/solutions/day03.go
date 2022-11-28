package main

import (
	"fmt"
	"github.com/drdaemos/advent-of-code/utils"
	"math"
	"strconv"
	"strings"
)

/*
https://adventofcode.com/2021/day/3
*/
func main() {
	input := utils.GetStrings(utils.GetInputPath("day03.txt"))

	fmt.Println("Power consumption:", partOne(input))
	fmt.Println("Life support rating:", partTwo(input))
}

func partOne(input []string) uint64 {
	mostCommonBits := findMostCommonBits(input)

	// parse binary to learn gamma rate
	gammaRate, _ := strconv.ParseUint(mostCommonBits, 2, len(mostCommonBits))

	// and flip it to find least common bits - epsilon rate
	allBits := uint64(math.Pow(2, float64(len(mostCommonBits)))) - 1
	epsilonRate := allBits ^ gammaRate

	// print power consumption
	return gammaRate * epsilonRate
}

func partTwo(input []string) int {
	//mostCommonBits := findMostCommonBits(input)

	return 0
}

func findMostCommonBits(input []string) string {
	var inputLength int
	bits := make([]int, len(input[0]))

	// count positive bits for each position
	for _, line := range input {
		for pos, char := range line {
			if char == '1' {
				bits[pos]++
			}
		}

		inputLength++
	}

	// build binary with most common bits
	var builder strings.Builder
	for _, bitCount := range bits {
		if bitCount > (inputLength / 2) {
			builder.WriteRune('1')
		} else {
			builder.WriteRune('0')
		}
	}

	return builder.String()
}
