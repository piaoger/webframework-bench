package main

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
)

type User struct {
	Id       int32  `json:"id"`
	Username string `json:"user_name"`
	Wechat   string `json:"wechat,omitempty"`
}

func hello(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintf(w, "Hello, World!")
}

func user(w http.ResponseWriter, r *http.Request) {
	user := User{
		Id:       10,
		Username: "net_http90",
		Wechat:   "mywechat_10",
	}
	w.Header().Set("content-type", "application/json")
	res, _ := json.Marshal(user)

	io.WriteString(w, string(res))
}

func main() {
	fmt.Println("net/http(go) - http://localhost:8091")

	http.HandleFunc("/hello", hello)
	http.HandleFunc("/user", user)

	http.ListenAndServe(":8091", nil)
}
