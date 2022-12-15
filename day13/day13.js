const fs = require("fs");
const path = require("path");

function compare(a, b) {
  if (typeof a == "number" && typeof b == "number") {
    if (a > b) {
      return -1;
    }

    if (a === b) {
      return 0;
    }

    return 1;
  }

  if (typeof a !== "number" && typeof b === "number") {
    return compare(a, [b]);
  }

  if (typeof b !== "number" && typeof a === "number") {
    return compare([a], b);
  }

  let i = 0;
  while (i < a.length && i < b.length) {
    let res = compare(a[i], b[i]);

    if (res !== 0) {
      return res;
    }
    i++;
  }

  if (i === a.length && i === b.length) {
    return 0;
  }

  if (a.length > b.length) {
    return -1;
  }

  return 1;
}

function partA(input) {
  const data = input.split("\n\n").map((l) => l.split("\n").map(JSON.parse));
  let count = 0;
  for (let i = 0; i < data.length; i++) {
    if (compare(...data[i]) === 1) {
      count += i + 1;
    }
  }
  return count;
}

function partB(input) {
  const data = input.replaceAll("\n\n", "\n").split("\n").map(JSON.parse);

  const a = [[2]];
  const b = [[6]];
  data.push(a);
  data.push(b);

  const sorted = data.sort((a, b) => compare(a, b) * -1);

  return (sorted.indexOf(a) + 1) * (sorted.indexOf(b) + 1);
}

console.log(
  `part 1 sample: ${partA(
    fs.readFileSync(path.join(__dirname, "sample_input.txt")).toString()
  )}`
);
console.log(
  `part 1: ${partA(
    fs.readFileSync(path.join(__dirname, "input.txt")).toString()
  )}`
);
console.log(
  `part 2 sample: ${partB(
    fs.readFileSync(path.join(__dirname, "sample_input.txt")).toString()
  )}`
);
console.log(
  `part 2: ${partB(
    fs.readFileSync(path.join(__dirname, "input.txt")).toString()
  )}`
);
