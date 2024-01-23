package main

import (
	"fmt"
	"github.com/gin-gonic/gin"
	"net/http"
	"os"
)

func main() {
	server := gin.Default()

	server.Static("/download", "./asset")
	server.GET("/update/:currentVersion", func(ctx *gin.Context) {
		currentVersion := ctx.Param("currentVersion")
		fmt.Println(currentVersion)
		ctx.Status(http.StatusNoContent)
	})

	port := os.Getenv("PORT")
	if err := server.Run(":" + port); err != nil {
		fmt.Printf("Start Server Error: %v \n", err)
	}
}
