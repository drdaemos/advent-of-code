package day01

import (
	"strings"
	"testing"
)

const Input = `1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet`

const Input2 = `two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
oneight`

func TestPartOne(t *testing.T) {
	actual := PartOne(convertInput(Input))
	if actual != 142 {
		t.Errorf("PartOne = %d; want 142", actual)
	}
}

func TestPartTwo(t *testing.T) {
	actual := PartTwo(convertInput(Input2))
	if actual != 299 {
		t.Errorf("PartTwo = %d; want 299", actual)
	}
}

func convertInput(input string) []string {
	return strings.Split(input, "\n")
}
