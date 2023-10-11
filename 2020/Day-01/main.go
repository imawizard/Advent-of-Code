package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func day01Part1() {
	f, _ := os.Open("../input01.txt")
	defer f.Close()
	s := bufio.NewScanner(f)
	var vals []int
	for s.Scan() {
		val, _ := strconv.ParseInt(s.Text(), 10, 0)
		vals = append(vals, int(val))
	}

outer:
	for i := 0; i < len(vals)-1; i++ {
		for j := 1; j < len(vals); j++ {
			if vals[i]+vals[j] == 2020 {
				fmt.Println(vals[i] * vals[j])
				break outer
			}
		}
	}
}

func day01Part2() {
	f, _ := os.Open("../input01.txt")
	defer f.Close()
	s := bufio.NewScanner(f)
	var vals []int
	for s.Scan() {
		val, _ := strconv.ParseInt(s.Text(), 10, 0)
		vals = append(vals, int(val))
	}

outer:
	for i := 0; i < len(vals)-2; i++ {
		for j := 1; j < len(vals)-1; j++ {
			for k := 2; k < len(vals); k++ {
				if vals[i]+vals[j]+vals[k] == 2020 {
					fmt.Println(vals[i] * vals[j] * vals[k])
					break outer
				}
			}
		}
	}
}
