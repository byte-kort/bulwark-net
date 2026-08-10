package main

import (
	"net/http"
	"fmt"
	"encoding/json"
	"time"
	"sync"
)
var (
	latestDeviceData jsonData
	deviceMu sync.RWMutex
)
func StartWebServer(){
	mux := http.NewServeMux()
	mux.HandleFunc("/", func(w http.ResponseWriter, r *http.Request){
		if r.URL.Path == "/"{
			http.ServeFile(w, r, "./index.html")
			return
		}
		http.NotFound(w, r)
	})
	mux.HandleFunc("/style.css",  func(w http.ResponseWriter, r *http.Request){
		w.Header().Set("Content-Type", "text/css")
		http.ServeFile(w, r, "./style.css")
	})
	mux.HandleFunc("api/register", func(w http.ResponseWriter, r *http.Request){
		if r.Method != http.MethodPost{
			http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
			return
		}
		username := r.FormValue("username")
		password := r.FormValue("password")
		err := RegisterUser(username,password)
		if err != nil{
			w.WriteHeader(http.StatusBadRequest)
			fmt.Fprintf(w, "registration failed")
		}
		fmt.Fprintf(w, "Registration successful for user: %s!", username)
	})
	mux.HandleFunc("/api/login", func(w http.ResponseWriter, r *http.Request){
		if r.Method != http.MethodPost{
			http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
			return
		}
		username := r.FormValue("username")
		password := r.FormValue("password")
		_, err := LoginUser(username, password)
		if err != nil{
			w.WriteHeader(http.StatusUnauthorized)
			fmt.Fprintf(w, "login failed %v", err)
			return
		}
		http.SetCookie(w, &http.Cookie{
			Name: "session_token",
			Value: "authenphicated_" + username,
			Expires: time.Now().Add(24 * time.Hour),
			HttpOnly: true,
			Secure: true,
			Path: "/",
		})
		fmt.Fprintf(w, "Welcome, %s! Login succesful via secure channel.", username)
	})
	mux.HandleFunc("/api/events", handleEvent)
	
	go func() {
		fmt.Println("http redirect server started on port :80")
		err := http.ListenAndServe(":80", http.HandlerFunc(func(w http.ResponseWriter, r *http.Request){
			target := "https://" + r.Host + r.URL.Path
			if len(r.URL.RawQuery) > 0{
				target += "?" + r.URL.RawQuery
			}
			http.Redirect(w, r, target, http.StatusMovedPermanently)
		}))
		if err != nil{
			fmt.Printf("http redirect server failed: %v\n", err)
		}
		certFile := "server.crt"
		keyFile := "server.key"
		fmt.Println ("https web server succesfully started on port :443")
		if err := http.ListenAndServeTLS(":443", certFile,keyFile,mux); err != nil{
			panic(fmt.Errorf("failed t start HTTPS web server: %w", err))
		}
	}()
}

func handleEvent(w http.ResponseWriter, r *http.Request){
	w.Header().Set("Strict-Transport-Security", "max-age=63072000; incluseSubDomains; preload")

	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}
	_, err := r.Cookie("session_token")
	if err != nil{
		w.WriteHeader(http.StatusUnauthorized)
		w.Header().Set("Content-Type", "application/json")
		fmt.Fprint(w, `{"error": "Unautorized: secure HTTPS session not found"}`)
		return
	}
	deviceMu.RLock()
	currentDevice := latestDeviceData
	deviceMu.RUnlock()
	if currentDevice.IP == nil{
		w.WriteHeader(http.StatusOK)
		w.Header().Set("Content-Type", "application/json")
		fmt.Fprint(w, `{"status": "waiting", "message": "No device data available from socket yet"}`)
		return
	}
	response := map[string]interface{}{
		"status": "success",
		"ip": currentDevice.IP.String(),
		"mac": currentDevice.Mac.String(),
	}
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	if err := json.NewEncoder(w).Encode(response); err != nil{
		fmt.Printf("failed to encode events json")
	}
}
