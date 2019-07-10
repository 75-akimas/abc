package main

import (
	"fmt"
	"sort"
)

func main() {
	var n, m int
	fmt.Scan(&n, &m)
	x := make([]int, m)

	for i := range x {
		fmt.Scan(&x[i])
	}

	if n > m {
		fmt.Println(0)
		return
	}
	sort.Ints(x)

	ans := 0
	y := make([]int, m-1)
	for i := 0; i < m-1; i++ {
		y[i] = x[i+1] - x[i]
	}
	sort.Ints(y)
	for i := 0; i < m-n; i++ {
		ans += y[i]
	}
	fmt.Println(ans)
}
