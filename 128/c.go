package main
import (
	"fmt"
	"math"
	"sort"
)

func enumeration(src []int) (result [][]int) {
	length := int(math.Pow(2, float64(len(src))))
	result = make([][]int, length)

	index := 0
	result[index] = []int{}
	index++

	for _, n := range src {
		max := index
		for i:=0;i<max;i++ {
			result[index] = copyAndAppend(result[i], n)
			index++
		}
	}


	return result
}

func copyAndAppend(src []int, n int) []int {
	dst := make([]int, len(src)+1)
	copy(dst, src)
	dst[len(src)] = n
	return dst
}

func arr_common(a []int, b []int) int {
	count := 0
	for _, aa := range a {
		for _, bb := range b {
			if aa == bb {
				count++
				break
			}
		}
	}
	return count
}

func main() {
	var n, m int
	fmt.Scan(&n, &m)
	p := make([]int, m)
	s := make([][]int, m)
	var s_exist []int
	for i:=0;i<n;i++ {
		s_exist = append(s_exist, i+1)
	}
	k := make([]int, m)
	for i:=0;i<m;i++ {
		fmt.Scan(&k[i])
		s[i] = make([]int, k[i])
		for j:=0;j<k[i];j++ {
			fmt.Scan(&s[i][j])
		}
	}
	for i:=0;i<m;i++ {
		fmt.Scan(&p[i])
	}
	sort.Ints(s_exist)
	enum := enumeration(s_exist)
	//fmt.Println(enum)
	count := 0

	for _, nn:= range enum {
		cond := true
		if len(nn) == 0 {
			aa := true
			for _, nnnn := range p {
				if (nnnn > 0) {
					aa = false
					break
				}
			}
			if aa {
				count++
			}
			continue
		}
		for ki := range k {
			common := arr_common(nn, s[ki])
			//fmt.Println(kk, nn, s[ki], common%2, p[ki])
			if common%2 != p[ki] {
				cond = false
				break
			}
		}

		if cond {
			//fmt.Println("true")
			count++
		} else {
			//fmt.Println("false")
		}
	}
	fmt.Println(count)
}
package main
import (
"fmt"
"sort"
"math"
)

func enumeration(src []int) (result [][]int) {
	length := int(math.Pow(2, float64(len(src))))
	result = make([][]int, length)

	index := 0
	result[index] = []int{}
	index++

	for _, n := range src {
		max := index
		for i:=0;i<max;i++ {
			result[index] = copyAndAppend(result[i], n)
			index++
		}
	}


	return result
}

func copyAndAppend(src []int, n int) []int {
	dst := make([]int, len(src)+1)
	copy(dst, src)
	dst[len(src)] = n
	return dst
}

func arr_common(a []int, b []int) int {
	count := 0
	for _, aa := range a {
		for _, bb := range b {
			if aa == bb {
				count++
				break
			}
		}
	}
	return count
}

func main() {
	var n, m int
	fmt.Scan(&n, &m)
	p := make([]int, m)
	s := make([][]int, m)
	var s_exist []int
	for i:=0;i<n;i++ {
		s_exist = append(s_exist, i+1)
	}
	k := make([]int, m)
	for i:=0;i<m;i++ {
		fmt.Scan(&k[i])
		s[i] = make([]int, k[i])
		for j:=0;j<k[i];j++ {
			fmt.Scan(&s[i][j])
		}
	}
	for i:=0;i<m;i++ {
		fmt.Scan(&p[i])
	}
	sort.Ints(s_exist)
	enum := enumeration(s_exist)
	//fmt.Println(enum)
	count := 0

	for _, nn:= range enum {
		cond := true
		if len(nn) == 0 {
			aa := true
			for _, nnnn := range p {
				if (nnnn > 0) {
					aa = false
					break
				}
			}
			if aa {
				count++
			}
			continue
		}
		for ki := range k {
			common := arr_common(nn, s[ki])
			//fmt.Println(kk, nn, s[ki], common%2, p[ki])
			if common%2 != p[ki] {
				cond = false
				break
			}
		}

		if cond {
			//fmt.Println("true")
			count++
		} else {
			//fmt.Println("false")
		}
	}
	fmt.Println(count)
}