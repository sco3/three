package main

import (
	"fmt"
	"net/http"
)

func main() {
	res, err := http.Get("https://plus.three.ie/core/offers/top")
	if err != nil {
		fmt.Printf("Fail: %v\n", err)
	}

	fmt.Printf("%v", res)
}
