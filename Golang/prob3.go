package main

import (
	"bufio"
	"fmt"
	"io"
	"math"
	"os"
	"strconv"
	"strings"
	"time"
)

/*
 * Complete the 'ModuloFibonacciSequence' function below.
 *
 * The function accepts following parameters:
 *  1. chan bool requestChan
 *  2. chan int resultChan
 */

const MOD = (1e9 + 7);

// https://www.geeksforgeeks.org/modulo-power-for-large-numbers-represented-as-strings/
func powerLL(x, n int) int {
        result := 1;
    for n > 0 {
        if n % 2 == 1 {
            result = result * x % MOD
        }
        n = n / 2;
        x = x * x % MOD
    }
    return result
}

func powerStrings(sa, sb string) int {
    // We convert strings to number
    a, b := 0, 0
     
    // calculating a % MOD
    for i := 0; i < len(sa); i++{
        a = (a * 10 + int((sa[i] - '0'))) % MOD;
    }
     
    // calculating b % (MOD - 1)
    for i := 0; i < len(sb); i++ {
        b = (b * 10 + int((sb[i] - '0'))) % (MOD - 1);
    }
     
    // Now a and b are long long let. We
    // calculate a^b using modulo exponentiation
    return powerLL(a, b);
}

func FibonacciLoop(n int) int {
	f := make([]int, n+1, n+2)
	if n < 2 {
		f = f[0:2]
	}
	f[0] = 0
	f[1] = 1
	for i := 2; i <= n; i++ {
		f[i] = f[i-1] + f[i-2]
	}
	return f[n]
}

func ModuloFibonacciSequence(requestChan chan bool, resultChan chan int) {
	i := 2
	for {
		select {
		case req := <-requestChan:
			if req {
				time.Sleep(10 * time.Millisecond)
				result := FibonacciLoop(i)
				i += 1
				resultChan <- result % int(math.Pow(10, 9))
			}
		}
	}
}
func main() {
	reader := bufio.NewReaderSize(os.Stdin, 16*1024*1024)

	skipTemp, err := strconv.ParseInt(strings.TrimSpace(readLine(reader)), 10, 64)
	checkError(err)
	skip := int32(skipTemp)

	totalTemp, err := strconv.ParseInt(strings.TrimSpace(readLine(reader)), 10, 64)
	checkError(err)
	total := int32(totalTemp)

	resultChan := make(chan int)
	requestChan := make(chan bool)
	go ModuloFibonacciSequence(requestChan, resultChan)
	for i := int32(0); i < skip+total; i++ {
		start := time.Now().UnixNano()
		requestChan <- true
		new := <-resultChan
		if i < skip {
			continue
		}
		end := time.Now().UnixNano()
		timeDiff := (end - start) / 1000000
		if timeDiff < 3 {
			fmt.Println("Rate is too high")
			break
		}
		fmt.Println(new)
	}
}

func readLine(reader *bufio.Reader) string {
	str, _, err := reader.ReadLine()
	if err == io.EOF {
		return ""
	}

	return strings.TrimRight(string(str), "\r\n")
}

func checkError(err error) {
	if err != nil {
		panic(err)
	}
}
