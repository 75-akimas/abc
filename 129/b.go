package main

import (
	"fmt"
	"math"
)

func main() {
	var n int
	fmt.Scan(&n)
	w := make([]int, n)

	for i := 0; i < n; i++ {
		fmt.Scan(&w[i])
	}

	ans := int(10e9)
	for i := 1; i < n; i++ {
		l, r := 0, 0
		for j := 0; j < i; j++ {
			l += w[j]
		}
		for k := i; k < n; k++ {
			r += w[k]
		}
		if subs := int(math.Abs(float64(l - r))); subs < ans {
			ans = subs
		}
	}
	fmt.Println(ans)
}
