const std = @import("std");
const day10 = @import("day10.zig").day10;
const day12 = @import("day12.zig").day12;

const days = [_]struct { day: u8, func: *const fn () anyerror!void }{
    .{ .day = 10, .func = day10 },
    .{ .day = 12, .func = day12 },
};

pub fn main() !void {
    var args = std.process.args();
    _ = args.next() orelse {
        std.debug.print("usage: aoc23zig [day]\n", .{});
        return;
    };

    const day = args.next() orelse {
        std.debug.print("usage: aoc23zig [day]\n", .{});
        return;
    };

    const day_num = std.fmt.parseInt(u8, day, 10) catch |err| {
        _ = err catch {};

        std.debug.print("invalid day number: \"{s}\"\n", .{day});
        return;
    };

    for (days) |pair| {
        if (pair.day == day_num) {
            try pair.func();
            return;
        }
    }

    std.debug.print("day invalid or not completed\n", .{});
}
