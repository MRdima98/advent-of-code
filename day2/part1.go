package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

const ( 
    red = "red"
    green = "green"
    blue = "blue"
    max_red = 12
    max_green = 13
    max_blue = 14 
)


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

func countCountBalls (line string) bool {
    re_balls := regexp.MustCompile(`\d+`)
    re_comma := regexp.MustCompile(`,`)
    re_red := regexp.MustCompile(`red`)
    re_blue := regexp.MustCompile(`blue`)
    re_green := regexp.MustCompile(`green`)
    var digit int

    if line == "" {
        return true
    }

    for line != "" {
        if re_comma.MatchString(line) {
            index := re_comma.FindStringIndex(line)
            tmp := line[:index[0]]
            // fmt.Println(tmp)
            digit, _ = strconv.Atoi(re_balls.FindString(tmp))
            switch os := true
            os {
            case re_red.MatchString(tmp):
                if digit > max_red {
                    return false 
                }
            case re_green.MatchString(tmp):
                if digit > max_green {
                    return false 
                }
            case re_blue.MatchString(tmp):
                if digit > max_blue {
                    return false 
                }
            }
            line = line[index[0] + 1 :]
        } else {
            digit, _ = strconv.Atoi(re_balls.FindString(line))
            switch os := true
            os {
            case re_red.MatchString(line):
                if digit > max_red {
                    return false 
                }
            case re_green.MatchString(line):
                if digit > max_green {
                    return false 
                }
            case re_blue.MatchString(line):
                if digit > max_blue {
                    return false 
                }
            }
            line = ""
        }
    }
    return true
}

func validGame(line string) bool {
    validGame := true
    for true {
        var extraction_game string
        extraction_game, line = splitGames(line)
        fmt.Println(extraction_game)
        if extraction_game == "" {
            validGame = countCountBalls(line)
            if validGame == false {
                return validGame
            }
            break
        }
        validGame = countCountBalls(extraction_game)
        if validGame == false {
            return validGame
        }
    }
    return validGame
}

func main() {

    readFile, err := os.Open("input")

    if err != nil {
        fmt.Println("Error reading file")
    }

    fileScanner := bufio.NewScanner(readFile)

    fileScanner.Split(bufio.ScanLines)

    possibileGames := 0;
    game_id := 0;

    for fileScanner.Scan() {
        line := fileScanner.Text()
        game_id, line = getGameId(line)
        if validGame(line) {
            possibileGames += game_id
            fmt.Println("validGame: ", game_id)
        }
    }

    fmt.Println("Possibile games: ", possibileGames)

    readFile.Close()
}
