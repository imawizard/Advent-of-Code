import gleam/dict
import gleam/float
import gleam/int
import gleam/io.{println}
import gleam/list
import gleam/option
import gleam/result
import gleam/set
import gleam/string
import simplifile

pub fn part_a(input) {
  input
  |> list.map(fn(t) {
    let #(want, got) = t
    let want = set.from_list(want)
    got
    |> list.filter(set.contains(want, _))
  })
  |> list.map(list.length)
  |> list.filter(fn(n) { n > 0 })
  |> list.filter_map(fn(n) { int.power(2, int.to_float(n - 1)) })
  |> list.map(float.truncate)
  |> int.sum
}

pub fn part_b(input) {
  input
  |> list.map(fn(t) {
    let #(want, got) = t
    let want = set.from_list(want)
    list.filter(got, set.contains(want, _))
  })
  |> list.map(list.length)
  |> list.zip(list.range(0, list.length(input) - 1), _)
  |> list.fold(dict.new(), fn(acc, t) {
    let #(i, n) = t

    let already =
      dict.get(acc, i)
      |> result.unwrap(or: 0)

    let inc = fn(i) { fn(v) { option.unwrap(v, 0) + i } }

    case n {
      0 -> dict.update(acc, i, inc(1))
      _ ->
        list.range(i + 1, i + n)
        |> list.fold(acc, fn(acc, idx) {
          dict.update(acc, idx, inc(1 + already))
        })
        |> dict.update(i, inc(1))
    }
  })
  |> dict.to_list
  |> list.map(fn(t) { t.1 })
  |> int.sum
}

pub fn parse(s: String) -> List(#(List(Int), List(Int))) {
  s
  |> string.split("\n")
  |> list.map(string.trim)
  |> list.filter(fn(line) {
    !string.is_empty(line)
    && string.contains(line, ":")
    && string.contains(line, "|")
  })
  |> list.map(fn(line) {
    line
    |> string.split(":")
    |> list.at(1)
    |> result.unwrap(or: "")
    |> string.split("|")
    |> list.map(fn(nums) {
      nums
      |> string.trim
      |> string.split(" ")
      |> list.filter(fn(s) { !string.is_empty(s) })
      |> list.filter_map(int.parse)
    })
  })
  |> list.filter_map(fn(l) {
    case l {
      [w, g] -> Ok(#(w, g))
      _ -> Error(Nil)
    }
  })
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

  input
  |> part_b
  |> int.to_string
  |> fn(res) { "b: " <> res }
  |> println
}
