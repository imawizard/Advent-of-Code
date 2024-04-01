package main_test

import (
	"testing"

	day01 "github.com/imawizard/Advent-of-Code/2021/Day-01"
)

var input = day01.Parse(`
199
200
208
210
200
207
240
269
260
263
`)

func TestPartA(t *testing.T) {
	if want, got := 7, day01.PartA(input); got != want {
		t.Errorf("got: %d, want: %d", got, want)
	}
}

func TestPartB(t *testing.T) {
	if want, got := 5, day01.PartB(input); got != want {
		t.Errorf("got: %d, want: %d", got, want)
	}
}
