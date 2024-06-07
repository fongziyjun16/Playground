package main

import (
	"fmt"
	"github.com/gin-gonic/gin"
	"log"
	"net/http"
	"os"
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
	port := os.Getenv("PORT")
	if err := r.Run(":" + port); err != nil {
		log.Println(fmt.Sprintf("Launch Services Error: %v", err))
		os.Exit(103)
	}
}
