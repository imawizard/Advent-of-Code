import gleam/dict
import gleam/int
import gleam/io.{println}
import gleam/list
import gleam/result
import gleam/string
import simplifile

pub fn part_a(input: List(#(Int, Int))) -> Int {
  input
  |> list.map(fn(t) {
    let #(time, dist) = t

    list.range(1, time - 1)
    |> list.map(fn(i) { { time - i } * i })
    |> list.filter(fn(d) { d > dist })
    |> list.length
  })
  |> int.product
}

pub fn parse_a(s: String) -> List(#(Int, Int)) {
  let vals =
    s
    |> string.trim
    |> string.split("\n")
    |> list.filter_map(string.split_once(_, ":"))
    |> list.map(fn(t) {
      #(
        t.0
          |> string.lowercase,
        t.1
          |> string.split(" ")
          |> list.filter_map(int.parse),
      )
    })
    |> dict.from_list

  let assert Ok(time) = dict.get(vals, "time")
  let assert Ok(dist) = dict.get(vals, "distance")

  list.zip(time, dist)
}

pub fn part_b(input: #(Int, Int)) -> Int {
  part_a([input])
}

pub fn parse_b(s: String) -> #(Int, Int) {
  s
  |> string.replace(" ", "")
  |> parse_a
  |> list.first
  |> result.unwrap(#(0, 0))
}

pub fn main() {
  use input <- result.map(simplifile.read("../input"))

  input
  |> parse_a
  |> part_a
  |> int.to_string
  |> fn(res) { "a: " <> res }
  |> println

  input
  |> parse_b
  |> part_b
  |> int.to_string
  |> fn(res) { "b: " <> res }
  |> println
}
