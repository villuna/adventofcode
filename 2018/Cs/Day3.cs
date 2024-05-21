namespace AoCs;

using Framework;
using Sprache;
using System.Diagnostics;

record struct Rect(int id, int x, int y, int w, int h) {
    public static Parser<Rect> Parser =
        from _hash in Parse.Char('#')
        from id in Parse.Digit.Many().Text()
        from _sep in Parse.String("@").Token()
        from x in Parse.Digit.Many().Text()
        from _comma in Parse.Char(',')
        from y in Parse.Digit.Many().Text()
        from _sep2 in Parse.String(":").Token()
        from w in Parse.Digit.Many().Text()
        from _x in Parse.Char('x')
        from h in Parse.Digit.Many().Text()
        select new Rect(int.Parse(id), int.Parse(x), int.Parse(y), int.Parse(w), int.Parse(h));
}

record struct Point(int x, int y);

public class Day3 : IDay {
    public void Solve(string input) {
        List<Rect> rects = input.Trim().Split("\n").Select(Rect.Parser.Parse).ToList();
        Dictionary<Point, int> overlapMap = new();

        foreach (Rect rect in rects) {
            for (int x = rect.x; x < rect.x + rect.w; x++) {
                for (int y = rect.y; y < rect.y + rect.h; y++) {
                    Point p = new Point(x, y);
                    if (overlapMap.ContainsKey(p)) {
                        overlapMap[p] += 1;
                    } else {
                        overlapMap[p] = 1;
                    }
                }
            }
        }

        var overlaps = 
            from kvp in overlapMap
            where kvp.Value > 1
            select kvp;
        
        Console.WriteLine($"Part 1: {overlaps.Count()}");

        foreach (Rect rect in rects) {
            for (int x = rect.x; x < rect.x + rect.w; x++) {
                for (int y = rect.y; y < rect.y + rect.h; y++) {
                    if (overlapMap[new Point(x, y)] != 1) {
                        // yes yes I know
                        // but i want to continue a nested loop and dont want another variable
                        // it's just advent of code, its *fine*
                        goto Continue;
                    }
                }
            }

            Console.WriteLine($"Part 2: {rect.id}");
            return;

        Continue:
            ;
        }
    }
}