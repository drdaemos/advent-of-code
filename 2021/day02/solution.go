package day02

import (
	"fmt"
	"github.com/drdaemos/advent-of-code/utils"
	"strings"
)

func Main() {
	input := utils.GetStrings(utils.GetPackageInput("day02"))

	partOne(input)
	partTwo(input)
}

/*
https://adventofcode.com/2021/day/2
Input is a series of instructions to modify the position of a certain object on y-down 2d space
Each line represents a separate command, like:
- forward X increases the horizontal position by X units.
- down X increases the depth by X units.
- up X decreases the depth by X units.
Output is final coordinates of an object and a multiple of them.
*/
func partOne(input []string) {
	var xPos int
	var yPos int

	for _, line := range input {
		// [0] is command, [1] is amount
		instruction := strings.Fields(line)
		command := instruction[0]
		amount := utils.ToInt(instruction[1])

		switch command {
		case "forward":
			xPos += amount
		case "down":
			yPos += amount
		case "up":
			yPos -= amount
		}
	}

	fmt.Println("x:", xPos, "y:", yPos)
	fmt.Println("[p.1] multiply:", xPos*yPos)
}

/*
https://adventofcode.com/2021/day/2#part2
Input is a series of instructions to modify the position of a certain object on y-down 2d space
Each line represents a separate command, like:
  - down X increases your aim by X units.
  - up X decreases your aim by X units.
  - forward X does two things:
    It increases your horizontal position by X units.
    It increases your depth by your aim multiplied by X.

Output is final coordinates of an object and a multiple of them.
*/
func partTwo(input []string) {
	var xPos int
	var yPos int
	var aim int

	for _, line := range input {
		// [0] is command, [1] is amount
		instruction := strings.Fields(line)
		command := instruction[0]
		amount := utils.ToInt(instruction[1])

		switch command {
		case "forward":
			xPos += amount
			yPos += amount * aim
		case "down":
			aim += amount
		case "up":
			aim -= amount
		}
	}

	fmt.Println("x:", xPos, "y:", yPos)
	fmt.Println("[p.2] multiply:", xPos*yPos)
}
