package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)
const infinity = 99999

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
    red := infinity
    green := infinity
    blue := infinity
    var digit int

    if line == "" {
        if red == infinity {
            red = 1
        }
        if green == infinity {
            green = 1
        }
        if blue == infinity {
            blue = 1
        }
        return red, green, blue
    }

    for line != "" {
        if re_comma.MatchString(line) {
            index := re_comma.FindStringIndex(line)
            tmp := line[:index[0]]
            digit, _ = strconv.Atoi(re_balls.FindString(tmp))
            switch os := true
            os {
            case re_red.MatchString(tmp):
                red = digit
            case re_green.MatchString(tmp):
                green = digit
            case re_blue.MatchString(tmp):
                blue = digit
            }
            line = line[index[0] + 1 :]
        } else {
            digit, _ = strconv.Atoi(re_balls.FindString(line))
            switch os := true
            os {
            case re_red.MatchString(line):
                red = digit
            case re_green.MatchString(line):
                green = digit
            case re_blue.MatchString(line):
                blue = digit
            }
            line = ""
        }
    }
    if red == infinity {
        red = 1
    }
    if green == infinity {
        green = 1
    }
    if blue == infinity {
        blue = 1
    }
    return red, green, blue
}

func minPower (line string) int {
    var red int
    var green int
    var blue int
    for true {
        red = infinity
        green = infinity 
        blue = infinity
        var extraction_game string
        extraction_game, line = splitGames(line)
        if extraction_game == "" {
            tmp_red, tmp_green, tmp_blue := minBalls(extraction_game)
            if red > tmp_red && green > tmp_green && blue > tmp_blue {
                red = tmp_red
                green = tmp_green
                blue = tmp_blue
            }
            break
        }
        tmp_red, tmp_green, tmp_blue := minBalls(extraction_game)
            fmt.Println(tmp_red, tmp_green, tmp_blue)
        if red > tmp_red && green > tmp_green && blue > tmp_blue {
            red = tmp_red
            green = tmp_green
            blue = tmp_blue
        }
    }
    fmt.Println(red, green, blue)
    return red * green * blue
}

func main() {

    readFile, err := os.Open("control_input")

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
        total_power += game_power
    }

    fmt.Println("Total power: ", total_power)

    readFile.Close()
}
