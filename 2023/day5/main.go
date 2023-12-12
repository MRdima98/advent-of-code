package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

type Almanac struct {
    seed  int
    soil  int
    fertilizer  int
    water  int
    light  int
    temperature  int
    humidity  int
    location  int
}

func main() {
    input_file := os.Args[1]
    readFile, err := os.Open(input_file)

    if err != nil {
        fmt.Println("Error reading file")
    }

    fileScanner := bufio.NewScanner(readFile)

    fileScanner.Split(bufio.ScanLines)

    
    var plants [] Almanac 

    for fileScanner.Scan() {
        line := fileScanner.Text()
        getLocations(&plants, line)
        getConditions(&plants, line)
    }

    fmt.Println("Part 1: ", plants)

    readFile.Close()
}

func getLocations(plants *[] Almanac, line string) {
    re_seeds := regexp.MustCompile("seeds:")
    re_nums := regexp.MustCompile(`\d+`)
    if re_seeds.MatchString(line) {
        for true {
            str_num := re_nums.FindString(line)
            num_pos := re_nums.FindStringIndex(line)
            num, _ := strconv.Atoi(str_num)
            var plant Almanac
            plant.seed = num
            *plants = append(*plants, plant)
            if num_pos[1] == len(line) {
                break
            }
            line = line[num_pos[1] + 1:]
        }
    }
}

func getConditions(plants *[] Almanac, line string) {
    var val []*int
    switch true {
    case regexp.MustCompile("seeds").MatchString(line):
        for _, el := range *plants {
            val = append(val, &el.seed)
        }
    case regexp.MustCompile("seed-to-soil").MatchString(line):
        for _, el := range *plants {
            val = append(val, &el.soil)
        }
    case regexp.MustCompile("soil-to-fert").MatchString(line):
        for _, el := range *plants {
            val = append(val, &el.fertilizer)
        }
    case regexp.MustCompile("zer-to-water").MatchString(line):
        for _, el := range *plants {
            val = append(val, &el.water)
        }
    case regexp.MustCompile("water-to-light").MatchString(line):
        for _, el := range *plants {
            val = append(val, &el.light)
        }
    case regexp.MustCompile("light-to-temp").MatchString(line):
        for _, el := range *plants {
            val = append(val, &el.temperature)
        }
    case regexp.MustCompile("ture-to-humid").MatchString(line):
        for _, el := range *plants {
            val = append(val, &el.humidity)
        }
    case regexp.MustCompile("ity-to-loc").MatchString(line):
        for _, el := range *plants {
            val = append(val, &el.location)
        }
    }
}

