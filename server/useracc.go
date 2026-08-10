package main

import (
	"database/sql"
	"errors"
	"fmt"
	"time"
	_ "github.com/lib/pq"
	"golang.org/x/crypto/bcrypt"
)

type User struct {
	ID int 
	Username string
	PasswordHash string
	CreatedAT time.Time
}

var DB *sql.DB
func InitDB(){
	host := "localhost"
	port := 5432
	user := "postgres"
	password := "your_password"
	dbname := "users_db"

	connstr := fmt.Sprintf("host=%s port=%d user=%s password=%s dbname=%s sslmode = disable\n", host, port,user, password,dbname)
	var err error
	DB, err = sql.Open("postgres", connstr)
	if err != nil{
		fmt.Sprintf("failed to open database connection")
	}
	err = DB.Ping()
	if err != nil{
		fmt.Sprintf("database is unreachable: %v\n")
	}
	fmt.Println("succesfully connected to the database!\n")
}
func RegisterUser(username, password string) error {
	if username == "" || password == "" {
		return errors.New("username and password cannot be empty")
	}
	hashedPassword, err := bcrypt.GenerateFromPassword([]byte(password), bcrypt.DefaultCost)
	if err != nil{
		return fmt.Errorf("failed to hash password: %w")
	}
	query := `INSERT INTO users (username, password_hash, created_at) VALUES ($1, $2, $3)`
	_, err = DB.Exec(query, username, string(hashedPassword), time.Now())
	if err != nil{
		fmt.Println("failed to register user (username might be taken)\n")
	}
		fmt.Printf("User %s succesfully saved to the database!\n", username)
		return nil
}
func LoginUser(username, password string) (User, error) {
	var user User
	query := `SELECT id, username, password_hash, created_at FROM users WHERE username = $1`
	err := DB.QueryRow(query, username).Scan(&user.ID, &user.Username, &user.PasswordHash, &user.CreatedAT)
	if err != nil {
		if err == sql.ErrNoRows{
			return User{}, errors.New("user not found")
		}
		return User{}, err
	}
	err = bcrypt.CompareHashAndPassword([]byte(user.PasswordHash), []byte(password))
	if err != nil {
		return User{}, errors.New("invalid password")
	}
	fmt.Printf("user %s succesfully authenphicated!\n", username)
	return user,nil
}
