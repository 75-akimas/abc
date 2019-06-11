package main
import (
	"fmt"
	"sort"
)

type Info struct {
	idx int
	city string
	val int
}
type Guide []*Info

func newGuide (idx int, city string, val int) (g *Info) {
	g = new(Info)
	g.idx, g.city, g.val = idx, city, val
	return g
}

func (p Guide) Len() int {
	return len(p)
}
func (p Guide) Swap(i, j int) {
	p[i], p[j] = p[j], p[i]
}
func (p Guide) Less(i, j int) bool {
	if (p[i].city < p[j].city) {
		return true
	} else if (p[i].city == p[j].city) {
		if (p[i].val > p[j].val) {
			return true
		}
	}
	return false
}

func main() {
	var n, a int
	var b string
	fmt.Scan(&n)

	var info Guide
	for i:=0;i<n;i++ {
		fmt.Scan(&b, &a)
		info = append(info, newGuide(i+1, b, a))
	}

	sort.Sort(info)

	for i:=0;i<n;i++ {
		fmt.Println(info[i].idx)
	}
}
