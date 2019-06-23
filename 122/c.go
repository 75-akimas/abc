package main

import "fmt"

func main() {
	var n, q int
	fmt.Scan(&n, &q)
	var s string
	t := make([]int, n)
	fmt.Scan(&s)
	for i := 1; i < n; i++ {
		t[i] = t[i-1]
		if s[i-1] == 'A' && s[i] == 'C' {
			t[i]++
		}
	}
	//fmt.Println(t)
	l, r := make([]int, q), make([]int, q)
	for i := 0; i < q; i++ {
		fmt.Scan(&l[i], &r[i])
	}
	for i := 0; i < q; i++ {
		fmt.Println(t[r[i]-1] - t[l[i]-1])
	}
}
