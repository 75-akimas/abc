package main

import "fmt"

func main()  {
	var n int
	var k int64
	fmt.Scan(&n, &k)

	a := make([]int64, n)
	for i:=0;i<n;i++ {
		fmt.Scan(&a[i])
	}
	var sum, count, j int64

	for i:=0;i<n;i++ {
		for sum < k && j<int64(n) {
			sum += a[j]
			j++
		}
		if sum<k {
			break
		}
		count += int64(n)-j+1
		sum -= a[i]
	}

	fmt.Println(count)
}
