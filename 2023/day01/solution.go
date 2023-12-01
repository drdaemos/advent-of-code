package day01

import (
	"fmt"
	"regexp"
	"strconv"
	"unicode"

	"github.com/drdaemos/advent-of-code/utils"
)

// Main https://adventofcode.com/2023/day/1
func Main() {
	input := utils.GetStrings(utils.GetPackageInput("day01"))

	fmt.Println("Calibration value:", PartOne(input))
	fmt.Println("Real calibration value:", PartTwo(input))
}

func PartOne(input []string) int {
	var lineValues []int
	for _, str := range input {
		lineValues = append(lineValues, getLineCalibrationValue(str))
	}

	sum := utils.Sum(lineValues)

	return sum
}

func PartTwo(input []string) int {
	var lineValues []int
	for _, str := range input {
		lineValues = append(lineValues, getRealLineCalibrationValue(str))
	}

	sum := utils.Sum(lineValues)

	return sum
}

func getLineCalibrationValue(str string) int {
	var left rune
	var right rune

	// left digit
	for _, ch := range str {
		if unicode.IsDigit(ch) {
			left = ch
			break
		}
	}

	// right digit
	for _, ch := range utils.Reverse(str) {
		if unicode.IsDigit(ch) {
			right = ch
			break
		}
	}

	var number = string(left) + string(right)

	result, err := strconv.Atoi(number)
	if err != nil {
		panic(err)
	}

	return result
}

func getRealLineCalibrationValue(str string) int {
	var left = getFirstDigit(str)
	var right = getLastDigit(str)

	result, err := strconv.Atoi(left + right)
	if err != nil {
		panic(err)
	}

	return result
}

func getFirstDigit(str string) string {
	re := regexp.MustCompile(`(one|two|three|four|five|six|seven|eight|nine|[1-9])`)
	match := re.FindString(str)

	return convertToNumeric(match)
}

func getLastDigit(str string) string {
	re := regexp.MustCompile("(one|two|three|four|five|six|seven|eight|nine|[1-9])")
	start := len(str) - 1

	for start > 0 {
		match := re.FindString(str[start:])
		if match != "" {
			return convertToNumeric(match)
		}
		start -= 1
	}
	return ""
}

func convertToNumeric(input string) string {
	switch input {
	case "one":
		return "1"
	case "two":
		return "2"
	case "three":
		return "3"
	case "four":
		return "4"
	case "five":
		return "5"
	case "six":
		return "6"
	case "seven":
		return "7"
	case "eight":
		return "8"
	case "nine":
		return "9"
	default:
		return input
	}
}
