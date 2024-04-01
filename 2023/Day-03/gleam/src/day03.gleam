import gleam/int
import gleam/io.{println}
import gleam/list
import gleam/regex
import gleam/result
import gleam/string
import simplifile

pub fn part_a(lines: List(String)) -> Int {
  let len =
    lines
    |> list.at(0)
    |> result.unwrap(or: "")
    |> string.length

  let chars =
    lines
    |> string.join("")
    |> string.split("")

  let symbols = get_symbol_positions(chars, len)
  let numbers = get_number_positions(chars, len)

  let #(adjacent, _) =
    numbers
    |> list.partition(fn(num) {
      let num_y = num.1
      list.range(num.0, num.0 + num.2 - 1)
      |> list.any(fn(num_x) {
        symbols
        |> list.any(fn(sym) {
          let #(sym_x, sym_y) = sym
          case
            [num_x - sym_x, num_y - sym_y]
            |> list.map(int.absolute_value)
          {
            [a, b] if a <= 1 && b <= 1 -> True
            _ -> False
          }
        })
      })
    })

  adjacent
  |> list.map(fn(num) {
    let #(x, y, n) = num
    list.range(0, n - 1)
    |> list.filter_map(fn(i) { list.at(chars, y * len + x + i) })
    |> string.join("")
  })
  |> list.filter_map(int.parse)
  |> int.sum
}

fn get_symbol_positions(chars, line_length) {
  let opts = regex.Options(case_insensitive: False, multi_line: False)
  let assert Ok(symbol) = regex.compile("[^0-9.]", opts)

  chars
  |> list.index_fold([], fn(acc, c, i) {
    let pos = #(i % line_length, i / line_length)
    case regex.check(symbol, c) {
      True -> [pos, ..acc]
      False -> acc
    }
  })
}

fn get_number_positions(chars, line_length) {
  let opts = regex.Options(case_insensitive: False, multi_line: False)
  let assert Ok(digit) = regex.compile("[0-9]", opts)

  chars
  |> list.index_fold([], fn(acc, c, i) {
    let pos = #(i % line_length, i / line_length)
    case regex.check(digit, c) {
      True -> [pos, ..acc]
      False -> acc
    }
  })
  |> list.reverse
  |> list.fold([], fn(acc: List(#(Int, Int, Int)), t: #(Int, Int)) {
    case acc {
      [] -> [#(t.0, t.1, 1)]
      [f, ..rest] ->
        case f.0 + f.2 == t.0, f.1 == t.1 {
          True, True -> [#(f.0, f.1, f.2 + 1), ..rest]
          _, _ -> [#(t.0, t.1, 1), ..acc]
        }
    }
  })
}

pub fn part_b(lines: List(String)) -> Int {
  let len =
    lines
    |> list.at(0)
    |> result.unwrap(or: "")
    |> string.length

  let chars =
    lines
    |> string.join("")
    |> string.split("")

  let stars = get_star_positions(chars, len)
  let numbers = get_number_positions(chars, len)

  stars
  |> list.filter_map(fn(star) {
    let #(star_x, star_y) = star

    let adjacent =
      numbers
      |> list.filter(fn(num) {
        let num_y = num.1
        list.range(num.0, num.0 + num.2 - 1)
        |> list.any(fn(num_x) {
          case
            [num_x - star_x, num_y - star_y]
            |> list.map(int.absolute_value)
          {
            [a, b] if a <= 1 && b <= 1 -> True
            _ -> False
          }
        })
      })

    case list.length(adjacent) {
      2 -> Ok(adjacent)
      _ -> Error(Nil)
    }
  })
  |> list.map(fn(pairs) {
    pairs
    |> list.map(fn(num) {
      let #(x, y, n) = num
      list.range(0, n - 1)
      |> list.filter_map(fn(i) { list.at(chars, y * len + x + i) })
      |> string.join("")
    })
    |> list.filter_map(int.parse)
    |> int.product
  })
  |> int.sum
}

fn get_star_positions(chars, line_length) {
  chars
  |> list.index_fold([], fn(acc, c, i) {
    let pos = #(i % line_length, i / line_length)
    case c {
      "*" -> [pos, ..acc]
      _ -> acc
    }
  })
}

pub fn parse(s: String) -> List(String) {
  s
  |> string.trim
  |> string.split("\n")
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
