namespace AoCs;
using Framework;

public class Day1 : IDay {
    public void Solve(string input) {
        // // Insane 1 line solution to part 1
        // // Unfortunately i cant use it bc you can calculate both part 1 and part 2 at the same time
        //
        // Console.WriteLine("Part 1: " + input.Trim().Split().Select(int.Parse).Sum());

        List<int> changes = input.Trim().Split().Select(int.Parse).ToList();
        int frequency = 0;
        int? totalFrequency = null;
        int? firstDuplicate = null;
        HashSet<int> frequencies = [0];

        int i = 0;
        while (firstDuplicate == null || totalFrequency == null) {
            frequency += changes[i];

            if (!frequencies.Add(frequency)) {
                firstDuplicate ??= frequency;
            }

            i++;
            if (i == changes.Count) {
                totalFrequency ??= frequency;
                i = 0;
            }
        }

        Console.WriteLine("Part 1: " + totalFrequency);
        Console.WriteLine("Part 2: " + firstDuplicate);
    }
}