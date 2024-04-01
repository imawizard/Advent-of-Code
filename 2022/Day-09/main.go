package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"math"
	"strconv"
	"strings"
)

func PartA(input []Move) int {
	m := make(map[Point]struct{})

	head := Point{}
	tail := Point{}

	for _, move := range input {
		for range move.steps {
			switch move.direction {
			case up:
				head.y++
			case down:
				head.y--
			case left:
				head.x--
			case right:
				head.x++
			}

			if math.Abs(float64(head.x-tail.x)) > 1 || math.Abs(float64(head.y-tail.y)) > 1 {
				switch move.direction {
				case up:
					tail.y++
				case down:
					tail.y--
				case left:
					tail.x--
				case right:
					tail.x++
				}

				switch move.direction {
				case up, down:
					tail.x = head.x
				case left, right:
					tail.y = head.y
				}
			}

			m[tail] = struct{}{}
		}
	}
	return len(m)
}

func PartB(input []Move, length int) int {
	m := make(map[Point]struct{})
	points := make([]Point, length)

	for _, move := range input {
		for range move.steps {
			dir := move.direction

			for idx := 1; idx < len(points); idx++ {
				head := &points[idx-1]
				tail := &points[idx]

				if idx == 1 {
					switch dir {
					case up:
						head.y++
					case down:
						head.y--
					case left:
						head.x--
					case right:
						head.x++
					}
				}

				x_diff := head.x - tail.x
				y_diff := head.y - tail.y
				if math.Abs(float64(x_diff)) > 1 {
					if x_diff > 0 {
						tail.x++
					} else if x_diff < 0 {
						tail.x--
					}

					if y_diff > 0 {
						tail.y++
					} else if y_diff < 0 {
						tail.y--
					}
				} else if math.Abs(float64(y_diff)) > 1 {
					if y_diff > 0 {
						tail.y++
					} else if y_diff < 0 {
						tail.y--
					}
					if math.Abs(float64(x_diff)) > 0 {
						if x_diff > 0 {
							tail.x++
						} else if x_diff < 0 {
							tail.x--
						}
					}
				}
				if idx == len(points)-1 {
					m[*tail] = struct{}{}
				}
			}
		}
	}
	return len(m)
}

type Point struct {
	x int
	y int
}

type Direction int

const (
	up Direction = iota
	down
	left
	right
)

type Move struct {
	direction Direction
	steps     uint
}

func Parse(s string) []Move {
	var res []Move

	sc := bufio.NewScanner(strings.NewReader(s))
	for sc.Scan() {
		before, after, found := strings.Cut(strings.TrimSpace(sc.Text()), " ")
		if !found {
			continue
		}

		var dir Direction
		switch before {
		case "U":
			dir = up
		case "D":
			dir = down
		case "L":
			dir = left
		case "R":
			dir = right
		}

		steps, err := strconv.Atoi(after)
		if steps < 1 || err != nil {
			continue
		}

		res = append(res, Move{
			direction: dir,
			steps:     uint(steps),
		})
	}
	return res
}

//go:embed input
var inputText string

func main() {
	input := Parse(inputText)

	fmt.Printf("a: %d\n", PartA(input))
	fmt.Printf("b: %d\n", PartB(input, 10))
}
