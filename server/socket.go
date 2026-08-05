package main

import (
	"encoding/json"
	"fmt"
	"net"
	"os"
)

type jsonData struct {
	Code int
	Coolword string
}

func socketandparse() {
	socketPath := "/tmp/bulwark.sock"
	if err := os.RemoveAll(socketPath); err != nil{
		fmt.Println("Error delete old socket", err)
		os.Exit(1)
	}

	listener, err := net.Listen("unix", socketPath)
	if err != nil {
		fmt.Println("error for make socket", err)
		os.Exit(1)
	}
	defer listener.Close()

	fmt.Printf("[Сервер] Слушаю сокет: %s\n", socketPath)

	for {
		// Получаем переменную conn от сокета
		conn, err := listener.Accept()
		if err != nil {
			fmt.Println("error for connect: ", err)
			continue
		}

		// Передаем переменную conn дальше в обработчик
		go handleClient(conn) 
	}
}

func handleClient(conn net.Conn) {
	defer conn.Close()
	fmt.Println("[Сервер] Клиент подключился, ожидаю поток JSON...")

	decoder := json.NewDecoder(conn)

	for {
		var data UserData
		err := decoder.Decode(&data)
		if err != nil {
			fmt.Println("[Сервер] Соединение закрыто или ошибка:", err)
			break
		}

		fmt.Printf("[Сервер] Получены новые данные: %+v\n", data)
	}
}
