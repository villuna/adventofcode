import * as fs from 'fs';

const SOURCE = [500, 0];

class Cave {
  cave: Set<string>;
  height: number;

  constructor() {
    this.cave = new Set();
    this.height = -1;
  }

  has(p: [number, number] | number[]): boolean;
  has(xOrP: number | [number, number] | number[], y?: number): boolean {
    if (y == undefined && Array.isArray(xOrP)) {
      return this.cave.has(xOrP.toString());
    } else if (typeof xOrP == 'number' && typeof y == 'number') {
      return this.cave.has([xOrP, y].toString());
    } else {
      throw 'Incorrect arguments to Cave.has';
    }
  }

  add(p: [number, number] | number[]) {
    this.cave.add(p.toString());
  }

  toString(xRange: number[]): string {
    let res = '';

    for (let y = 0; y < this.height + 2; y++) {
      for (let x = xRange[0]; x <= xRange[1]; x++) {
        if (y == this.height + 1 || this.has([x,y])) {
          res += '#';
        } else if (x == SOURCE[0] && y == SOURCE[1]) {
          res += '+';
        } else {
          res += '.';
        }
      }

      res += '\n';
    }

    return res;
  }
};

function parseStructure(input: string): [number, number][] {
  return input.split(" -> ")
    .map((str) => {
      let coords = str.split(",");
      return [parseInt(coords[0]), parseInt(coords[1])];
    });
}

function readInput(input: string): Cave {
  let structures = input.split("\n")
    .map(parseStructure);
  let maxHeight: number = -1;
  
  let cave = new Cave();

  structures.forEach((structure) => {
    for (let i = 0; i < structure.length - 1; i += 1) {
      let start = structure[i];
      let end = structure[i + 1];

      if (start[1] >= maxHeight) {
        maxHeight = start[1]; 
      }

      if (end[1] >= maxHeight) {
        maxHeight = end[1];
      }

      // Add every point in the line to the cave
      if (start[0] == end[0]) {
        // Same x value
        let sign = Math.sign(end[1] - start[1]);
        for (let y = start[1]; y != end[1]; y += sign) {
          cave.add([start[0], y]);
        }

        cave.add(end);
      } else if (start[1] == end[1]){
        // Same y value
        let sign = Math.sign(end[0] - start[0]);
        for (let x = start[0]; x != end[0]; x += sign) {
          cave.add([x, start[1]]);
        }

        cave.add(end);
      } else {
        throw "Rock path not a lateral line!";
      }
    }
  });

  cave.height = maxHeight + 1;
  return cave;
}

// Returns the units of sand that can be added to the cave before they start 
// falling into the abyss
function simulate(cave: Cave, hasFloor: boolean): number {
  let counter = 0;

  while (true) {
    // Spawn a new piece of sand
    let sandPosition = [...SOURCE];

    while (true) {
      // Keep moving it down until it is stationary or it falls to the abyss

      if (!hasFloor) {
        // Check if it's in the abyss yet
        if (sandPosition[1] >= cave.height) {
          return counter;
        }
      }

      // Move the sand
      if (hasFloor && sandPosition[1] >= cave.height) {
        // If it reaches the floor it's done
        cave.add(sandPosition);
        break;
      } else {
        if (!cave.has([sandPosition[0], sandPosition[1] + 1])) {
          sandPosition[1] += 1;
        } else if (!cave.has([sandPosition[0] - 1, sandPosition[1] + 1])) {
          sandPosition[0] -= 1;
          sandPosition[1] += 1;
        } else if (!cave.has([sandPosition[0] + 1, sandPosition[1] + 1])) {
          sandPosition[0] += 1;
          sandPosition[1] += 1;
        } else {
          // We can't move, so just stay here and make a new sand grain
          cave.add(sandPosition);
          break;
        }
      }
    }

    counter += 1;

    // If the sand is clogging up the source we're done
    if (sandPosition[0] == SOURCE[0] && sandPosition[1] == SOURCE[1]) {
      return counter;
    }
  }
}

fs.readFile('../input/day14.txt', 'utf8', (err, data) => {
  if (err) throw err;

  let cave = readInput(data);
  let partOne = simulate(cave, false);
  console.log(partOne);
  let partTwo = partOne + simulate(cave, true);
  //console.log(cave.toString([450, 550]));
  console.log(partTwo);
});
