package day02

import (
	"fmt"
	"regexp"
	"strconv"
	"strings"

	"github.com/drdaemos/advent-of-code/utils"
)

type CubeSet struct {
	Red   int
	Green int
	Blue  int
}

// Main https://adventofcode.com/2023/day/2
func Main() {
	input := utils.GetStrings(utils.GetPackageInput("day02"))

	fmt.Println("Sum of game IDs:", PartOne(input))
	fmt.Println("Sum of powers:", PartTwo(input))
}

func PartOne(input []string) int {
	var validIds []int
	for i, str := range input {
		sets := parseCubeSets(str)
		if isPossibleGame(sets) {
			validIds = append(validIds, i+1)
		}
	}

	sum := utils.Sum(validIds)

	return sum
}

func PartTwo(input []string) int {
	var powers []int
	for _, str := range input {
		sets := parseCubeSets(str)
		gameSet := findMinimalGameSet(sets)
		power := findSetPower(gameSet)
		powers = append(powers, power)
	}

	sum := utils.Sum(powers)

	return sum
}

func isPossibleGame(sets []CubeSet) bool {
	for _, set := range sets {
		if set.Red > 12 || set.Green > 13 || set.Blue > 14 {
			return false
		}
	}

	return true
}

func findMinimalGameSet(sets []CubeSet) CubeSet {
	gameSet := CubeSet{
		Red:   0,
		Green: 0,
		Blue:  0,
	}
	for _, set := range sets {
		gameSet.Red = utils.Max(gameSet.Red, set.Red)
		gameSet.Green = utils.Max(gameSet.Green, set.Green)
		gameSet.Blue = utils.Max(gameSet.Blue, set.Blue)
	}
	return gameSet
}

func findSetPower(set CubeSet) int {
	return set.Red * set.Green * set.Blue
}

func parseCubeSets(input string) []CubeSet {
	var result []CubeSet
	re := regexp.MustCompile(`Game \d+: (.*)`)
	setsString := re.FindString(input)
	sets := strings.Split(setsString, "; ")

	for _, str := range sets {
		result = append(result, parseSet(str))
	}
	return result
}

func parseSet(input string) CubeSet {
	return CubeSet{
		Red:   parseCubeCount(input, "red"),
		Green: parseCubeCount(input, "green"),
		Blue:  parseCubeCount(input, "blue"),
	}
}

func parseCubeCount(input string, kind string) int {
	re := regexp.MustCompile(`(\d+) ` + kind)
	matches := re.FindStringSubmatch(input)
	if len(matches) > 1 {
		result, err := strconv.Atoi(matches[1])
		if err != nil {
			panic(err)
		}

		return result
	} else {
		return 0
	}
}
