package main

import "fmt"

func main()  {
	var n, m int
	fmt.Scan(&n, &m)

	s, t := make([]int, n), make([]int, m)
	for i:=0;i<n;i++ {
		fmt.Scan(&s[i])
	}
	for i:=0;i<m;i++ {
		fmt.Scan(&t[i])
	}


}