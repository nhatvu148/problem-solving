'use strict';

const fs = require('fs');

process.stdin.resume();
process.stdin.setEncoding('utf-8');

let inputString = '';
let currentLine = 0;

process.stdin.on('data', function(inputStdin) {
    inputString += inputStdin;
});

process.stdin.on('end', function() {
    inputString = inputString.split('\n');

    main();
});

function readLine() {
    return inputString[currentLine++];
}



/*
 * Complete the 'getMaxFreqDeviation' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts STRING s as parameter.
 */

const getAllSubstrings = (str, size) => {
  let i,
    j,
    result = [];
  size = size || 0;
  for (i = 0; i < str.length; i++) {
    for (j = str.length; j - i >= size; j--) {
      result.push(str.slice(i, j));
    }
  }
  return result;
};

const getFreqDeviation = (str) => {
  const itemCounts = str.split("").reduce((obj, cur) => {
    return {
      ...obj,
      [cur]: obj[cur] !== undefined ? obj[cur] + 1 : 1,
    };
  }, {});
  const values = Object.values(itemCounts);
  return Math.max(...values) - Math.min(...values);
};

function getMaxFreqDeviation(s) {
    const subStrings = getAllSubstrings(s).filter(s => s !== "").sort((a,b) => a.length - b.length);
    // console.log(subStrings);
    const freqDeviationArr = subStrings.map(sStr => getFreqDeviation(sStr));
    // console.log(freqDeviationArr);
    return Math.max(...freqDeviationArr);
}

function main() {
    const ws = fs.createWriteStream(process.env.OUTPUT_PATH);

    const s = readLine();

    const result = getMaxFreqDeviation(s);

    ws.write(result + '\n');

    ws.end();
}
