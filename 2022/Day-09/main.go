package main

import (
	_ "embed"
	"fmt"
	"testing"
)

func PartA(input []string) int {
	return 0
}

func PartB(input []string) int {
	return 0
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

func Parse(s string) []string {
	var res []string
	return res
}

//go:embed input
var inputText string

func main() {
	input := Parse(inputText)
	fmt.Printf("a: %d\n", PartA(input))
}

func TestPartA(t *testing.T) {
	return
}
