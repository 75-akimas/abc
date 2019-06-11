package main

import (
	"fmt"
)

func main() {
	var n, m int
	fmt.Scan(&n, &m)

	a := make([]int, m)
	dp := make([]int, n+2)
	broken := make([]bool, n+2)
	dp[n] = 1
	for i := range a {
		fmt.Scan(&a[i])
		broken[a[i]] = true
	}
	for i := n - 1; i >= 0; i-- {
		if broken[i] {
			dp[i] = 0
		} else {
			dp[i] = (dp[i+1] + dp[i+2]) % 1000000007
		}
	}
	fmt.Println(dp[0])
}
