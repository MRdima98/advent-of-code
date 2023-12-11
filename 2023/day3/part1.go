package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func checkUpDown(above_line string, line string, below_line string, i int) bool {
    if above_line == "" {
        if below_line[i] != '.' {
            return true 
        }
    } else if below_line == "" {
        if above_line[i] != '.' {
            return true 
        }
    } else {
        if below_line[i] != '.' || above_line[i] != '.' {
            return true 
        }
    }
    return false 
}

func validateNumber(above_line string, line string, below_line string, indexes []int) bool {
    for i := indexes[0]; i <= indexes[1]; i++ {
        if checkUpDown(above_line, line, below_line, i) {
            return true 
        }
    }
    if indexes[0] != 0 {
        if line[indexes[0] - 1] != '.' {
            return true
        }
        if checkUpDown(above_line, line, below_line, indexes[0] - 1) {
            return true
        }
    }
    if len(line) != indexes[1] + 1 {
        fmt.Println(string(line[indexes[1]]))
        if line[indexes[1]] != '.' { 
            return true
        }
        return checkUpDown(above_line, line, below_line, indexes[1])
    }
    return false
}


func getRowSum (above_line string, line string, below_line string) int {
    re_number := regexp.MustCompile(`\d+`)
    sum := 0
    for true {
        number := re_number.FindString(line)
        if number == "" {
            break
        }
        indexes := re_number.FindStringIndex(line)
        if indexes[1] == len(line) {
            indexes[1] = indexes[1] - 1
            if validateNumber(above_line, line, below_line, indexes) {
                num, _ := strconv.Atoi(number)
                fmt.Println(num)
                sum += num
            }
            break
        }
        if validateNumber(above_line, line, below_line, indexes) {
            num, _ := strconv.Atoi(number)
            fmt.Println(num)
            sum += num
        }
        line = line[indexes[1] :]
        if above_line != "" {
            above_line = above_line[indexes[1] :]
        }
        if below_line != "" {
            below_line = below_line[indexes[1] :]
        }
    }
    return sum
}

func main() {
    input_file := os.Args[1]
    readFile, err := os.Open(input_file)

    if err != nil {
        fmt.Println("Error reading file")
    }

    fileScanner := bufio.NewScanner(readFile)

    fileScanner.Split(bufio.ScanLines)

    engine := [] string {}

    for fileScanner.Scan() {
        engine = append(engine, fileScanner.Text())
    }

    sum := 0
    for i, line := range engine {
        above_line := ""
        below_line := ""
        if i > 0 {
            above_line = engine[i - 1]
        }
        if i < len(engine) - 1 {
            below_line = engine[i + 1]
        }
        sum += getRowSum(above_line, line, below_line)
    }

    fmt.Println(sum)

    readFile.Close()
}

