const std = @import("std");
const utils = @import("utils.zig");

const Grid = utils.Grid;
const HashSet = utils.HashSet;
const Vec2 = utils.Vec2;
const vec2 = utils.vec2;

fn findNextStart(grid: *const Grid, visited: *const HashSet(Vec2)) ?Vec2 {
    for (0..grid.width()) |x| {
        for (0..grid.height()) |y| {
            if (!visited.contains(vec2(@intCast(x), @intCast(y)))) {
                return vec2(@intCast(x), @intCast(y));
            }
        }
    }

    return null;
}

fn floodfill(grid: *const Grid, start: Vec2, area: *HashSet(Vec2), visited: *HashSet(Vec2)) !void {
    try area.put(start, {});
    try visited.put(start, {});

    for ([_]Vec2{ vec2(0, -1), vec2(0, 1), vec2(-1, 0), vec2(1, 0) }) |d| {
        const next = vec2(start.x + d.x, start.y + d.y);

        if (grid.isInBounds(next) and grid.charAt(next) == grid.charAt(start) and !area.contains(next)) {
            try floodfill(grid, next, area, visited);
        }
    }
}

fn findAreas(grid: *const Grid, alloc: std.mem.Allocator) !std.ArrayList(HashSet(Vec2)) {
    var res = std.ArrayList(HashSet(Vec2)).init(alloc);
    var visited = HashSet(Vec2).init(alloc);

    while (true) {
        const start = findNextStart(grid, &visited) orelse break;
        var area = HashSet(Vec2).init(alloc);
        try floodfill(grid, start, &area, &visited);
        try res.append(area);
    }

    return res;
}

fn part1(areas: *const std.ArrayList(HashSet(Vec2))) usize {
    var res: usize = 0;

    for (areas.items) |a| {
        var cost: usize = 0;
        var iter = a.keyIterator();

        while (iter.next()) |v| {
            for ([_]Vec2{ vec2(0, -1), vec2(0, 1), vec2(-1, 0), vec2(1, 0) }) |d| {
                const next = vec2(v.x + d.x, v.y + d.y);
                if (!a.contains(next)) {
                    cost += 1;
                }
            }
        }

        res += cost * a.count();
    }

    return res;
}

fn part2(areas: *const std.ArrayList(HashSet(Vec2)), alloc: std.mem.Allocator) !usize {
    var res: usize = 0;

    // I'm representing sides as vec2s in this way:
    // If x is positive the side is vertical, bordering that x valued cell.
    // y = -1 means on the left side of the block and -2 means the right side.
    //
    // Analogous for y, but if x = -1 it means the top and x = -2 means the bottom.
    for (areas.items) |a| {
        var sides = std.AutoHashMap(Vec2, HashSet(Vec2)).init(alloc);
        var cost: usize = 0;
        var iter = a.keyIterator();

        while (iter.next()) |v| {
            for ([_]Vec2{ vec2(0, -1), vec2(0, 1), vec2(-1, 0), vec2(1, 0) }) |d| {
                const next = vec2(v.x + d.x, v.y + d.y);

                if (!a.contains(next)) {
                    var side = vec2(0, 0);

                    if (d.x == 0) {
                        if (d.y == -1) {
                            side = vec2(-1, v.y);
                        } else {
                            side = vec2(-2, v.y);
                        }
                    } else {
                        if (d.x == -1) {
                            side = vec2(v.x, -1);
                        } else {
                            side = vec2(v.x, -2);
                        }
                    }

                    if (!sides.contains(side) or !sides.getPtr(side).?.contains(v.*)) {
                        if (!sides.contains(side)) {
                            try sides.put(side, HashSet(Vec2).init(alloc));
                        }

                        cost += 1;

                        try sides.getPtr(side).?.put(v.*, {});

                        for ([_]Vec2{ vec2(d.y * -1, d.x * -1), vec2(d.y, d.x) }) |d2| {
                            var meow = vec2(v.x + d2.x, v.y + d2.y);

                            while (a.contains(meow) and !a.contains(vec2(meow.x + d.x, meow.y + d.y))) {
                                try sides.getPtr(side).?.put(meow, {});
                                meow = vec2(meow.x + d2.x, meow.y + d2.y);
                            }
                        }
                    }
                }
            }
        }

        res += cost * a.count();
    }

    return res;
}

pub fn day12() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const alloc = arena.allocator();

    var file = try utils.openDayFile(12, alloc);
    const grid = try Grid.parse(&file, alloc);

    const areas = try findAreas(&grid, alloc);

    const p1 = part1(&areas);
    std.debug.print("Part 1: {d}\n", .{p1});

    const p2 = try part2(&areas, alloc);
    std.debug.print("Part 2: {d}\n", .{p2});
}
