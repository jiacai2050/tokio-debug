package main

import (
	"fmt"
	"html"
	"log"
	"net/http"
	"time"
)

func main() {
	log.SetFlags(log.Lmicroseconds | log.Ldate | log.Ltime | log.Lshortfile)
	handler := func(w http.ResponseWriter, r *http.Request) {
		now := time.Now()
		log.Printf("Get request %s\n", r.URL.Path)

		time.Sleep(time.Duration(time.Millisecond * 50))

		fmt.Fprintf(w, "Hello, %q", html.EscapeString(r.URL.Path))
		log.Printf("%s cost: %dms\n", r.URL.Path, time.Since(now).Milliseconds())
	}

	addr := ":8080"
	log.Printf("Listen on %s...\n", addr)
	log.Fatal(http.ListenAndServe(addr, http.HandlerFunc(handler)))
}
