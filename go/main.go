package main

import (
	"fmt"
	// "io"
	"os"
)

func main() {
    dat, err := os.ReadFile("./input.txt")
    if err != nil {
        panic(err)
    } else {
        fmt.Println(string(dat))
    }

    // scanner := bufio.NewScanner(dat)
}