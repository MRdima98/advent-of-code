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

func validateNumber (above_line string, line string, below_line string, indexes [] int) bool {
    for i := indexes[0]; i <= indexes[1]; i++ {
        valid := checkUpDown(above_line, line, below_line, i)
        if valid {
            return valid 
        }
    }
    if indexes[0] != 0 {
        if line[indexes[0] - 1] != '.' {
            return true
        }
        valid := checkUpDown(above_line, line, below_line, indexes[0] - 1)
        if valid {
            return valid 
        }
    }
    if len(line) != indexes[1] + 1 {
        if line[indexes[1]] != '.' { 
            return true
        }
        return checkUpDown(above_line, line, below_line, indexes[1] +1 )
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
            break
        }
        if validateNumber(above_line, line, below_line, indexes) {
            num, _ := strconv.Atoi(number)
            // fmt.Println(num)
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

    const ROWS = 3
    const COLUMNS = 10
    var base [ROWS][COLUMNS] rune
    for i := 0; i < ROWS; i++ {
        for j := 0; j < COLUMNS; j++ {
            base[i][j] = '.'
        }
    }
    var cases [] [ROWS][COLUMNS] rune
    stuff := []int{0, 4, 9}

    for i := 0; i < ROWS; i++ {
        tmp := base
        for _, el := range stuff {
            tmp := makeExample(tmp, cases, i, el)
            cases = append(cases, tmp...)
        }
    }
    // for _, el := range cases {
    //     for _, el2 := range el {
    //         fmt.Printf("%c\n", el2)
    //     }
    //     fmt.Println()
    // }

}

func makeExample(base [3][10] rune, cases [][3][10] rune, index int, index2 int) ([] [3][10] rune) {
    base[index][index2] = '1'
    for i := index - 1; i <= index + 1; i++ {
        tmp := base
        for j := index2 - 1; j <= index2 + 1; j++ {
            if i >= 0 && j >= 0 && i < 3 && j < 10 && i != j {
                tmp[i][j] = '*'
                cases = append(cases, tmp)
                fmt.Println(len(cases))
            }
        }
    }
    return cases
}
