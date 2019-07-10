package main

import (
	"fmt"
	"math"
)

func main() {
	var n, d int
	fmt.Scan(&n, &d)

	x := make([][]float64, n)
	for i := 0; i < n; i++ {
		x[i] = make([]float64, d)
		for j := 0; j < d; j++ {
			fmt.Scan(&x[i][j])
		}
	}

	ans := 0
	for i := 0; i < n-1; i++ {
		for j := i + 1; j < n; j++ {
			var sum, tmp float64
			for k := 0; k < d; k++ {
				tmp = math.Abs(x[i][k] - x[j][k])
				tmp *= tmp
				sum += tmp
			}
			sum = math.Sqrt(sum)
			if sum == float64(int64(sum)) {
				ans++
			}
		}
	}

	fmt.Println(ans)
}
