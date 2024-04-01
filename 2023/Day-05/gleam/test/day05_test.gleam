import gleam/dict
import gleam/function.{tap}
import gleam/list
import gleam/result
import gleeunit
import gleeunit/should
import day05

const input = "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"

pub fn part_a_test() {
  let #(seeds, converters) =
    input
    |> day05.parse_a

  let assert Ok(seed2soil) = dict.get(converters, "seed-to-soil")
  let assert Ok(soil2fert) = dict.get(converters, "soil-to-fertilizer")
  let assert Ok(fert2water) = dict.get(converters, "fertilizer-to-water")
  let assert Ok(water2light) = dict.get(converters, "water-to-light")
  let assert Ok(light2temp) = dict.get(converters, "light-to-temperature")
  let assert Ok(temp2humidity) = dict.get(converters, "temperature-to-humidity")
  let assert Ok(humidity2loc) = dict.get(converters, "humidity-to-location")

  let unwrap = fn(f) { fn(x) { result.unwrap_both(f(x)) } }

  should.equal(seed2soil(0), Error(0))
  should.equal(seed2soil(1), Error(1))
  should.equal(seed2soil(49), Error(49))
  should.equal(seed2soil(50), Ok(52))
  should.equal(seed2soil(97), Ok(99))
  should.equal(seed2soil(98), Ok(50))
  should.equal(seed2soil(99), Ok(51))

  seeds
  |> list.map(unwrap(seed2soil))
  |> tap(should.equal(_, [81, 14, 57, 13]))
  |> list.map(unwrap(soil2fert))
  |> tap(should.equal(_, [81, 53, 57, 52]))
  |> list.map(unwrap(fert2water))
  |> tap(should.equal(_, [81, 49, 53, 41]))
  |> list.map(unwrap(water2light))
  |> tap(should.equal(_, [74, 42, 46, 34]))
  |> list.map(unwrap(light2temp))
  |> tap(should.equal(_, [78, 42, 82, 34]))
  |> list.map(unwrap(temp2humidity))
  |> tap(should.equal(_, [78, 43, 82, 35]))
  |> list.map(unwrap(humidity2loc))
  |> tap(should.equal(_, [82, 43, 86, 35]))

  day05.part_a(seeds, converters)
  |> should.equal(35)
}

pub fn part_b_test() {
  let #(seeds, converters) =
    input
    |> day05.parse_b

  let assert Ok(seed2soil) = dict.get(converters, "seed-to-soil")
  let assert Ok(soil2fert) = dict.get(converters, "soil-to-fertilizer")

  [day05.Range(0, 99)]
  |> day05.translate(seed2soil)
  |> tap(should.equal(_, [
    day05.Range(0, 49),
    day05.Range(50, 51),
    day05.Range(52, 99),
  ]))
  |> day05.translate(soil2fert)
  |> tap(should.equal(_, [
    day05.Range(0, 34),
    day05.Range(35, 36),
    day05.Range(37, 38),
    day05.Range(39, 53),
    day05.Range(54, 99),
  ]))

  day05.part_b(seeds, converters)
  |> should.equal(46)
}

pub fn main() {
  gleeunit.main()
}
