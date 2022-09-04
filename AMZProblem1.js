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
 * Complete the 'maximumQuality' function below.
 *
 * The function is expected to return a LONG_INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY packets
 *  2. INTEGER channels
 */

const getMedian = (arr) => {
    const middle = (arr.length + 1) / 2;

    const sorted = [...arr].sort((a, b) => a - b);
    const isEven = sorted.length % 2 === 0;

    return isEven ? (sorted[middle - 1.5]
        + sorted[middle - 0.5]) / 2 :
        sorted[middle - 1];
}

function maximumQuality(packets, channels) {
    const sortedPackets = [...packets].sort((a, b) => a - b);
    let quality = 0;
    
    for (let i = channels; i > 1; i--) {
        quality = quality + sortedPackets.pop();
    }
    
    return Math.round(quality + getMedian(sortedPackets));
}
function main() {
    const ws = fs.createWriteStream(process.env.OUTPUT_PATH);

    const packetsCount = parseInt(readLine().trim(), 10);

    let packets = [];

    for (let i = 0; i < packetsCount; i++) {
        const packetsItem = parseInt(readLine().trim(), 10);
        packets.push(packetsItem);
    }

    const channels = parseInt(readLine().trim(), 10);

    const result = maximumQuality(packets, channels);

    ws.write(result + '\n');

    ws.end();
}
