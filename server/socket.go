package main

import (
	"encoding/json"
	"fmt"
	"net"
)

type jsonData struct {
	Code int
	Coolword string
}

func socketandparse() {
	socketPath := "/tmp/bulwark.sock"

	fmt.Printf("Connect to socket: %s", socketPath)

	conn, err := net.Dial("unix", socketPath); 
	if err != nil{
		fmt.Printf("\tFailed to connect to the socket: %v\n", err)
		return
	}
	defer conn.Close()

	fmt.Printf("[Сервер] Слушаю сокет: %s\n", socketPath)
	decoder := json.NewDecoder(conn)
	for {
		var data jsonData

		err := decoder.Decode(&data)
		if err != nil{
			fmt.Printf("error", err)
			break
		}

		fmt.Printf("")
	}
}
