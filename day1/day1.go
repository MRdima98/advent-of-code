package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	// "strings"
	"strconv"
)

func getStringDigit(re *regexp.Regexp, line string) string {
    val := re.FindString(line)
    switch os := val
    os {
        case "zero":
            return "0"
        case "one":
            return "1"
        case "two":
            return "2"
        case "three":
            return "3"
        case "four":
            return "4"
        case "five":
            return "5"
        case "six":
            return "6"
        case "seven":
            return "7"
        case "eight":
            return "8"
        case "nine":
            return "9"
        default: 
            return val
        
    }
}

func main() {

    readFile, err := os.Open("input")

    if err != nil {
        fmt.Println("Error reading file")
    }

    fileScanner := bufio.NewScanner(readFile)

    fileScanner.Split(bufio.ScanLines)

    var allNumbers []string
    for fileScanner.Scan() {
        r := regexp.MustCompile(`\d|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)|(zero)`)
        allDigits := ""
        line := fileScanner.Text()
        for true {
            digit := getStringDigit(r, line)
            index := r.FindStringIndex(line)
            fmt.Println(digit)
            if (digit != "" && len(index) > 0) {
                index := index[0]
                allDigits += digit
                if index == 0 && len(line) == 1 {
                    line = ""
                } else if index + 1 == len(line) {
                    line = line[:index - 1]
                } else if index == 0 {
                    line = line[index+1:]
                } else {
                    line = line[:index - 1] + line[index + 1:]
                }
            } else {
                allNumbers = append(allNumbers, allDigits )
                break
            }
        }
    }

    fmt.Println(allNumbers)

    sum := 0
    for i, el := range allNumbers {
        allNumbers[i] = string(el[0]) + string(el[len(el) - 1])
        num, _  := strconv.Atoi(allNumbers[i])
        sum += num
    }

    fmt.Println(len(allNumbers))
    fmt.Println(allNumbers)
    fmt.Println(sum)

    readFile.Close()
}
