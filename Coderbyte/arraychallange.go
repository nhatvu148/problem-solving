package main
import (
  "fmt"
  "sort"
)

func doubleSum(arr []int) int {
  sum := 0
  for _, v := range arr {
    sum += v
  }
  return sum * 2
}

func ArrayChallenge(arr []int) bool {
  dSumOfArr := doubleSum(arr)
  sort.Ints(arr)
  if arr[len(arr) - 1] * arr[len(arr) - 2] > dSumOfArr {
    return true
  }
  return false
}

func main() {

  // do not modify below here, readline is our function
  // that properly reads in the input for you
  fmt.Println(ArrayChallenge(readline()))

}

//Have the function ArrayChallenge(arr) take the array of numbers stored in arr and return the string true if any two numbers can be multiplied so that the answer is greater than double the sum of all the elements in the array. If not, return the string false. For example: if arr is [2, 5, 6, -6, 16, 2, 3, 6, 5, 3] then the sum of all these elements is 42 and doubling it is 84. There are two elements in the array, 16 * 6 = 96 and 96 is greater than 84, so your program should return the string true.
//Examples
//Input: []int {2, 2, 2, 2, 4, 1}
//Output: false
//Input: []int {1, 1, 2, 10, 3, 1, 12}
//Output: true
