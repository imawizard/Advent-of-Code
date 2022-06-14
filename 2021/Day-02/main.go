package main

import (
	"bufio"
	"bytes"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func PartA(input []Line) (int, int) {
	horz, depth := 0, 0

	for _, v := range input {
		switch v.dir {
		case "forward":
			horz += v.x
		case "up":
			depth -= v.x
		case "down":
			depth += v.x
		}
	}

	return horz, depth
}

func PartB(input []Line) (int, int) {
	horz, depth, aim := 0, 0, 0

	for _, v := range input {
		switch v.dir {
		case "forward":
			horz += v.x
			depth += aim * v.x
		case "up":
			aim -= v.x
		case "down":
			aim += v.x
		}
	}

	return horz, depth
}

type Line struct {
	dir string
	x   int
}

func main() {
	input, _ := os.ReadFile("input")
	s := bufio.NewScanner(bytes.NewReader(input))

	var lines []Line
	for s.Scan() {
		f := strings.Fields(s.Text())
		if len(f) < 2 {
			continue
		}

		dir := strings.ToLower(f[0])
		x, _ := strconv.Atoi(f[1])
		lines = append(lines, Line{dir, x})
	}

	horz, depth := PartA(lines)
	fmt.Println("a:", horz*depth)

	horz, depth = PartB(lines)
	fmt.Println("b:", horz*depth)
}
