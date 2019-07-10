package main

import (
	"fmt"
	"strconv"
	"strings"
)

func main() {
	var n int
	fmt.Scan(&n)
	count := 0
	for i := n; i > 0; {
		i /= 10
		count++
	}
	ans := 0
	for i := 3; i <= count; i++ {
		ans += dfs(n, i, "")
	}
	fmt.Println(ans)
}

func dfs(n int, digits int, sitigosan string) int {
	if digits == 0 {
		val, _ := strconv.Atoi(sitigosan)
		if val <= n && strings.Contains(sitigosan, "7") && strings.Contains(sitigosan, "5") && strings.Contains(sitigosan, "3") {
			return 1
		} else {
			return 0
		}
	}
	return dfs(n, digits-1, sitigosan+"3") + dfs(n, digits-1, sitigosan+"5") + dfs(n, digits-1, sitigosan+"7")
}
