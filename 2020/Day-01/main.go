package main

import (
	"bufio"
	_ "embed"
	"fmt"
	"strconv"
	"strings"
)

func PartA(vals []int) int {
	for i := 0; i < len(vals)-1; i++ {
		for j := 1; j < len(vals); j++ {
			if vals[i]+vals[j] == 2020 {
				return vals[i] * vals[j]
			}
		}
	}
	return 0
}

func PartB(vals []int) int {
	for i := 0; i < len(vals)-2; i++ {
		for j := 1; j < len(vals)-1; j++ {
			for k := 2; k < len(vals); k++ {
				if vals[i]+vals[j]+vals[k] == 2020 {
					return vals[i] * vals[j] * vals[k]
				}
			}
		}
	}
	return 0
}

func Parse(s string) []int {
	var res []int
	sc := bufio.NewScanner(strings.NewReader(s))
	for sc.Scan() {
		if sc.Text() == "" {
			continue
		}
		val, _ := strconv.ParseInt(sc.Text(), 10, 0)
		res = append(res, int(val))
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
