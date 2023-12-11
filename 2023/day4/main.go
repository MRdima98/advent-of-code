package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func main() {
    input_file := os.Args[1]
    readFile, err := os.Open(input_file)

    if err != nil {
        fmt.Println("Error reading file")
    }

    fileScanner := bufio.NewScanner(readFile)

    fileScanner.Split(bufio.ScanLines)

    points := 0

    for fileScanner.Scan() {
        line := fileScanner.Text()
        points += singleGamePoints(line)
    }

    fmt.Println(points)

    readFile.Close()
}

func singleGamePoints(line string) int {
    game := 0
    choosen, winners := parseGames(line)
    for _, el := range choosen {
        for _, el2 := range winners {
            if el == el2 {
                if game == 0 {
                    game = 1
                } else {
                    game *= 2
                }
            }
        }
    }
    parseGames(line)
    return game
}

func parseGames(line string) ([]int, []int) {
    var choosen []int
    var winners []int
    colon_re := regexp.MustCompile(`:`)
    pipe_re := regexp.MustCompile(`\|`)
    index := colon_re.FindStringIndex(line)
    fmt.Println(line)
    line = line[index[0] + 1:]
    index = pipe_re.FindStringIndex(line)
    rawChoosen := line[:index[0] - 1]
    choosen = stringToArray(rawChoosen)
    rawWinners := line[index[0] + 1:]
    winners = stringToArray(rawWinners)
    return choosen, winners
}

func stringToArray(line string) []int {
    var nums []int
    digit_re := regexp.MustCompile(`\d+`)
    for true {
        num := digit_re.FindString(line)
        index := digit_re.FindStringIndex(line)
        num_parsed, _ := strconv.Atoi(num)
        nums = append(nums, num_parsed)
        if index[1] == len(line) {
            break
        }
        line = line[index[1] + 1:]
    }
    return nums
}
