import gleeunit
import gleeunit/should
import day03

const input = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"

pub fn part_a_test() {
  input
  |> day03.parse
  |> day03.part_a
  |> should.equal(4361)
}

pub fn part_b_test() {
  input
  |> day03.parse
  |> day03.part_b
  |> should.equal(467_835)
}

pub fn main() {
  gleeunit.main()
}
