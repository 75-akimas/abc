package main

import (
	"fmt"
	"sort"
)

func main() {
	var n, m int
	fmt.Scan(&n, &m)

	ids := make(Ids, m)
	num := make([]int, n)
	dict := map[Id]int{}
	for i := range ids {
		fmt.Scan(&ids[i].p, &ids[i].y)
		a := &Id{ids[i].p, ids[i].y}
		dict[*a] = 0
	}
	ids2 := make(Ids, m)
	copy(ids2, ids)
	sort.Sort(ids)

	for i := 0; i < m; i++ {
		num[ids[i].p-1]++
		a := &Id{ids[i].p, ids[i].y}
		dict[*a] = num[ids[i].p-1]
	}
	for i := 0; i < m; i++ {
		a := &Id{ids2[i].p, ids2[i].y}
		fmt.Printf("%06d%06d\n", ids2[i].p, dict[*a])
	}
}

type Id struct {
	p int
	y int
}

type Ids []Id

func (a Ids) Len() int {
	return len(a)
}

func (a Ids) Swap(i, j int) {
	a[i], a[j] = a[j], a[i]
}

func (a Ids) Less(i, j int) bool {
	if a[i].p < a[j].p {
		return true
	} else if a[i].p == a[j].p {
		if a[i].y < a[j].y {
			return true
		}
	}
	return false
}
