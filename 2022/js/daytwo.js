const fs = require("fs");

function value(char) {
  switch (char) {
    case "A":
    case "X":
      return 1;
    case "B":
    case "Y":
      return 2;
    case "C":
    case "Z":
      return 3;
    default:
      throw `Uh oh! Encountered character ${char}`;
  }
}

function transformData(input) {
  // >Takes one functional programming course
  return input.split("\n")
    .filter((s) => s.length !== 0)
    .map((s) => s.split(" "))
    .map((arr) => arr.map(value))
}

function partOne(input) {
  let total = 0;

  for (let i = 0; i < input.length; i++) {
    let match = input[i];

    total += match[1];

    if (match[0] % 3 == match[1] - 1) { 
      // A win
      total += 6;
    } else if (match[0] == match[1]) {
      // A draw
      total += 3;
    }
  }

  return total;
}

function partTwo(input) {
  // We can still use the transformed data from part 1, just need to transform it
  // 1 is a loss, 2 is a draw, 3 is a win
  let total = 0;

  for (let i = 0; i < input.length; i++) {
    let match = input[i];

    total += solve(match[0], match[1]);

    if (match[1] == 2) {
      total += 3;
    } else if (match[1] == 3) {
      total += 6;
    }
  }

  return total;
}

function solve(elfMove, outcome) {
  if (outcome == 1) {
    let res = elfMove - 1;

    if (res == 0) {
      res = 3;
    }

    return res;
  } else if (outcome == 2) {
    return elfMove;
  } else {
    let res = elfMove + 1;
    if (res == 4) {
      res = 1;
    }
    return res;
  }
} 

fs.readFile("../input/day2.txt", "utf8", (err, data) => {
  if (err) {
    console.log(err);
    return;
  }

  const input = transformData(data);
  
  console.log(partOne(input));
  console.log(partTwo(input));
});
