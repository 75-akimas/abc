package main

import (
	"fmt"
	"math"
	"sort"
)

func main()  {
	var n,m int
	fmt.Scan(&n, &m)
	var a [100000]int
	var b, c [100000]int

	for i:=0;i<n;i++ {
		fmt.Scan(&a[i])
	}
	for i:=0;i<m;i++ {
		fmt.Scan(&b[i], &c[i])
	}
	a_s := a[0:n]
	sort.Ints(a_s)

	var bi int
	for i:=0; i<m; i++ {
		bi = int(math.Max(float64(n), float64(b[i])))
		fmt.Println(a_s)
		for j:=n; j>0 && bi>0; j-- {
			if c[i] > a_s[j-1] {
				if j>1 {
					bi--
					last_ := a_s[j:]
					last := make([]int, cap(last_))
					copy(last, last_)
					temp := append(a_s[1:j], c[i])
					a_s = append(temp, last...)
				} else {
					a_s[j-1] = c[i]
				}
			}
		}
	}

	var count int64
	count = 0
	for i:=0;i<n;i++ {
		count += int64(a_s[i])
	}
	fmt.Println(count)

}