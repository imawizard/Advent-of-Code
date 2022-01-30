package main

import (
	"bufio"
	"bytes"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func PartA(input []string) (int, int) {
	pos, depth := 0, 0

	for _, cmd := range input {
		f := strings.Fields(cmd)
		if len(f) < 2 {
			continue
		}

		dir := strings.ToLower(f[0])
		units, _ := strconv.Atoi(f[1])
		switch dir {
		case "forward":
			pos += units
		case "up":
			depth -= units
		case "down":
			depth += units
		}
	}

	return pos, depth
}

func PartB(input []string) (int, int) {
	pos, depth, aim := 0, 0, 0

	for _, cmd := range input {
		f := strings.Fields(cmd)
		if len(f) < 2 {
			continue
		}

		dir := strings.ToLower(f[0])
		units, _ := strconv.Atoi(f[1])
		switch dir {
		case "forward":
			pos += units
			depth += aim * units
		case "up":
			aim -= units
		case "down":
			aim += units
		}
	}

	return pos, depth
}

func main() {
	input, _ := os.ReadFile("input")
	s := bufio.NewScanner(bytes.NewReader(input))

	var commands []string
	for s.Scan() {
		commands = append(commands, s.Text())
	}

	apos, adepth := PartA(commands)
	fmt.Println("a:", apos*adepth)

	bpos, bdepth := PartB(commands)
	fmt.Println("b:", bpos*bdepth)
}
