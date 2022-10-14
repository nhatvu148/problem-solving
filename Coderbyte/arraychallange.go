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
