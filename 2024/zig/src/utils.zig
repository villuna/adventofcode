const std = @import("std");
const ArrayList = std.ArrayList;
const File = std.fs.File;
const expect = std.testing.expect;

pub fn HashSet(comptime K: type) type {
    return std.AutoHashMap(K, void);
}

pub const Vec2 = struct {
    x: isize,
    y: isize,

    fn eq(self: *const Vec2, other: *const Vec2) bool {
        return self.x == other.x and self.y == other.y;
    }
};

pub const Grid = struct {
    map: ArrayList(ArrayList(u8)),

    pub fn width(self: *const Grid) usize {
        return self.map.items[0].items.len;
    }

    pub fn height(self: *const Grid) usize {
        return self.map.items.len;
    }

    pub fn isInBounds(self: *const Grid, pos: Vec2) bool {
        return pos.y >= 0 and pos.y < self.height() and pos.x >= 0 and pos.x < self.width();
    }

    // Returns the char at position `pos` if it exists and null otherwise
    pub fn charAt(self: *const Grid, pos: Vec2) ?u8 {
        if (self.isInBounds(pos)) {
            return self.map.items[@intCast(pos.y)].items[@intCast(pos.x)];
        } else {
            return null;
        }
    }

    pub fn parse(file: *File, alloc: std.mem.Allocator) !Grid {
        var res = ArrayList(ArrayList(u8)).init(alloc);
        var reader = file.reader();
        var y: isize = 0;

        while (true) {
            var line = ArrayList(u8).init(alloc);

            reader.streamUntilDelimiter(line.writer(), '\n', null) catch |err| {
                try expect(err == error.EndOfStream);
                break;
            };

            try res.append(line);
            y += 1;
        }

        return Grid{
            .map = res,
        };
    }
};

pub fn vec2(x: isize, y: isize) Vec2 {
    return Vec2{ .x = x, .y = y };
}

pub fn openDayFile(day: usize, alloc: std.mem.Allocator) !File {
    // Create a string to hold the filename and write the formatted path to it
    var path = ArrayList(u8).init(alloc);
    defer path.deinit();
    _ = try std.fmt.format(path.writer(), "../input/day{d}.txt", .{day});

    return std.fs.cwd().openFile(path.items, .{});
}

pub fn dayFileLines(day: usize, alloc: std.mem.Allocator) !ArrayList(ArrayList(u8)) {
    var file = try openDayFile(day, alloc);
    defer file.close();
    var reader = file.reader();
    var result = ArrayList(ArrayList(u8)).init(alloc);

    while (true) {
        var line = ArrayList(u8).init(alloc);
        const writer = line.writer();

        reader.streamUntilDelimiter(writer, '\n', null) catch |err| {
            try expect(err == error.EndOfStream);
            break;
        };

        try result.append(line);
    }

    return result;
}

pub fn find(haystack: []const u8, needle: []const u8) ?usize {
    if (needle.len > haystack.len)
        return null;

    var i: usize = 0;
    while (i < haystack.len - needle.len + 1) : (i += 1) {
        const found = haystack[i .. i + needle.len];

        var all = true;
        for (found, 0..) |char, j| {
            if (char != needle[j]) {
                all = false;
                break;
            }
        }

        if (all)
            return i;
    }

    return null;
}

pub fn findLast(haystack: []const u8, needle: []const u8) ?usize {
    if (needle.len > haystack.len)
        return null;

    var i: usize = haystack.len - needle.len;
    while (true) {
        const found = haystack[i .. i + needle.len];

        var all = true;
        for (found, 0..) |char, j| {
            if (char != needle[j]) {
                all = false;
                break;
            }
        }

        if (all)
            return i;

        if (i > 0) {
            i -= 1;
        } else {
            break;
        }
    }

    return null;
}

pub fn abs(x: anytype) @TypeOf(x) {
    if (x >= 0) {
        return x;
    } else {
        return -x;
    }
}
