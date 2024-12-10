const std = @import("std");
const utils = @import("utils.zig");

const ArrayList = std.ArrayList;
const File = std.fs.File;
const expect = std.testing.expect;
const Vec2 = utils.Vec2;
const vec2 = utils.vec2;

const Environment = struct {
    map: ArrayList(ArrayList(u8)),
    starts: ArrayList(Vec2),

    fn isInBounds(self: *const Environment, pos: Vec2) bool {
        return pos.y >= 0 and pos.y < self.map.items.len and pos.x >= 0 and pos.x < self.map.items[@intCast(pos.y)].items.len;
    }

    // Returns the level at position `pos` if it exists and null otherwise
    fn levelAt(self: *const Environment, pos: Vec2) ?u8 {
        if (self.isInBounds(pos)) {
            return self.map.items[@intCast(pos.y)].items[@intCast(pos.x)];
        } else {
            return null;
        }
    }
};

fn HashSet(comptime K: type) type {
    return std.AutoHashMap(K, void);
}

fn parse(file: *File, alloc: std.mem.Allocator) !Environment {
    var res = ArrayList(ArrayList(u8)).init(alloc);
    var reader = file.reader();
    var y: isize = 0;
    var starts = ArrayList(Vec2).init(alloc);

    while (true) {
        var line = ArrayList(u8).init(alloc);

        reader.streamUntilDelimiter(line.writer(), '\n', null) catch |err| {
            try expect(err == error.EndOfStream);
            break;
        };

        for (line.items, 0..) |c, i| {
            line.items[i] = c - '0';

            if (c == '0') {
                try starts.append(Vec2{ .x = @intCast(i), .y = y });
            }
        }

        try res.append(line);
        y += 1;
    }

    return Environment{
        .map = res,
        .starts = starts,
    };
}

fn dfs(env: *const Environment, start: Vec2, visited: *HashSet(Vec2)) !usize {
    var score: usize = 0;
    try visited.put(start, {});

    if (env.levelAt(start).? == 9) {
        score += 1;
    }

    for ([_]Vec2{ vec2(-1, 0), vec2(1, 0), vec2(0, -1), vec2(0, 1) }) |diff| {
        const coord = vec2(diff.x + start.x, diff.y + start.y);
        if (visited.get(coord) == null and env.isInBounds(coord) and @as(i16, env.levelAt(coord).?) - @as(i16, env.levelAt(start).?) == 1) {
            score += try dfs(env, coord, visited);
        }
    }

    return score;
}

fn part1(env: *const Environment, alloc: std.mem.Allocator) !usize {
    var score: usize = 0;

    for (env.starts.items) |start| {
        var visited = HashSet(Vec2).init(alloc);
        score += try dfs(env, start, &visited);
    }

    return score;
}

fn trails_dfs(env: *const Environment, start: Vec2) usize {
    if (env.levelAt(start).? == 9) {
        return 1;
    }

    var trails: usize = 0;

    for ([_]Vec2{ vec2(-1, 0), vec2(1, 0), vec2(0, -1), vec2(0, 1) }) |diff| {
        const coord = vec2(diff.x + start.x, diff.y + start.y);
        if (env.isInBounds(coord) and @as(i16, env.levelAt(coord).?) - @as(i16, env.levelAt(start).?) == 1) {
            trails += trails_dfs(env, coord);
        }
    }

    return trails;
}

fn part2(env: *const Environment) usize {
    var score: usize = 0;

    for (env.starts.items) |start| {
        score += trails_dfs(env, start);
    }

    return score;
}

pub fn day10() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const alloc = arena.allocator();

    var file = try utils.openDayFile(10, alloc);
    const env = try parse(&file, alloc);

    std.debug.print("Part 1: {d}\n", .{try part1(&env, alloc)});
    std.debug.print("Part 2: {d}\n", .{part2(&env)});
}
