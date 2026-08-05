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

	fmt.Printf("Connect to socket: %s\n", socketPath)

	conn, err := net.Dial("unix", socketPath); 
	if err != nil{
		fmt.Printf("Failed to connect to the socket: %v\n", err)
		return
	}
	defer conn.Close()

	fmt.Printf("listening on the socket %s\n", socketPath)
	decoder := json.NewDecoder(conn)
	for {
		var data jsonData
		err := decoder.Decode(&data)
		if err != nil{
			if err.Error() == "EOF"{
				fmt.Println("End of life server")
			} else {
				fmt.Printf("Connection error")
			}
			fmt.Printf("error", err)
			break
		}

		fmt.Printf("Code: %d, Coolword: %s\n", data.Code, data.Coolword)
	}
}
