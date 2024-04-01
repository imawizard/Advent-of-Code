import gleeunit
import gleeunit/should
import day06

const input = "
Time:      7  15   30
Distance:  9  40  200
"

pub fn part_a_test() {
  input
  |> day06.parse_a
  |> day06.part_a
  |> should.equal(288)
}

pub fn part_b_test() {
  input
  |> day06.parse_b
  |> day06.part_b
  |> should.equal(71_503)
}

pub fn main() {
  gleeunit.main()
}
