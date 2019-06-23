package main

import (
	"fmt"
	"math"
)

func main() {
	var n, a, b, c int
	fmt.Scan(&n, &a, &b, &c)
	l := make([]int, n)
	for i := 0; i < n; i++ {
		fmt.Scan(&l[i])
	}
	fmt.Println(dfs(n, 0, l, a, b, c, 0, 0, 0))
}

func dfs(n int, depth int, l []int, A int, B int, C int, a int, b int, c int) int {
	if depth == n {
		if a > 0 && b > 0 && c > 0 {
			//fmt.Println( int(math.Abs(float64(A-a)) + math.Abs(float64(B-b)) + math.Abs(float64(C-c))) - 30)
			return int(math.Abs(float64(A-a))+math.Abs(float64(B-b))+math.Abs(float64(C-c))) - 30
		}
		return int(10e9)
	}

	min := dfs(n, depth+1, l, A, B, C, a, b, c)
	min = int(math.Min(float64(dfs(n, depth+1, l, A, B, C, a+l[depth], b, c)+10), float64(min)))
	min = int(math.Min(float64(dfs(n, depth+1, l, A, B, C, a, b+l[depth], c)+10), float64(min)))
	return int(math.Min(float64(dfs(n, depth+1, l, A, B, C, a, b, c+l[depth])+10), float64(min)))

}
