package day14

import (
	"fmt"
	"strings"

	"github.com/drdaemos/advent-of-code/utils"
	"github.com/gdamore/tcell/v2"
	"github.com/samber/lo"
)

type pos struct {
	x, y int
}

const (
	Air  string = "."
	Rock string = "█"
	Sand string = "o"
	Flow string = "~"
)

// Main https://adventofcode.com/2022/day/14
func Main() {
	input := utils.GetStrings(utils.GetPackageInput("day14"))

	fmt.Println("Part one:", PartOne(input))
	fmt.Println("Part two:", PartTwo(input))
}

func Visualization() {
	input := utils.GetStrings(utils.GetPackageInput("day14"))

	defStyle := tcell.StyleDefault.Background(tcell.ColorReset).Foreground(tcell.ColorReset)
	// style := tcell.StyleDefault.Foreground(tcell.ColorWhite).Background(tcell.ColorPurple)
	screen := lo.Must(tcell.NewScreen())
	if err := screen.Init(); err != nil {
		panic(err)
	}

	// xmax, ymax := screen.Size()

	screen.SetStyle(defStyle)
	screen.Clear()

	cave := ParseCave(input)
	DrawCave(screen, cave)

	// screen.SetContent(10, 10, tcell.RuneHLine, nil, style)
	// screen.SetContent(20, 20, tcell.RuneHLine, nil, style)
	// drawText(screen, 10, 10, 50, 20, style, fmt.Sprint(xmax, ymax))

	quit := func() {
		// You have to catch panics in a defer, clean up, and
		// re-raise them - otherwise your application can
		// die without leaving any diagnostic trace.
		maybePanic := recover()
		screen.Fini()
		if maybePanic != nil {
			panic(maybePanic)
		}
	}
	defer quit()

	// Event loop
	for {
		// Update screen
		screen.Show()

		// Poll event
		ev := screen.PollEvent()

		// Process event
		switch ev := ev.(type) {
		case *tcell.EventResize:
			screen.Sync()
		case *tcell.EventKey:
			if ev.Key() == tcell.KeyEscape || ev.Key() == tcell.KeyCtrlC {
				return
			} else if ev.Key() == tcell.KeyCtrlL {
				screen.Sync()
			} else if ev.Rune() == 'C' || ev.Rune() == 'c' {
				screen.Clear()
			}
		}
	}
}

func drawText(s tcell.Screen, x1, y1, x2, y2 int, style tcell.Style, text string) {
	row := y1
	col := x1
	for _, r := range text {
		s.SetContent(col, row, r, nil, style)
		col++
		if col >= x2 {
			row++
			col = x1
		}
		if row > y2 {
			break
		}
	}
}

func ParseCave(input []string) map[pos]string {
	cave := make(map[pos]string)

	for i := 0; i < len(input); i++ {
		points := lo.Map(
			strings.Split(input[i], "->"),
			func(x string, _ int) pos {
				parts := lo.Map(strings.Split(strings.TrimSpace(x), ","), func(item string, _ int) int { return utils.ToInt(item) })
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

	return cave
}

func DrawCave(screen tcell.Screen, cave map[pos]string) {
	style := tcell.StyleDefault.Foreground(tcell.ColorWhite).Background(tcell.ColorPurple)
	for pos, cell := range cave {
		runes := []rune(cell)
		screen.SetContent(pos.x-400, pos.y-10, runes[0], nil, style)
	}
}

func PartOne(input []string) int {
	cave := ParseCave(input)
	fmt.Println(cave)
	return 0
}

func PartTwo(input []string) int {
	return 0
}
