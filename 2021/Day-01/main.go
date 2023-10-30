package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"strconv"
	"strings"
)

func PartA(input []int) int {
	var n int
	for i := range input[:len(input)-1] {
		if input[i] < input[i+1] {
			n++
		}
	}
	return n
}

func PartB(input []int) int {
	var windows []int
	for i := 0; i < len(input)-len(input)%3; i++ {
		window := input[i+0] +
			input[i+1] +
			input[i+2]
		windows = append(windows, window)
	}

	return PartA(windows)
}

func Parse(s string) []int {
	var res []int
	sc := bufio.NewScanner(strings.NewReader(s))
	for sc.Scan() {
		if m, err := strconv.Atoi(sc.Text()); err == nil {
			res = append(res, m)
		}
	}
	return res
}

//go:embed input
var inputText string

func main() {
	input := Parse(inputText)

	fmt.Printf("a: %d\n", PartA(input))
	fmt.Printf("b: %d\n", PartB(input))
}
