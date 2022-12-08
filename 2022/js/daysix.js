const fs = require("fs");

function findPacket(signal, length) {
  for (let i = 0; i < signal.length - length - 1; i++) {
    // minus one because of the newline character
    if (stringIsUnique(signal.slice(i, i + length))) {
      return i + length;
    }
  }
}

function stringIsUnique(string) {
  let set = new Set();

  for (let i = 0; i < string.length; i++) {
    if (set.has(string[i])) {
      return false;
    } else {
      set.add(string[i]);
    }
  }

  return true;
}

fs.readFile("../input/day6.txt", "utf8", (err, data) => {
  if (err) {
    console.log(err);
    return;
  }

  console.log(findPacket(data, 4));
  console.log(findPacket(data, 14));
})
