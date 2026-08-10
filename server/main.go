package main

import "sync"

func main(){
	InitDB()
	defer DB.Close()
	go socketandparse()

	var wg sync.WaitGroup
	wg.Add(1)
	wg.Wait()
}
