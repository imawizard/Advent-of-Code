package main

import (
	"bufio"
	"bytes"
	"fmt"
	"math/bits"
	"os"
	"strings"
)

func PartA(input []int, digits int, length int) (int, int) {
	ratios := make(map[int]int)

	for _, number := range input {
		for i := 0; i < length; i++ {
			bit := number >> (digits - 1) & 1
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

func PartB(input []int, digits int) (int, int) {
	var left []int

	oxygenGeneratorRating := 0
	left = append(left, input...)
	for i := 0; i < digits; i++ {
		gamma, _ := PartA(left, digits-i, 1)
		for j := 0; j < len(left); j++ {
			if left[j]>>(digits-i-1)&1 != gamma {
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
	for i := 0; i < digits; i++ {
		_, epsilon := PartA(left, digits-i, 1)
		for j := 0; j < len(left); j++ {
			if left[j]>>(digits-i-1)&1 != epsilon {
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

func main() {
	input, _ := os.ReadFile("input")
	s := bufio.NewScanner(bytes.NewReader(input))

	var numbers []int
	var digits int
	for s.Scan() {
		r := strings.NewReader(s.Text())
		var value int
		fmt.Fscanf(r, "%b", &value)
		numbers = append(numbers, value)
		if bits.Len(uint(value)) > digits {
			digits = bits.Len(uint(value))
		}
	}

	gamma, epsilon := PartA(numbers, digits, digits)
	fmt.Println("a:", gamma*epsilon)

	oxygen, co2 := PartB(numbers, digits)
	fmt.Println("b:", oxygen*co2)
}
