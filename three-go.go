package main

import (
	"fmt"
	"net/http"
	"io"
)

func main() {
	res, err := http.Get("https://plus.three.ie/core/offers/top")
	if err == nil {
		defer res.Body.Close()

		body, err := io.ReadAll(res.Body)
		if err == nil {
			fmt.Printf("%s\n", body)
		
		}
		fmt.Printf("TLS Version: %v\n", res.TLS.Version)
	} else {
	    fmt.Printf ("%v\n",err)
	}
	
		
}
