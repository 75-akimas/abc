package main

import (
	"fmt"
	"sort"
)

func main()  {
	var n, q int
	fmt.Scan(&n, &q)
	stops := make(Stops, n)
	dist := map[int]int{}
	for i:=0;i<n;i++ {
		fmt.Scan(&stops[i].s, &stops[i].t, &stops[i].x)
	}
	sort.Sort(stops)

	d := make([]int, q)
	for i:=0;i<q;i++ {
		fmt.Scan(&d[i])
	}

	for i:=0;i<n;i++ {
		for j:=stops[i].s-stops[i].x;j<stops[i].t-stops[i].x;j++ {
			val, ok := dist[j]
			//fmt.Println(j, ok, val, stops[i].x)
			if  ok {
				if val > stops[i].x {
					dist[j] = stops[i].x
				} else {
					continue
				}
			} else if j >= 0 {
				dist[j] = stops[i].x
			}
		}
		//fmt.Println(dist)
	}

	for i:=0;i<q;i++ {
		if val, ok := dist[d[i]]; ok {
			fmt.Println(val)
		} else {
			fmt.Println(-1)
		}
	}
}

type Stop struct {
	s int
	t int
	x int
}

type Stops []Stop

func (a Stops) Len() int { return len(a) }
func (a Stops) Swap(i, j int) { a[i], a[j] = a[j], a[i] }
func (a Stops) Less(i, j int) bool { return a[i].x < a[j].x }