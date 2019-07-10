package main

import (
	"fmt"
	"sort"
)

func main() {
	var n int
	fmt.Scan(&n)

	d := make([]int, n)
	for i := 0; i < n; i++ {
		fmt.Scan(&d[i])
	}

	sort.Ints(d)
	dv := len(d) / 2
	fmt.Println(d[dv] - d[dv-1])
}
