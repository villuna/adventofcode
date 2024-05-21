namespace AoCs;
using Framework;

public class Day2 : IDay {
    static bool HasMatches(string s, int n) {
        IEnumerable<char> matches =
            from c in s
            where s.Count(k => k == c) == n
            select c;
        
        return matches.Any();
    }

    static int HammingDistance(string s1, string s2) {
        int dist = 0;
        for (int i = 0; i < s1.Length; i++) {
            if (s1[i] != s2[i]) {
                dist++;
            }
        }

        return dist;
    }

    public void Solve(string input) {
        IEnumerable<string> ids = input.Trim().Split();
        int twos = ids.Where(s => HasMatches(s, 2)).Count();
        int threes = ids.Where(s => HasMatches(s, 3)).Count();
        Console.WriteLine($"Part 1: {twos * threes}");

        // Every language that doesnt have shadowing is dumb
        List<string> ids_l = ids.ToList();
        for (int i = 0; i < ids_l.Count; i++) {
            for (int j = i + 1; j < ids_l.Count; j++) {
                int dist = HammingDistance(ids_l[i], ids_l[j]);
                if (dist == 1) {
                    string commonChars = string.Concat(ids_l[i].Where(c => ids_l[j].Contains(c)));
                    Console.WriteLine("Part 2: " + commonChars);
                    // Stupid way to break out of a nested for loop
                    return;
                }
            }
        }
    }
}