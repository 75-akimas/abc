package main

import (
	"fmt"
	"math"
)

func main() {
	var s string
	fmt.Scan(&s)
	first := '0'
	firstCount := 0
	second := '1'
	secondCount := 0
	//fmt.Println(len(s))
	for _, v := range s {
		switch v {
		case first:
			firstCount++
		case second:
			secondCount++
		}
		//fmt.Println(firstCount, secondCount)
		first, second = second, first
	}
	fmt.Println(math.Min(float64(firstCount), float64(secondCount)))
}
