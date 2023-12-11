package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func calcGearVal(row int, col int, engine []string) (int, error) {
    re := regexp.MustCompile(`\d`)
    number := string(engine[row][col])
    tmp := col
    for true { 
        tmp++
        if tmp == len(engine[row]) || !re.MatchString(string(engine[row][tmp])) {
            break
        }
        number = number + string(engine[row][tmp])
    }
    tmp = col
    for true { 
        tmp--
        if tmp == -1 || !re.MatchString(string(engine[row][tmp])) {
            break
        }
        number = string(engine[row][tmp]) + number
    }
    // fmt.Println(number)
    return strconv.Atoi(number)
}

func isGearShift(engine [] string, y int, x int) (bool, int) {
    first_gear := false
    second_gear := false
    var first_gear_val int
    var second_gear_val int
    fmt.Println()
    for i := x - 1; i <= x + 1; i++ {
        for j := y - 1; j <= y + 1; j++ {
            tmp, _ := calcGearVal(i, j, engine);
            if !first_gear  && tmp != 0{
                first_gear = true
                first_gear_val = tmp
                fmt.Println("First val: ", first_gear_val)
            } 
            if first_gear {
                tmp, _ := calcGearVal(i, j, engine);
                fmt.Println("Second: ", second_gear_val)
                if first_gear_val != tmp && tmp != 0{
                    fmt.Println("good")
                    second_gear_val = tmp
                    second_gear = true
                }
            }
        }
    }
    fmt.Println("Result: ", first_gear_val, second_gear_val)
    fmt.Println()
    return first_gear && second_gear, first_gear_val * second_gear_val
}

func gearShift(curr_line string, engine []string, x int) int {
    shift := 0
    for i, el := range curr_line {
        if el == '*' {
            cond, val := isGearShift(engine, i, x) 
            if cond {
                shift += val
            }
        }
    }
    return shift
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
        sum += gearShift(line, engine, i)
    }

    fmt.Println(sum)

    readFile.Close()
}

