package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"math/bits"
	"strings"
)

func PartA(input []int, bitcnt int, length int) (int, int) {
	ratios := make(map[int]int)

	for _, number := range input {
		for i := 0; i < length; i++ {
			bit := number >> (bitcnt - 1) & 1
			r := ratios[i]
			ratios[i] = r + (bit*2 - 1)
			number <<= 1
		}
	}

	gamma := 0
	for i := 0; i < len(ratios); i++ {
		ratio := ratios[i]
		gamma <<= 1
		switch {
		case ratio < 0:
			gamma &= ^1
		case ratio == 0:
			fallthrough
		case ratio > 0:
			gamma |= 1
		}
	}

	epsilon := ^gamma & int(^(^uint(0) << len(ratios)))
	return gamma, epsilon
}

func PartB(input []int, bitcnt int) (int, int) {
	var left []int

	oxygenGeneratorRating := 0
	left = append(left, input...)
	for i := 0; i < bitcnt; i++ {
		gamma, _ := PartA(left, bitcnt-i, 1)
		for j := 0; j < len(left); j++ {
			if left[j]>>(bitcnt-i-1)&1 != gamma {
				left = append(left[:j], left[j+1:]...)
				j--
			}
		}
		if len(left) == 1 {
			oxygenGeneratorRating = left[0]
			break
		}
	}

	co2ScrubberRating := 0
	left = append(left, input...)
	for i := 0; i < bitcnt; i++ {
		_, epsilon := PartA(left, bitcnt-i, 1)
		for j := 0; j < len(left); j++ {
			if left[j]>>(bitcnt-i-1)&1 != epsilon {
				left = append(left[:j], left[j+1:]...)
				j--
			}
		}
		if len(left) == 1 {
			co2ScrubberRating = left[0]
			break
		}
	}

	return oxygenGeneratorRating, co2ScrubberRating
}

func Parse(s string) ([]int, int) {
	var numbers []int
	var bitcnt int
	sc := bufio.NewScanner(strings.NewReader(s))
	for sc.Scan() {
		r := strings.NewReader(sc.Text())
		var value int
		fmt.Fscanf(r, "%b", &value)
		numbers = append(numbers, value)
		if bits.Len(uint(value)) > bitcnt {
			bitcnt = bits.Len(uint(value))
		}
	}
	return numbers, bitcnt
}

//go:embed input
var inputText string

func main() {
	numbers, bitcnt := Parse(inputText)

	gamma, epsilon := PartA(numbers, bitcnt, bitcnt)
	fmt.Printf("a: %d\n", gamma*epsilon)

	oxygen, co2 := PartB(numbers, bitcnt)
	fmt.Printf("b: %d\n", oxygen*co2)
}
