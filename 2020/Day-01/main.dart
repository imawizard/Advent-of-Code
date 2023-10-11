import "dart:convert";
import "dart:io";

void part1(List<int> vals) {
  outer:
  for (var i in vals.sublist(0, vals.length - 1)) {
    for (var j in vals.sublist(1)) {
      if (i + j == 2020) {
        print(i * j);
        break outer;
      }
    }
  }
}

void part2(List<int> vals) {
  outer:
  for (var i in vals.sublist(0, vals.length - 2)) {
    for (var j in vals.sublist(1, vals.length - 1)) {
      for (var k in vals.sublist(2)) {
        if (i + j + k == 2020) {
          print(i * j * k);
          break outer;
        }
      }
    }
  }
}

Future<void> main(List<String> args) async {
  var vals = await File("../input01.txt")
      .openRead()
      .map(utf8.decode)
      .transform(LineSplitter())
      .map(int.parse)
      .toList();
  part1(vals);
  part2(vals);
}
