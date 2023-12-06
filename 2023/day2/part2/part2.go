package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)
const start = 1

func splitGames (line string) (string, string) {
    re := regexp.MustCompile(`;`)
    index := re.FindStringIndex(line);
    extraction_line := ""
    if re.MatchString(line) {
        extraction_line = line[:index[0]]
        line = line[index[0] + 1:]
    }

    return extraction_line, line
}

func getGameId (line string) (int, string) {
    re_game := regexp.MustCompile(`:`)
    re_digit := regexp.MustCompile(`\d+`)

    digit := re_digit.FindString(line)
    num, _  := strconv.Atoi(digit)
    index := re_game.FindStringIndex(line)
    
    return num, line[index[0] + 1:]
}

func minBalls (line string) (int, int, int) {
    re_balls := regexp.MustCompile(`\d+`)
    re_comma := regexp.MustCompile(`,`)
    re_red := regexp.MustCompile(`red`)
    re_blue := regexp.MustCompile(`blue`)
    re_green := regexp.MustCompile(`green`)
    red := start
    green := start
    blue := start
    var digit int

    if line == "" {
        return red, green, blue
    }

    for line != "" {
        if re_comma.MatchString(line) {
            index := re_comma.FindStringIndex(line)
            tmp := line[:index[0]]
            digit, _ = strconv.Atoi(re_balls.FindString(tmp))
            switch os := true
            os {
            case re_red.MatchString(tmp) && digit > red:
                red = digit
            case re_green.MatchString(tmp) && digit > green:
                green = digit
            case re_blue.MatchString(tmp) && digit > blue:
                blue = digit
            }
            line = line[index[0] + 1 :]
        } else {
            digit, _ = strconv.Atoi(re_balls.FindString(line))
            switch os := true
            os {
            case re_red.MatchString(line) && digit > red:
                red = digit
            case re_green.MatchString(line) && digit > green:
                green = digit
            case re_blue.MatchString(line) && digit > blue:
                blue = digit
            }
            line = ""
        }
    }
    return red, green, blue
}

func minPower (line string) int {
    red := start
    green := start 
    blue := start
    for true {
        var extraction_game string
        extraction_game, line = splitGames(line)
        if extraction_game == "" {
            tmp_red, tmp_green, tmp_blue := minBalls(line)
            if (red < tmp_red) {
                red = tmp_red
            }
            if (green < tmp_green) {
                green = tmp_green
            }
            if (blue < tmp_blue) {
                blue = tmp_blue
            }
            break
        }
        tmp_red, tmp_green, tmp_blue := minBalls(extraction_game)
        if (red < tmp_red) {
            red = tmp_red
        }
        if (green < tmp_green) {
            green = tmp_green
        }
        if (blue < tmp_blue) {
            blue = tmp_blue
        }
    }
    fmt.Println(red, green, blue)
    return red * green * blue
}

func main() {

    readFile, err := os.Open("input")

    if err != nil {
        fmt.Println("Error reading file")
    }

    fileScanner := bufio.NewScanner(readFile)

    fileScanner.Split(bufio.ScanLines)

    total_power := 0;
    game_power := 0;

    for fileScanner.Scan() {
        line := fileScanner.Text()
        _, line = getGameId(line)
        game_power = minPower(line)
        fmt.Println("Game power: ", game_power)
        total_power += game_power
    }

    fmt.Println("Total power: ", total_power)

    readFile.Close()
}
