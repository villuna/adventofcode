package main

import (
    "fmt"
    "os"
    "strings"
    "strconv"
)

func day1() {
    input, err := os.ReadFile("../input/day1.txt")
    must(err)
    p1, p2 := d1(string(input))
    fmt.Println(p1)
    fmt.Println(p2)
}

func d1(input string) (int, int) {
    words := strings.Fields(input)
    var sum int
    var extra int

    for _, word := range words {
        i, err := strconv.Atoi(word)
        must(err)

        fuel := (i / 3) - 2
        sum += fuel
        extra += extraFuelMass(fuel)
    }

    return sum, extra
}

func extraFuelMass(fuel int) int {
    extra := fuel
    sum := 0

    for extra > 0 {
        sum += extra 
        extra = (extra / 3) - 2
    }

    return sum
}
