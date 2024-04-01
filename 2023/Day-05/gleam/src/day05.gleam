import gleam/dict.{type Dict}
import gleam/int
import gleam/io.{println}
import gleam/iterator
import gleam/list
import gleam/regex
import gleam/result
import gleam/string
import simplifile

pub fn part_a(seeds, converters) -> Int {
  [
    "seed-to-soil", "soil-to-fertilizer", "fertilizer-to-water",
    "water-to-light", "light-to-temperature", "temperature-to-humidity",
    "humidity-to-location",
  ]
  |> list.filter_map(dict.get(converters, _))
  |> list.fold(seeds, fn(acc, f) {
    acc
    |> list.map(f)
    |> list.map(result.unwrap_both)
  })
  |> list.reduce(int.min)
  |> result.unwrap(or: 0)
}

pub fn parse_a(
  s: String,
) -> #(List(Int), Dict(String, fn(Int) -> Result(Int, Int))) {
  let assert Ok(re) = regex.from_string("\n(?=\\d)")

  let nums =
    s
    |> string.trim
    |> string.replace("\n\n", "\n")
    |> regex.split(re, _)
    |> string.join(" ")
    |> string.split("\n")
    |> list.filter_map(string.split_once(_, ":"))
    |> list.map(fn(t) {
      #(
        t.0,
        t.1
          |> string.split(" ")
          |> list.filter_map(int.parse),
      )
    })
    |> dict.from_list

  let assert Ok(seeds) = dict.get(nums, "seeds")

  let converters =
    nums
    |> dict.delete("seeds")
    |> dict.to_list
    |> list.map(fn(t) {
      let assert Ok(ident) =
        t.0
        |> string.split(" map")
        |> list.at(0)

      let fns =
        t.1
        |> list.sized_chunk(3)
        |> list.map(fn(chunk) {
          let assert [dest, src, len] = chunk
          fn(num) {
            case src <= num && num <= src + len - 1 {
              True -> Ok(num - src + dest)
              False -> Error(num)
            }
          }
        })

      let convert = fn(num) {
        fns
        |> list.fold(Error(num), result.try_recover)
      }

      #(ident, convert)
    })
    |> dict.from_list

  #(seeds, converters)
}

pub fn part_b_with_parse_a(seeds, converters) -> Int {
  let fns =
    [
      "seed-to-soil", "soil-to-fertilizer", "fertilizer-to-water",
      "water-to-light", "light-to-temperature", "temperature-to-humidity",
      "humidity-to-location",
    ]
    |> list.filter_map(dict.get(converters, _))

  seeds
  |> list.sized_chunk(2)
  |> list.map(fn(r) {
    let assert [start, len] = r
    iterator.range(start, start + len - 1)
    |> iterator.map(fn(seed) {
      fns
      |> list.fold(seed, fn(acc, f) { result.unwrap_both(f(acc)) })
    })
    |> iterator.reduce(int.min)
    |> result.unwrap(or: 0)
  })
  |> list.reduce(int.min)
  |> result.unwrap(or: 0)
}

pub fn part_b(seeds, converters) -> Int {
  let converters =
    [
      "seed-to-soil", "soil-to-fertilizer", "fertilizer-to-water",
      "water-to-light", "light-to-temperature", "temperature-to-humidity",
      "humidity-to-location",
    ]
    |> list.filter_map(dict.get(converters, _))

  seeds
  |> list.map(fn(range) {
    converters
    |> list.fold([range], fn(acc, mappings) { translate(acc, mappings) })
    |> list.map(fn(range) { range.start })
    |> list.reduce(int.min)
    |> result.unwrap(or: range.start)
  })
  |> list.reduce(int.min)
  |> result.unwrap(or: 0)
}

pub type Range {
  Range(start: Int, end: Int)
}

type Overlap {
  Full
  Start
  End
  Middle
}

fn is_overlapping(a: Range, b: Range) -> Result(Overlap, Nil) {
  case True {
    _ if b.start <= a.start && a.end <= b.end -> Ok(Full)
    _ if a.start < b.start && b.end < a.end -> Ok(Middle)
    _ if b.start <= a.start && a.start <= b.end -> Ok(Start)
    _ if b.end >= a.end && a.end >= b.start -> Ok(End)
    _ -> Error(Nil)
  }
}

pub fn translate(
  ranges: List(Range),
  mappings: List(#(Range, Int)),
) -> List(Range) {
  ranges
  |> list.flat_map(fn(a) {
    mappings
    |> list.fold(#([], [a]), fn(acc, mapping) {
      let #(mapped, unmapped) = acc

      unmapped
      |> list.map(fn(a) {
        let #(b, dest) = mapping
        case is_overlapping(a, b) {
          Ok(Full) -> {
            let shifted = dest + a.start - b.start
            let len = a.end - a.start
            #([Range(shifted, shifted + len)], [])
          }
          Ok(Start) -> {
            let shifted = dest + a.start - b.start
            let len = b.end - a.start
            #([Range(shifted, shifted + len)], [Range(b.end + 1, a.end)])
          }
          Ok(End) -> {
            let shifted = dest
            let len = a.end - b.start
            #([Range(shifted, shifted + len)], [Range(a.start, b.start - 1)])
          }
          Ok(Middle) -> {
            #([Range(dest, dest + b.end - b.start)], [
              Range(a.start, b.start - 1),
              Range(b.end + 1, a.end),
            ])
          }
          Error(_) -> #([], [a])
        }
      })
      |> list.fold(#([], []), fn(acc, t) {
        #(list.concat([acc.0, t.0]), list.concat([acc.1, t.1]))
      })
      |> fn(t: #(List(Range), List(Range))) {
        #(list.concat([mapped, t.0]), t.1)
      }
    })
    |> fn(t: #(List(Range), List(Range))) { list.concat([t.0, t.1]) }
  })
  |> list.sort(fn(a, b) { int.compare(a.start, b.start) })
}

pub fn parse_b(s: String) -> #(List(Range), Dict(String, List(#(Range, Int)))) {
  let assert Ok(re) = regex.from_string("\n(?=\\d)")

  let nums =
    s
    |> string.trim
    |> string.replace("\n\n", "\n")
    |> regex.split(re, _)
    |> string.join(" ")
    |> string.split("\n")
    |> list.filter_map(string.split_once(_, ":"))
    |> list.map(fn(t) {
      #(
        t.0,
        t.1
          |> string.split(" ")
          |> list.filter_map(int.parse),
      )
    })
    |> dict.from_list

  let assert Ok(seeds) = dict.get(nums, "seeds")
  let seeds =
    seeds
    |> list.sized_chunk(2)
    |> list.map(fn(seed) {
      let assert [start, len] = seed
      Range(start, start + len - 1)
    })

  let converters =
    nums
    |> dict.delete("seeds")
    |> dict.to_list
    |> list.map(fn(t) {
      let assert Ok(ident) =
        t.0
        |> string.split(" map")
        |> list.at(0)

      let mappings =
        t.1
        |> list.sized_chunk(3)
        |> list.map(fn(chunk) {
          let assert [dest, src, len] = chunk
          #(Range(src, src + len - 1), dest)
        })

      #(ident, mappings)
    })
    |> dict.from_list

  #(seeds, converters)
}

pub fn main() {
  use input <- result.map(simplifile.read("../input"))

  let #(seeds, converters) = parse_a(input)
  part_a(seeds, converters)
  |> int.to_string
  |> fn(res) { "a: " <> res }
  |> println

  let #(seeds, converters) = parse_b(input)
  part_b(seeds, converters)
  |> int.to_string
  |> fn(res) { "b: " <> res }
  |> println
}
