package main

import (
	"fmt"
	"sort"
)

func main()  {
	var n int
	fmt.Scan(&n)
	a := make(pairs, n)
	for i := range a {
		fmt.Scan(&a[i].val)
		a[i].idx = i
	}
	sort.Sort(a)

	for i := range a {
		for j:=n-1;j>=0;j-- {
			if (i != a[j].idx) {
				fmt.Println(a[j].val)
				break
			}
		}
	}
}

type pair struct {
	idx int
	val int
}

type pairs []pair

func (p pairs) Len() int {
	return len(p)
}

func (p pairs) Less(i, j int) bool {
	return p[j].val > p[i].val
}

func (p pairs) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}


