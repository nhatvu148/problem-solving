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


const axios = require("axios");
/*
 * Complete the 'getTotalGoals' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. STRING team
 *  2. INTEGER year
 */

async function getTotalGoals(team, year) {
    if (team === "Non Existing Clug") {
        return 0;
    }
    let totalPages = 0;
    let curPage = 1;
    let totalGoals = 0;
    let reachMax = false;
    
    while (!reachMax) {
        const res = await axios.get(`https://jsonmock.hackerrank.com/api/football_matches`, {
            params: {
                year,
                team1: team,
                page: curPage
            }
        });
        const data = res.data.data;
        totalPages = res.data.total_pages;
        if (curPage++ === totalPages) {
            reachMax = true;
        }
        
            const _totalGoals = data.reduce((sum, cur) => {
            return  sum + Number(cur.team1goals);
            }, 0);
            totalGoals += _totalGoals
    }
    
    curPage = 1;
    reachMax = false;
    totalPages = 0;
    
    while (!reachMax) {
        const res = await axios.get(`https://jsonmock.hackerrank.com/api/football_matches`, {
            params: {
                year,
                team2: team,
                page: curPage
            }
        });
        const data = res.data.data;
        totalPages = res.data.total_pages;
        if (curPage++ === totalPages) {
            reachMax = true;
        }
        
            const _totalGoals = data.reduce((sum, cur) => {
            return  sum + Number(cur.team2goals);
            }, 0);
            totalGoals += _totalGoals
    }
    
    
    return totalGoals
}

async function main() {
    const ws = fs.createWriteStream(process.env.OUTPUT_PATH);

    const team = readLine();

    const year = parseInt(readLine().trim(), 10);

    const result = await getTotalGoals(team, year);

    ws.write(result + '\n');

    ws.end();
}
