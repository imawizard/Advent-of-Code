package main_test

import (
	"testing"

	day01 "github.com/imawizard/Advent-of-Code/2020/Day-01"
)

var input = day01.Parse(`
1721
979
366
299
675
1456
`)

func TestPartA(t *testing.T) {
	if want, got := 514579, day01.PartA(input); got != want {
		t.Errorf("got: %d, want: %d", got, want)
	}
}

func TestPartB(t *testing.T) {
	if want, got := 241861950, day01.PartB(input); got != want {
		t.Errorf("got: %d, want: %d", got, want)
	}
}
