package main_test

import (
	"testing"

	day03 "github.com/imawizard/Advent-of-Code/2021/Day-03"
)

var numbers, bitcnt = day03.Parse(`
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
`)

func TestPartA(t *testing.T) {
	gamma, epsilon := day03.PartA(numbers, bitcnt, bitcnt)
	if want, got := 198, gamma*epsilon; got != want {
		t.Errorf("got: %d, want: %d", got, want)
	}
}

func TestPartB(t *testing.T) {
	oxygen, co2 := day03.PartB(numbers, bitcnt)
	if want, got := 230, oxygen*co2; got != want {
		t.Errorf("got: %d, want: %d", got, want)
	}
}
