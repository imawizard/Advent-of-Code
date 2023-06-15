package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func partA(vals []int) int {
	for i := 0; i < len(vals)-1; i++ {
		for j := 1; j < len(vals); j++ {
			if vals[i]+vals[j] == 2020 {
				return vals[i] * vals[j]
			}
		}
	}
	return 0
}

func partB(vals []int) int {
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

func main() {
	f, _ := os.Open("input")
	defer f.Close()
	s := bufio.NewScanner(f)
	var vals []int
	for s.Scan() {
		val, _ := strconv.ParseInt(s.Text(), 10, 0)
		vals = append(vals, int(val))
	}

	fmt.Printf("a: %d\n", partA(vals))
	fmt.Printf("b: %d\n", partB(vals))
}
