using AoCs;
using Framework;

if (args.Length != 1) {
    Console.WriteLine("Usage: aocs [day]");
    return -1;
}

Dictionary<int, IDay> days = new() { { 1, new Day1() } };
int dayNo;

if (!int.TryParse(args[0], out dayNo)) {
    Console.WriteLine("Usage: aocs [day] (day must be an integer you dingus)");
    return -1;
}

if (!days.ContainsKey(dayNo)) {
    Console.WriteLine($"Day {dayNo} not completed");
    return -1;
}

string contents;

try {
    StreamReader file = new($"../input/day{dayNo}.txt");
    contents = file.ReadToEnd();
} catch {
    Console.WriteLine($"Couldn't read input file \"../input/day{dayNo}.txt\"! (Go download it)");
    return -1;
}

days[dayNo].Solve(contents);

return 0;