package main

import(
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
