package day02

import (
	"strings"
	"testing"
)

const Input = `A Y
B X
C Z`

func TestPartOne(t *testing.T) {
	actual := PartOne(convertInput(Input))
	if actual != 15 {
		t.Errorf("PartOne = %d; want 15", actual)
	}
}

func TestPartTwo(t *testing.T) {
	actual := PartTwo(convertInput(Input))
	if actual != 12 {
		t.Errorf("PartTwo = %d; want 12", actual)
	}
}

func convertInput(input string) []string {
	return strings.Split(input, "\n")
}
