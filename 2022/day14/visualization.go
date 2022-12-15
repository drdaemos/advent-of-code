package day14

import (
	"errors"
	"fmt"
	"time"

	"github.com/drdaemos/advent-of-code/utils"
	"github.com/gdamore/tcell/v2"
	"github.com/samber/lo"
)

type Vis struct {
	screen  tcell.Screen
	halt    bool
	pause   bool
	leftX   int
	bottomY int
	yOffset int
	ticker  *time.Ticker
	tick    int
	event   chan Event
	cave    plane2d
	sand    pos
	rested  int
}

type Event struct {
	Type string
}

const (
	halt       = "halt"
	scrollup   = "scrollup"
	scrolldown = "scrolldown"
)

func Visualization() {
	input := utils.GetStrings(utils.GetPackageInput("day14")) // 692

	defStyle := tcell.StyleDefault.Background(tcell.ColorReset).Foreground(tcell.ColorReset)
	screen := lo.Must(tcell.NewScreen())
	if err := screen.Init(); err != nil {
		panic(err)
	}

	screen.SetStyle(defStyle)
	screen.Clear()

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

	cave, leftX, bottomY := ParseCave(input)

	ticker := time.NewTicker(1 * time.Millisecond)
	defer ticker.Stop()

	event := make(chan Event)

	context := Vis{
		screen:  screen,
		halt:    false,
		pause:   false,
		cave:    cave,
		leftX:   leftX,
		bottomY: bottomY,
		yOffset: 9,
		ticker:  ticker,
		tick:    0,
		event:   event,
		sand:    pos{0, 0},
		rested:  -1,
	}

	go eventLoop(screen, event)

	mainLoop(&context)
}

func mainLoop(context *Vis) {
	// Event loop
	for {
		select {
		case ev := <-context.event:
			switch ev.Type {
			case halt:
				return
			case scrollup:
				if context.yOffset >= 3 {
					context.yOffset -= 3
				}
			case scrolldown:
				if context.yOffset < context.bottomY-20 {
					context.yOffset += 3
				}
			}
		case <-context.ticker.C:
			update(context)
		}

		render(context)
	}
}

func update(context *Vis) {
	if !context.pause {
		context.tick++

		if PosEquals(context.sand, pos{0, 0}) {
			context.sand = AddSand(context.cave)
			context.rested++
		} else {
			moved, err := moveActiveSand(context)
			if err != nil {
				context.pause = true
			}
			context.sand = moved
		}
	}
}

func moveActiveSand(context *Vis) (pos, error) {
	moved := SandNextPos(context.sand, context.cave, context.bottomY+1)
	if PosEquals(moved, pos{-1, -1}) {
		return moved, errors.New("sand fell in abyss")
	}
	delete(context.cave, context.sand)
	context.cave[moved] = Sand

	if PosEquals(context.sand, moved) {
		return pos{0, 0}, nil
	} else {
		return moved, nil
	}
}

func eventLoop(s tcell.Screen, event chan<- Event) {
	for {
		ev := s.PollEvent()

		switch ev := ev.(type) {
		case *tcell.EventResize:
			s.Sync()
		case *tcell.EventKey:
			if ev.Key() == tcell.KeyEscape || ev.Key() == tcell.KeyCtrlC {
				event <- Event{Type: halt}
			} else if ev.Key() == tcell.KeyCtrlL {
				s.Sync()
			} else if ev.Rune() == 'C' || ev.Rune() == 'c' {
				s.Clear()
			} else if ev.Key() == tcell.KeyUp {
				event <- Event{Type: scrollup}
			} else if ev.Key() == tcell.KeyDown {
				event <- Event{Type: scrolldown}
			}
		default:
			continue
		}
	}
}

func render(context *Vis) {
	context.screen.Fill(' ', tcell.StyleDefault)

	white := tcell.StyleDefault.Foreground(tcell.ColorLightGoldenrodYellow).Background(tcell.ColorDarkBlue)
	yellow := tcell.StyleDefault.Foreground(tcell.ColorLightYellow).Background(tcell.ColorBlack)
	brown := tcell.StyleDefault.Foreground(tcell.ColorBurlyWood).Background(tcell.ColorBlack)

	drawCave(context.screen, context.cave, context.leftX-10, context.yOffset, brown, yellow)
	drawBottom(context.screen, context.bottomY+5, context.yOffset, brown)
	drawText(context.screen, 0, 0, 20, 1, fmt.Sprintf("Screen: %d:%d", 0, context.yOffset), white)
	drawText(context.screen, 0, 1, 20, 2, fmt.Sprintf("Cave: %d:%d", context.leftX, context.bottomY), white)
	drawText(context.screen, 0, 2, 20, 3, fmt.Sprintf("Tick: %d", context.tick), white)
	drawText(context.screen, 0, 3, 20, 4, fmt.Sprintf("Rested: %d", context.rested), white)

	context.screen.Show()
}

func drawBottom(s tcell.Screen, bottomY int, vertOffset int, style tcell.Style) {
	for x := 0; x < 100; x++ {
		s.SetContent(x, bottomY-vertOffset, '╦', nil, style)
	}
}

func drawCave(s tcell.Screen, cave map[pos]string, horOffset int, vertOffset int, rock tcell.Style, sand tcell.Style) {
	for pos, cell := range cave {
		runes := []rune(cell)

		if runes[0] == '▓' {
			s.SetContent(pos.x-horOffset, pos.y-vertOffset, runes[0], nil, rock)
		} else {
			s.SetContent(pos.x-horOffset, pos.y-vertOffset, runes[0], nil, sand)
		}
	}
}

func drawText(s tcell.Screen, x1, y1, x2, y2 int, text string, style tcell.Style) {
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
