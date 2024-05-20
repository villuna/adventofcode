using AoCs;
using Framework;

// Parse input argument (the day to solve)
if (args.Length != 1) {
    Console.WriteLine("Usage: aocs [day]");
    return -1;
}

int dayNo;

if (!int.TryParse(args[0], out dayNo)) {
    Console.WriteLine("Usage: aocs [day] (day must be an integer you dingus)");
    return -1;
}

// A collection of day solver classes
// which we will use to access the correct solution
Dictionary<int, IDay> days = new() { 
    { 1, new Day1() },
    { 2, new Day2() },
};

if (!days.TryGetValue(dayNo, out IDay? day)) {
    Console.WriteLine($"Day {dayNo} not completed");
    return -1;
}

// Read input file
string contents;

try {
    StreamReader file = new($"../input/day{dayNo}.txt");
    contents = file.ReadToEnd();
} catch {
    Console.WriteLine($"Couldn't read input file \"../input/day{dayNo}.txt\"! (Go download it)");
    return -1;
}

day.Solve(contents);

return 0;