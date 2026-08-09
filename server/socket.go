package main

import (
	"encoding/json"
	"fmt"
	"net"
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

		fmt.Printf("IP: %v, MAC-ADDRESS: %v\n", data.IP, data.Mac)
	}
}
