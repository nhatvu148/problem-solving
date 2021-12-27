package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"sort"
	"strconv"
	"strings"
)

/*
 * Complete the 'customSorting' function below.
 *
 * The function is expected to return a STRING_ARRAY.
 * The function accepts STRING_ARRAY strArr as parameter.
 */

func customSorting(strArr []string) []string {
	// var result []string
	var oddArr []string
	var evenArr []string
	for _, str := range strArr[0:] {
		strLen := len(str)
		if strLen%2 == 0 {
			evenArr = append(evenArr, str)
		} else {
			oddArr = append(oddArr, str)
		}

	}

	sort.Slice(oddArr, func(i, j int) bool {
		l1, l2 := len(oddArr[i]), len(oddArr[j])
		if l1 != l2 {
			return l1 < l2
		}
		return oddArr[i] < oddArr[j]
	})

	sort.Slice(evenArr, func(i, j int) bool {
		l1, l2 := len(evenArr[i]), len(evenArr[j])
		if l1 != l2 {
			return l1 > l2
		}
		return evenArr[i] < evenArr[j]
	})

	fmt.Println(oddArr)
	fmt.Println(evenArr)
	oddArr = append(oddArr, evenArr...)
	return oddArr
}

func main() {
	reader := bufio.NewReaderSize(os.Stdin, 16*1024*1024)

	stdout, err := os.Create(os.Getenv("OUTPUT_PATH"))
	checkError(err)

	defer stdout.Close()

	writer := bufio.NewWriterSize(stdout, 16*1024*1024)

	strArrCount, err := strconv.ParseInt(strings.TrimSpace(readLine(reader)), 10, 64)
	checkError(err)

	var strArr []string

	for i := 0; i < int(strArrCount); i++ {
		strArrItem := readLine(reader)
		strArr = append(strArr, strArrItem)
	}

	result := customSorting(strArr)

	for i, resultItem := range result {
		fmt.Fprintf(writer, "%s", resultItem)

		if i != len(result)-1 {
			fmt.Fprintf(writer, "\n")
		}
	}

	fmt.Fprintf(writer, "\n")

	writer.Flush()
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
