package day14

import (
	"fmt"
	"strings"

	"github.com/drdaemos/advent-of-code/utils"
	"github.com/samber/lo"
)

type pos struct {
	x, y int
}

type plane2d = map[pos]string

const (
	Air  string = ""
	Rock string = "▓"
	Sand string = "░"
	Flow string = "~"
)

const InputTest = `
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9`

// Main https://adventofcode.com/2022/day/14
func Main() {
	input := utils.GetStrings(utils.GetPackageInput("day14"))

	fmt.Println("Part one:", PartOne(input))
	fmt.Println("Part two:", PartTwo(input))
}

func ParseCave(input []string) (plane2d, int, int) {
	cave := make(map[pos]string)
	lowestY := 0
	leftmostX := 999

	for i := 0; i < len(input); i++ {
		points := lo.Map(
			strings.Split(input[i], "->"),
			func(x string, _ int) pos {
				parts := lo.Map(strings.Split(strings.TrimSpace(x), ","), func(item string, _ int) int { return utils.ToInt(item) })
				if parts[0] < leftmostX {
					leftmostX = parts[0]
				}
				if parts[1] > lowestY {
					lowestY = parts[1]
				}
				return pos{parts[0], parts[1]}
			},
		)

		for i := 1; i < len(points); i++ {
			head := points[i-1]
			tail := points[i]

			if head.x == tail.x {
				var yrange []int
				if head.y > tail.y {
					yrange = utils.MakeRange(tail.y, head.y)
				} else {
					yrange = utils.MakeRange(head.y, tail.y)
				}

				for y := yrange[0]; y < len(yrange)+yrange[0]; y++ {
					cave[pos{head.x, y}] = Rock
				}
			} else if head.y == tail.y {
				var xrange []int
				if head.x > tail.x {
					xrange = utils.MakeRange(tail.x, head.x)
				} else {
					xrange = utils.MakeRange(head.x, tail.x)
				}

				for x := xrange[0]; x < len(xrange)+xrange[0]; x++ {
					cave[pos{x, head.y}] = Rock
				}
			}
		}
	}

	return cave, leftmostX, lowestY
}

func SandNextPos(unit pos, cave plane2d, lowestY int) pos {
	if unit.y >= lowestY {
		return pos{-1, -1}
	}

	under := cave[pos{unit.x, unit.y + 1}]
	if under == Air {
		return pos{unit.x, unit.y + 1}
	}

	left := cave[pos{unit.x - 1, unit.y + 1}]
	if left == Air {
		return pos{unit.x - 1, unit.y + 1}
	}

	right := cave[pos{unit.x + 1, unit.y + 1}]
	if right == Air {
		return pos{unit.x + 1, unit.y + 1}
	}

	return unit
}

func AddSand(cave plane2d) pos {
	cave[pos{500, 0}] = Sand

	return pos{500, 0}
}

func PosEquals(a pos, b pos) bool {
	return a.x == b.x && a.y == b.y
}

func PartOne(input []string) int {
	cave, _, _ := ParseCave(input)
	fmt.Println(cave)
	return 0
}

func PartTwo(input []string) int {
	return 0
}
