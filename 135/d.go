package main

import "fmt"

func main()  {
	var s string
	fmt.Scan(&s)

	const N int32 = 13
	dp := make([]int32, N)
	dp[0] = 1

	var c byte
	mul := int32(1)
	for i := len(s)-1; i>=0; i-- {
		nextDP := make([]int32, N)
		mod := int32(1000000007)
		c = s[i]
		if c == '?' {
			for k := int32(0); k < 10; k++ {
				for j := int32(0); j < N; j++ {
					nextDP[(k*mul+ j)%N] += dp[j]
					nextDP[(k*mul+ j)%N] %= mod
				}
				fmt.Println(string(s[i]), nextDP)
			}
		} else {
			k := int32(s[i] - '0')
			for j := int32(0); j < N; j++ {
				nextDP[(k*mul+ j)%N] += dp[j]
				nextDP[(k*mul+ j)%N] %= mod
			}
			fmt.Println(string(s[i]), nextDP)
		}
		mul *= 10
		mul %= N
		dp = nextDP
	}
	fmt.Println(dp[5])
}