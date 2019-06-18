package main

import "fmt"

func main()  {
	var n, x int
	fmt.Scan(&n, &x)

	sum := 0
	l := make([]int, n+1)
	for i:=1;i<=n;i++ {
		fmt.Scan(&l[i])
	}

	for i:=0;i<=n;i++ {
		sum += l[i]
		if sum > x {
			fmt.Println(i)
			break
		} else if i == n {
			fmt.Println(i+1)
		}
	}
}