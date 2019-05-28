package main

import (
	"fmt"
	"sort"
)

func main()  {
	var n,m int
	fmt.Scan(&n, &m)
	var a [100001]int
	var b, c [100001]int
	arr_val := make([]int, n+m, n+m)
	d := map[int]int{}

	for i:=0;i<n;i++ {
		fmt.Scan(&a[i])
		arr_val[i] = a[i]
		d[a[i]]++
	}
	for i:=0;i<m;i++ {
		fmt.Scan(&b[i], &c[i])
		d[c[i]] += b[i]
		arr_val[i+n] = c[i]
	}
	sort.Ints(arr_val)

	//fmt.Println(arr_val)

	sum := 0
	for i:=len(arr_val); i>0; i-- {
		for d[arr_val[i-1]] > 0 && n > 0 {
			//fmt.Println(arr_val[i-1], d[arr_val[i-1]], n)
			d[arr_val[i-1]]--
			n--
			sum += arr_val[i-1]
		}
	}

	fmt.Println(sum)
}