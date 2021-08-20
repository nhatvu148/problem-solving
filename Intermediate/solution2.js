function minOperations(arr, threshold, d) {
  let divideCounter = 0;
  const newArr = arr.sort((a, b) => b - a);
  const transformedArr = [...newArr];
  console.log(transformedArr);

  for (let i = 0; i < arr.length - 1; i++) {
    let a = arr[i];
    let b = arr[i + 1];
    while (a !== b && a >= 0 && b >= 0) {
      if (a > b) {
        a = divideInt(a, d);
        // if (a === 0) {
        //   break;
        // }
        divideCounter++;
        transformedArr[i] = a;
        if (transformedArr[i - 1] !== undefined) {
          transformedArr[i - 1] === divideInt(transformedArr[i - 1], d);
        }
        if (findNumOfDup(transformedArr) === threshold) {
          console.log(
            transformedArr,
            divideCounter,
            findNumOfDup(transformedArr)
          );
          return divideCounter;
        }
      } else {
        b = divideInt(b, d);
        // if (b === 0) {
        //   break;
        // }
        divideCounter++;
        transformedArr[i + 1] = b;
        if (findNumOfDup(transformedArr) === threshold) {
          console.log(
            transformedArr,
            divideCounter,
            findNumOfDup(transformedArr)
          );
          return divideCounter;
        }
      }
    }
    console.log(a, b);

    if (findNumOfDup(transformedArr) === threshold) {
      return divideCounter;
    }
    console.log(transformedArr, divideCounter, findNumOfDup(transformedArr));
  }
  return divideCounter;
}

const divideInt = (a, b) => {
  return Math.floor(a / b);
};

const findNumOfDup = (arr) => {
  //   let arr = [1, 2, 2, 3, 1, 1, 4];
  let mf = 1;
  let m = 0;
  let item;

  for (let i = 0; i < arr.length; i++) {
    for (let j = i; j < arr.length; j++) {
      if (arr[i] == arr[j]) m++;
      if (mf < m) {
        mf = m;
        item = arr[i];
      }
    }

    m = 0;
  }

  //   console.log(item + " ( " + mf + " times ) ");
  return mf;
};

// const result = minOperations([1, 2, 3, 4, 5], 3, 2);
// const result = minOperations([64, 30, 25, 33], 2, 2);
const result = minOperations([1, 2, 3, 4], 4, 3);
console.log(`Result is ${result}`);
