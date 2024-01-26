package handler

import (
	"fmt"
	"github.com/gin-gonic/gin"
	"os"
)

func StartServer() {
	server := gin.Default()

	server.POST("/version", updateLatestVersion)
	server.GET("/version", checkLatestVersion)

	port := os.Getenv("PORT")
	if err := server.Run(":" + port); err != nil {
		fmt.Printf("Start Server Error: %v \n", err)
	}
}
