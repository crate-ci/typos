package main

import (
	"fmt"
)

func printDict(dict []string) {
	length := len(dict)
	for i, word := range dict {
		if i%2 == 0 && i != length {
			fmt.Printf("%s,%s\n", word, dict[i+1])
		}
	}
}

func main() {
	printDict(DictMain)
	printDict(DictAmerican)
}
