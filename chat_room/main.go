package main

import (
	"fmt"
	"github.com/gin-gonic/gin"
	"net/http"
)

func main() {
	hub := newHub()
	go hub.run()

	r := gin.Default()
	r.GET("/", func(context *gin.Context) {
		fmt.Println(context.Request.URL)
		if context.Request.URL.Path != "/" {
			context.String(http.StatusNotFound, "Not Found")
			return
		}
		if context.Request.Method != http.MethodGet {
			context.String(http.StatusMethodNotAllowed, "Method Not Allowed")
			return
		}
		http.ServeFile(context.Writer, context.Request, "home.html")
	})
	r.GET("/ws", func(context *gin.Context) {
		conn, err := upgrader.Upgrade(context.Writer, context.Request, nil)
		if err != nil {
			fmt.Println(err)
			return
		}
		client := &Client{hub: hub, conn: conn, send: make(chan []byte, 256)}
		client.hub.register <- client

		go client.writePump()
		go client.readPump()
	})
	r.Run()
}
