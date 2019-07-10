package main

import "fmt"

func main() {
	var s string
	fmt.Scan(&s)

	m := make(map[uint8]int)
	for j := 0; j < 4; j++ {
		if _, ok := m[s[j]]; ok {
			m[s[j]]++
		} else {
			m[s[j]] = 1
		}
	}
	for _, v := range m {
		if v != 2 {
			fmt.Println("No")
			return
		}
	}
	fmt.Println("Yes")
}
