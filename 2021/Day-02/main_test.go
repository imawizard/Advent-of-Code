package main_test

import (
	"testing"

	day02 "github.com/imawizard/Advent-of-Code/2021/Day-02"
)

var input = day02.Parse(`
forward 5
down 5
forward 8
up 3
down 8
forward 2
`)

func TestPartA(t *testing.T) {
	horz, depth := day02.PartA(input)
	if want, got := 150, horz*depth; got != want {
		t.Errorf("got: %d, want: %d", got, want)
	}
}

func TestPartB(t *testing.T) {
	horz, depth := day02.PartB(input)
	if want, got := 900, horz*depth; got != want {
		t.Errorf("got: %d, want: %d", got, want)
	}
}
