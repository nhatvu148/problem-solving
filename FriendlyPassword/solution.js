"use strict";

const fs = require("fs");

process.stdin.resume();
process.stdin.setEncoding("utf-8");

let inputString = "";
let currentLine = 0;

process.stdin.on("data", function (inputStdin) {
  inputString += inputStdin;
});

process.stdin.on("end", function () {
  inputString = inputString.split("\n");

  main();
});

function readLine() {
  return inputString[currentLine++];
}

/*
 * Complete the 'authEvents' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts 2D_STRING_ARRAY events as parameter.
 */

function authEvents(events) {
  let currentPassword = "";
  const hash = (word) => {
    const arr = [];

    for (let i = 0; i < word.length; i++) {
      let coeff = word[i].charCodeAt(0) * Math.pow(131, word.length - 1 - i);
      arr.push(coeff);
    }

    return arr.reduce((a, c) => a + c) % (Math.pow(10, 9) + 7);
  };

  let hashRes = 0;

  const alphabet1 = "abcdefghijklmnopqrstuvwxyz".split("");
  const alphabet2 = "abcdefghijklmnopqrstuvwxyz".toUpperCase().split("");
  const digit = "0123456789".toUpperCase().split("");
  const alpha = [...alphabet1, ...alphabet2, ...digit];

  const res = events.map((ef) => {
    if (ef[0] === "setPassword") {
      currentPassword = ef[1];
      hashRes = hash(ef[1]);
    } else if (ef[1] == hashRes) {
      return 1;
    } else {
      let key = false;
      alpha.forEach((af) => {
        let temp = currentPassword + af;

        if (hash(temp) == ef[1]) {
          key = true;
        }
      });

      // console.log(hash(ef[1].slice(0, ef[1].length)));
      return key ? 1 : 0;
    }
  });
  // console.log(currentPassword);
  return res.filter((rf) => !!rf || rf === 0);
}
function main() {
  const ws = fs.createWriteStream(process.env.OUTPUT_PATH);

  const eventsRows = parseInt(readLine().trim(), 10);

  const eventsColumns = parseInt(readLine().trim(), 10);

  let events = Array(eventsRows);

  for (let i = 0; i < eventsRows; i++) {
    events[i] = readLine().replace(/\s+$/g, "").split(" ");
  }

  const result = authEvents(events);

  ws.write(result.join("\n") + "\n");

  ws.end();
}
