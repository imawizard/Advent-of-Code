package main

import (
	"bufio"
	"bytes"
	"fmt"
	"os"
	"strconv"
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

func main() {
	input, _ := os.ReadFile("input")
	s := bufio.NewScanner(bytes.NewReader(input))

	var measurements []int
	for s.Scan() {
		if m, err := strconv.Atoi(s.Text()); err == nil {
			measurements = append(measurements, m)
		}
	}

	fmt.Println("a:", PartA(measurements))
	fmt.Println("b:", PartB(measurements))
}
