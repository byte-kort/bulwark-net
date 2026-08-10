package main

import (
	"encoding/json"
	"fmt"
	"net"
	"time"
)
func socketandparse() {
	socketPath := "/tmp/bulwark.sock"
	conn, err := net.Dial("unix", socketPath); 
    for{
	fmt.Printf("Connect to socket: %s\n", socketPath)

	if err == nil {
		break
	}
	fmt.Printf("Failed to connect: %v. Retrying...\n", err)
	time.Sleep(10 * time.Millisecond)
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
			fmt.Println("error", err)
			break
		}

		fmt.Printf("IP: %v, MAC-ADDRESS: %v\n", data.IP, data.Mac)
	}
}
