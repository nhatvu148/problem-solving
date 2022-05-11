// function solution(digits) {
//     if (digits.length === 0) {
//         return [];
//     }
//     const inputNum = Number(digits.join(""));
//     const incrementedNum = inputNum + 1;
//     const resultArr = incrementedNum.toString().split("").map(n => Number(n));
//     console.log(resultArr);
//     return resultArr;
// }

function solution(digits) {
    let resultArr = [...digits];
    for (let i = digits.length - 1; i >= 0; i--) {
        if (digits[i] < 9) {
            resultArr[i] = resultArr[i] + 1;
            break;
        } else {
            resultArr[i] = 0;
        }
    }
    if (resultArr[0] === 0) {
        resultArr = [1, ...resultArr];
    }
    return resultArr;
}

// Input = [1, 3, 5]
// Output = [1, 3, 6]


// Input = [1, 3, 9]
// Output = [1, 4, 0]

// Input = [9, 9, 9]
// Output = [1, 0, 0, 0]


// Input = [1, 3, 5, .......... 1, 1]
// Output = [1, 3, 5, .......... 1, 2]


// Input = [1, 3, 5, .......... 1, 9]
// Output = [1, 3, 5, .......... 2, 0]
