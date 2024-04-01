package main_test

import (
	"testing"

	day09 "github.com/imawizard/Advent-of-Code/2022/Day-09"
)

func TestPartA(t *testing.T) {
	input := day09.Parse(`
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
    `)

	if want, got := 13, day09.PartA(input); got != want {
		t.Errorf("got: %d, want: %d", got, want)
	}
}

func TestPartB(t *testing.T) {
	input1 := day09.Parse(`
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
    `)
	input2 := day09.Parse(`
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
    `)

	if want, got := 1, day09.PartB(input1, 10); got != want {
		t.Errorf("got: %d, want: %d", got, want)
	}
	if want, got := 36, day09.PartB(input2, 10); got != want {
		t.Errorf("got: %d, want: %d", got, want)
	}
}
