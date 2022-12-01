package day01

import (
	"strings"
	"testing"
)

const Input = `1000
2000
3000

4000

5000
6000

7000
8000
9000

10000`

func TestPartOne(t *testing.T) {
	actual := PartOne(convertInput(Input))
	if actual != 24000 {
		t.Errorf("PartOne = %d; want 24000", actual)
	}
}

func convertInput(input string) []string {
	return strings.Split(input, "\n")
}
