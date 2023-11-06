const std = @import("std");
const ascii = @import("std").ascii;
const fmt = @import("std").fmt;
const heap = @import("std").heap;
const mem = @import("std").mem;
const testing = @import("std").testing;

const ArrayList = @import("std").ArrayList;
const AutoHashMap = @import("std").AutoHashMap;

fn part_a(a: mem.Allocator, input: []Move) !u32 {
    var m = AutoHashMap(Point, void).init(a);
    defer m.deinit();

    var head = Point{};
    var tail = Point{};

    for (input) |move| {
        for (0..move.steps) |_| {
            switch (move.direction) {
                .up => head.y += 1,
                .down => head.y -= 1,
                .left => head.x -= 1,
                .right => head.x += 1,
            }
            if (@abs(head.x - tail.x) > 1 or @abs(head.y - tail.y) > 1) {
                switch (move.direction) {
                    .up => tail.y += 1,
                    .down => tail.y -= 1,
                    .left => tail.x -= 1,
                    .right => tail.x += 1,
                }

                switch (move.direction) {
                    .up, .down => tail.x = head.x,
                    .left, .right => tail.y = head.y,
                }
            }
            _ = try m.getOrPut(tail);
        }
    }
    return m.count();
}

fn part_b(a: mem.Allocator, input: []Move, comptime length: u32) !u32 {
    comptime {
        if (length < 2) {
            @compileError("length must be greater than two");
        }
    }

    var m = AutoHashMap(Point, void).init(a);
    defer m.deinit();

    var points = [_]Point{.{}} ** length;

    for (input) |move| {
        for (0..move.steps) |_| {
            var dir = move.direction;

            for (1..points.len) |idx| {
                var head = &points[idx - 1];
                var tail = &points[idx];

                if (idx == 1) {
                    switch (dir) {
                        .up => head.y += 1,
                        .down => head.y -= 1,
                        .left => head.x -= 1,
                        .right => head.x += 1,
                    }
                }

                const x_diff = head.x - tail.x;
                const y_diff = head.y - tail.y;
                if (@abs(x_diff) > 1) {
                    if (x_diff > 0) {
                        tail.x += 1;
                    } else if (x_diff < 0) {
                        tail.x -= 1;
                    }
                    if (@abs(y_diff) > 0) {
                        if (y_diff > 0) {
                            tail.y += 1;
                        } else if (y_diff < 0) {
                            tail.y -= 1;
                        }
                    }
                } else if (@abs(y_diff) > 1) {
                    if (y_diff > 0) {
                        tail.y += 1;
                    } else if (y_diff < 0) {
                        tail.y -= 1;
                    }
                    if (@abs(x_diff) > 0) {
                        if (x_diff > 0) {
                            tail.x += 1;
                        } else if (x_diff < 0) {
                            tail.x -= 1;
                        }
                    }
                }

                if (idx == points.len - 1) {
                    _ = try m.getOrPut(tail.*);
                }
            }
        }
    }

    return m.count();
}

fn print_points(a: mem.Allocator, points: []Point) ![]u8 {
    var min_x: i32 = 0;
    var max_x: i32 = 0;
    var min_y: i32 = 0;
    var max_y: i32 = 0;

    for (points) |point| {
        if (point.x < min_x) {
            min_x = point.x;
        } else if (max_x < point.x) {
            max_x = point.x;
        }
        if (point.y < min_y) {
            min_y = point.y;
        } else if (max_y < point.y) {
            max_y = point.y;
        }
    }

    const width = @abs(max_x - min_x) + 1;
    const height = @abs(max_y - min_y) + 1;
    const off_x: usize = if (min_x < 0) @abs(min_x) else 0;
    const off_y: usize = @abs(max_y);

    var buf = try a.alloc(u8, width * height);
    defer a.free(buf);
    @memset(buf, '.');
    buf[off_y * width + off_x] = 's';

    for (points, 0..) |point, i| {
        const x = @abs(point.x + @as(i32, @intCast(off_x)));
        const y = if (point.y < 0) @abs(point.y) + off_y else off_y - @abs(point.y);
        var p = &buf[y * width + x];
        if (p.* == '.' or p.* == 's') {
            if (points.len < 10) {
                buf[y * width + x] = if (i == 0) 'H' else '0' + @as(u8, @truncate(i));
            } else {
                buf[y * width + x] = '#';
            }
        }
    }

    var res = try a.alloc(u8, width * height + height + 1);
    res[res.len - 1] = 0;

    var p = res.ptr;

    for (0..height) |y| {
        for (0..width) |x| {
            p[0] = buf[y * width + x];
            p += 1;
        }
        p[0] = '\n';
        p += 1;
    }
    return res;
}

const Point = struct { x: i32 = 0, y: i32 = 0 };

const Move = struct {
    direction: enum { up, down, left, right },
    steps: u32,
};

fn parse(a: mem.Allocator, s: []const u8) ArrayList(Move) {
    var res = ArrayList(Move).init(a);

    var it = mem.tokenizeSequence(u8, s, "\n");
    while (it.next()) |line| {
        const l = mem.trim(u8, line, &ascii.whitespace);

        res.append(.{
            .direction = switch (ascii.toUpper(l[0])) {
                'U' => .up,
                'D' => .down,
                'L' => .left,
                'R' => .right,
                else => continue,
            },
            .steps = fmt.parseInt(
                u32,
                l[mem.indexOfAny(u8, l, "0123456789") orelse continue ..],
                10,
            ) catch continue,
        }) catch continue;
    }

    return res;
}

pub fn main() !void {
    var a = heap.GeneralPurposeAllocator(.{}){};
    const input = parse(a.allocator(), @embedFile("input"));
    defer input.deinit();

    std.debug.print("a: {}\n", .{try part_a(a.allocator(), input.items)});
    std.debug.print("b: {}\n", .{try part_b(a.allocator(), input.items, 10)});
}

test part_a {
    const input = parse(testing.allocator,
        \\R 4
        \\U 4
        \\L 3
        \\D 1
        \\R 4
        \\D 1
        \\L 5
        \\R 2
    );
    defer input.deinit();

    try testing.expectEqual(@as(u32, 13), try part_a(testing.allocator, input.items));
}

test part_b {
    const input1 = parse(testing.allocator,
        \\R 4
        \\U 4
        \\L 3
        \\D 1
        \\R 4
        \\D 1
        \\L 5
        \\R 2
    );
    const input2 = parse(testing.allocator,
        \\R 5
        \\U 8
        \\L 8
        \\D 3
        \\R 17
        \\D 10
        \\L 25
        \\U 20
    );
    defer input1.deinit();
    defer input2.deinit();

    try testing.expectEqual(@as(u32, 1), try part_b(testing.allocator, input1.items, 10));
    try testing.expectEqual(@as(u32, 36), try part_b(testing.allocator, input2.items, 10));
}
