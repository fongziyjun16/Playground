package main

import (
	"encoding/json"
	"log"
)

type Hub struct {
	clients    map[string]*Client
	broadcast  chan []byte
	register   chan *Client
	unregister chan *Client
}

func newHub() *Hub {
	return &Hub{
		clients:    make(map[string]*Client),
		broadcast:  make(chan []byte),
		register:   make(chan *Client),
		unregister: make(chan *Client),
	}
}

type ChatMessage struct {
	From    string `json:"from"`
	To      string `json:"to"` // empty means broadcast
	Content string `json:"content"`
}

func (hub *Hub) run() {
	for {
		select {
		case client := <-hub.register:
			hub.clients[client.tokenInfo.SnowflakeId] = client
			log.Printf("New Client, Id: %s, Username: %s", client.tokenInfo.SnowflakeId, client.tokenInfo.Username)
		case client := <-hub.unregister:
			if _, ok := hub.clients[client.tokenInfo.SnowflakeId]; ok {
				delete(hub.clients, client.tokenInfo.SnowflakeId)
				close(client.send)
				log.Printf("Client Leaves, Id: %s, Username: %s", client.tokenInfo.SnowflakeId, client.tokenInfo.Username)
			}
		case message := <-hub.broadcast:
			var chatMessage ChatMessage
			if err := json.Unmarshal(message, &chatMessage); err != nil {
				log.Println("Wrong ChatMessage Format")
				continue
			}

			if len(chatMessage.To) == 0 {
				for id, client := range hub.clients {
					if client.tokenInfo.SnowflakeId == chatMessage.From {
						continue
					}
					select {
					case client.send <- []byte(chatMessage.Content):
					default:
						delete(hub.clients, id)
						close(client.send)
					}
				}
			} else {
				if targetClient, ok := hub.clients[chatMessage.To]; ok {
					select {
					case targetClient.send <- []byte(chatMessage.Content):
					default:
						delete(hub.clients, targetClient.tokenInfo.SnowflakeId)
						close(targetClient.send)
					}
				}
			}
		}
	}
}
