import gleam/dict
import gleam/int
import gleam/io.{println}
import gleam/list
import gleam/result
import gleam/string
import simplifile

pub fn part_a(input: List(Instruction)) -> Int {
  let x =
    input
    |> list.scan([#(0, 1, 0)], fn(acc, instr) {
      let assert Ok(#(cycle, x, d)) = list.last(acc)
      case instr {
        Addx(v) -> [#(cycle + 1, x + d, 0), #(cycle + 2, x + d, v)]
        Noop -> [#(cycle + 1, x + d, 0)]
      }
    })
    |> list.flatten
    |> list.map(fn(t) { #(t.0, t.1) })
    |> dict.from_list

  list.range(0, 5)
  |> list.map(fn(n) { n * 40 + 20 })
  |> list.map(fn(idx) { #(idx, dict.get(x, idx)) })
  |> list.map(fn(t) { #(t.0, result.unwrap(t.1, 0)) })
  |> list.map(fn(t) { t.0 * t.1 })
  |> int.sum
}

pub fn part_b(input: List(Instruction)) -> List(String) {
  input
  |> list.scan([#(0, 1, 0)], fn(acc, instr) {
    let assert Ok(#(cycle, x, d)) = list.last(acc)
    case instr {
      Addx(v) -> [#(cycle + 1, x + d, 0), #(cycle + 2, x + d, v)]
      Noop -> [#(cycle + 1, x + d, 0)]
    }
  })
  |> list.flatten
  |> list.map(fn(t) {
    case int.absolute_value({ t.0 - 1 } % 40 - t.1) {
      0 | 1 -> "#"
      _ -> "."
    }
  })
  |> list.sized_chunk(40)
  |> list.map(string.join(_, ""))
}

pub type Instruction {
  Addx(Int)
  Noop
}

fn new_instruction(s: String) -> Result(Instruction, Nil) {
  case
    s
    |> string.trim
    |> string.lowercase
  {
    "addx " <> arg ->
      Ok(Addx(
        arg
        |> int.parse
        |> result.unwrap(or: 0),
      ))
    "noop" -> Ok(Noop)
    _ -> Error(Nil)
  }
}

pub fn parse(s: String) {
  s
  |> string.split("\n")
  |> list.filter(fn(s) { !string.is_empty(s) })
  |> list.filter_map(new_instruction)
}

pub fn main() {
  let input =
    simplifile.read("../input")
    |> result.unwrap(or: "")
    |> parse

  input
  |> part_a
  |> int.to_string
  |> fn(res) { "a: " <> res }
  |> println

  println("b:")
  input
  |> part_b
  |> list.map(println)
}
