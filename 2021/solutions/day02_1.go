package main

import (
	"fmt"
	"github.com/drdaemos/advent-of-code/utils"
	"strings"
)

/*
https://adventofcode.com/2021/day/2
Input is a series of instructions to modify the position of a certain object on y-down 2d space
Each line represents a separate command, like:
- forward X increases the horizontal position by X units.
- down X increases the depth by X units.
- up X decreases the depth by X units.
Output is final coordinates of an object and a multiple of them.
*/
func main() {
	input := utils.GetStrings(utils.GetInputPath("day02.txt"))

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
	fmt.Println("multiply:", xPos*yPos)
}
