package main

import (
	"fmt"
	"github.com/Masterminds/semver/v3"
	"github.com/gin-gonic/gin"
	"net/http"
	"os"
)

func main() {
	server := gin.Default()

	server.Static("/download", "./asset")
	server.GET("/update/:currentVersion", checkUpdate)

	port := os.Getenv("PORT")
	if err := server.Run(":" + port); err != nil {
		fmt.Printf("Start Server Error: %v \n", err)
	}
}

func checkUpdate(ctx *gin.Context) {
	constraint, err := semver.NewConstraint("< 1.3.31")
	if err != nil {
		fmt.Printf("New Constraint Error: %v \n", err)
		ctx.Status(http.StatusNoContent)
	}

	currentVersion, err := semver.NewVersion(ctx.Param("currentVersion"))
	if err != nil {
		fmt.Printf("Current Version Error: %v \n", err)
		ctx.Status(http.StatusNoContent)
	}

	if !constraint.Check(currentVersion) {
		ctx.Status(http.StatusNoContent)
		return
	}

	data, err := os.ReadFile("asset/vr-game-launcher_1.3.31_x64_en-US.msi.zip.sig")
	if err != nil {
		fmt.Println("Read sig file error")
	}
	signature := string(data)

	//{
	//	"version": "0.2.0",
	//	"pub_date": "2020-09-18T12:29:53+01:00",
	//	"url": "https://mycompany.example.com/myapp/releases/myrelease.tar.gz",
	//	"signature": "Content of the relevant .sig file",
	//	"notes": "These are some release notes"
	//}

	ctx.JSON(http.StatusOK, gin.H{
		"version":   "1.3.31",
		"pub_date":  "2024-01-23T12:29:53+01:00",
		"url":       "https://update-test-dd55ddbe5912.herokuapp.com/download/vr-game-launcher_1.3.31_x64_en-US.msi.zip",
		"signature": signature,
		"notes":     "latest version",
	})
}
