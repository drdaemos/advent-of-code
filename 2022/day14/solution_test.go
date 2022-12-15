package day14

import (
	"fmt"
	"reflect"
	"strings"
	"testing"
)

const Input = `
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9`

func TestPartOne(t *testing.T) {
	actual := PartOne(convertInput(Input))
	if actual != 24 {
		t.Errorf("PartOne = %d; want 24", actual)
	}
}

func TestParseCave(t *testing.T) {
	actual := ParseCave(convertInput(Input))
	expected := map[pos]string{{498, 4}: Rock}
	if !reflect.DeepEqual(actual, expected) {
		fmt.Print(expected)
	}
}

func convertInput(input string) []string {
	return strings.Split(input, "\n")
}
