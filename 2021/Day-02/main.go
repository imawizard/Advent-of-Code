package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"strconv"
	"strings"
)

func PartA(input []line) (int, int) {
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

func PartB(input []line) (int, int) {
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

type line struct {
	dir string
	x   int
}

func Parse(s string) []line {
	var res []line
	sc := bufio.NewScanner(strings.NewReader(s))
	for sc.Scan() {
		f := strings.Fields(sc.Text())
		if len(f) < 2 {
			continue
		}

		dir := strings.ToLower(f[0])
		x, _ := strconv.Atoi(f[1])
		res = append(res, line{dir, x})
	}
	return res
}

//go:embed input
var inputText string

func main() {
	input := Parse(inputText)

	horz, depth := PartA(input)
	fmt.Printf("a: %d\n", horz*depth)

	horz, depth = PartB(input)
	fmt.Printf("b: %d\n", horz*depth)
}
