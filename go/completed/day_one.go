package day_one

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
)

func compact(file *os.File) int {
    scanner := bufio.NewScanner(file)
    compacted := []int{0}
    for scanner.Scan() {
        if scanner.Text() != "" {
            v, err := strconv.ParseUint(scanner.Text(), 10, 32)
            if err != nil {
                panic(err)
            } else {
                compacted[len(compacted)-1] += int(v)
            }
        } else {
            compacted = append(compacted, 0)
        }
    }
    sort.Slice(compacted, func(i, j int) bool {
        return compacted[i] > compacted[j]
    })
    // return compacted[0]
    return compacted[0] + compacted[1] + compacted[2]
}

func main() {
    file, err := os.Open("./input.txt")
    if err != nil {
        panic(err)
    } else {
        fmt.Println(compact(file))
    }
}