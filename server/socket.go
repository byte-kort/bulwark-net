package main

import (
	"encoding/json"
	"fmt"
	"net"
	"time"
)

type jsonData struct {
	IP net.IP `json:"ip"`
	Mac net.HardwareAddr `json:"mac"`
}


func (j *jsonData) UnmarshalJSON(data []byte) error {
	type Alias jsonData
	aux := struct{
		IP string `json:"ip"`
		Mac string `json:"mac"`
		*Alias
	}{
		Alias: (*Alias)(j),
	}
	if err := json.Unmarshal(data, &aux); err != nil{
		return err
	}
	parsedIP := net.ParseIP(aux.IP)
	if parsedIP == nil || parsedIP.To4() == nil{
		return  fmt.Errorf("invalid or non-ipv4 address: %s", aux.IP)
	}
	j.IP = parsedIP.To4()
	parsedMAC, err := net.ParseMAC(aux.Mac)
	if err != nil{
		return fmt.Errorf("invalid MAC address %s: %w", aux.Mac, err)
	}
	j.Mac = parsedMAC
	return nil
}

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
