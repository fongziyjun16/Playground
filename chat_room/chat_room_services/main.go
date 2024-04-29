package main

import (
	"example/chat_room_services/utils"
	"fmt"
	"github.com/gin-gonic/gin"
	"net/http"
	"regexp"
	"strconv"
	"strings"
	"time"
)

type TokenInfo struct {
	Username    string
	Timestamp   int64
	SnowflakeId string
}

func main() {
	hub := newHub()
	go hub.run()

	r := gin.Default()

	r.Use(func(context *gin.Context) {
		context.Writer.Header().Set("Access-Control-Allow-Origin", "*")
		context.Writer.Header().Set("Access-Control-Allow-Credentials", "true")
		context.Writer.Header().Set("Access-Control-Allow-Headers", "Authorization, Content-Type")
		context.Writer.Header().Set("Access-Control-Allow-Methods", "POST, DELETE, PUT, GET, OPTIONS")

		if context.Request.Method == http.MethodOptions {
			context.AbortWithStatus(http.StatusOK)
			return
		}

		context.Next()
	})

	tokenSecret := "chat-room"
	usernameRegExpPattern := regexp.MustCompile("^[a-zA-Z0-9_]+(?: [a-zA-Z0-9_]+)*$")
	r.POST("/newSpeaker/:username", func(context *gin.Context) {
		username := context.Param("username")
		if !usernameRegExpPattern.MatchString(username) {
			context.Status(http.StatusBadRequest)
			return
		}

		snowflakeId := utils.NextSnowflakeIdString()
		timestamp := time.Now().Unix()
		base := fmt.Sprintf("%s:%d:%s:%s", username, timestamp, snowflakeId, tokenSecret)
		token := fmt.Sprintf("%s|%s|%d|%s", utils.SHA256ToString(base), username, timestamp, snowflakeId)

		context.String(http.StatusOK, token)
	})

	tokenVerifier := func(token string) (bool, TokenInfo) {
		parts := strings.Split(token, "|")
		if len(parts) != 4 {
			return false, TokenInfo{}
		}

		encryptedBase := parts[0]
		username := parts[1]
		timestamp, err := strconv.ParseInt(parts[2], 10, 64)
		if err != nil || time.Now().Unix()-timestamp >= 3600 {
			return false, TokenInfo{}
		}
		snowflakeId := parts[3]

		if encryptedBase != utils.SHA256ToString(fmt.Sprintf("%s:%d:%s:%s", username, timestamp, snowflakeId, tokenSecret)) {
			return false, TokenInfo{}
		}

		return true, TokenInfo{Username: username, Timestamp: timestamp, SnowflakeId: snowflakeId}
	}

	r.GET("/isNew", func(context *gin.Context) {
		context.Status(http.StatusOK)
		headerValue := context.GetHeader("Authorization")
		prefixLength := len("Bearer ")
		if len(headerValue) < prefixLength {
			context.Status(http.StatusUnauthorized)
			return
		}
		if rs, _ := tokenVerifier(headerValue[prefixLength:]); !rs {
			context.Status(http.StatusUnauthorized)
		}
	})

	r.GET("/ws", func(context *gin.Context) {
		//verificationResult, tokenInfo := tokenVerifier(context.GetHeader("Sec-WebSocket-Protocol"))
		//if !verificationResult {
		//	context.Status(http.StatusUnauthorized)
		//	return
		//}

		conn, err := upgrader.Upgrade(context.Writer, context.Request, nil)
		if err != nil {
			fmt.Println(err)
			return
		}
		client := &Client{tokenInfo: TokenInfo{}, hub: hub, conn: conn, send: make(chan []byte, 256)}
		client.hub.register <- client

		go client.writePump()
		go client.readPump()
	})

	r.Run()
}
