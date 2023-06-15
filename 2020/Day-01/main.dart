import "dart:convert";
import "dart:io";

int partA(List<int> vals) {
  for (var i in vals.sublist(0, vals.length - 1)) {
    for (var j in vals.sublist(1)) {
      if (i + j == 2020) {
        return i * j;
      }
    }
  }
  return 0;
}

int partB(List<int> vals) {
  for (var i in vals.sublist(0, vals.length - 2)) {
    for (var j in vals.sublist(1, vals.length - 1)) {
      for (var k in vals.sublist(2)) {
        if (i + j + k == 2020) {
          return i * j * k;
        }
      }
    }
  }
  return 0;
}

Future<void> main(List<String> args) async {
  var vals = await File("input")
      .openRead()
      .map(utf8.decode)
      .transform(LineSplitter())
      .map(int.parse)
      .toList();

  print("a: ${partA(vals)}");
  print("b: ${partB(vals)}");
}
