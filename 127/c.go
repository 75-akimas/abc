package main

import (
	"fmt"
	"math"
)

func main() {
	var n, m int
	fmt.Scan(&n, &m)
	var l, r [100000]int
	for i:=0;i<m;i++ {
		fmt.Scan(&l[i], &r[i])
		if i>0 {
			l[0] = int(math.Max(float64(l[0]), float64(l[i])))
			r[0] = int(math.Min(float64(r[0]), float64(r[i])))
		}
	}
	count := 0
	if l[0] <= r[0] {
		count = r[0] - l[0] + 1
	}
	fmt.Println(count)
}