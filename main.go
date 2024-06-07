package main

import (
	"github.com/gin-gonic/gin"
	"log"
	"net/http"
)

func main() {
	hub := newHub()
	go hub.run()
	r := gin.Default()
	r.GET("/ws", func(c *gin.Context) {
		auth := c.GetHeader("Authorization")
		if auth != "nllWB61TGC" {
			c.Status(http.StatusForbidden)
			return
		}
		conn, err := upgrader.Upgrade(c.Writer, c.Request, nil)
		if err != nil {
			log.Println(err)
			return
		}
		client := &Client{hub: hub, conn: conn, send: make(chan []byte, 256)}
		client.hub.register <- client

		// Allow collection of memory referenced by the caller by doing all work in
		// new goroutines.
		go client.writePump()
		go client.readPump()
	})
	r.Run() // listen and serve on 0.0.0.0:8080
}
