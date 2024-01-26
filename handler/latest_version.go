package handler

import (
	"encoding/json"
	"fmt"
	"github.com/Masterminds/semver/v3"
	"github.com/gin-gonic/gin"
	"net/http"
	"os"
	"strconv"
	"time"
)

type LatestVersion struct {
	Version        string
	BuildTimeStamp string
	Signature      string
}

func checkLatestVersion(ctx *gin.Context) {
	latestVersionData, err := os.ReadFile("latest_version.json")
	if err != nil {
		ctx.Status(http.StatusInternalServerError)
		return
	}

	var latestVersion LatestVersion
	if err := json.Unmarshal(latestVersionData, &latestVersion); err != nil {
		ctx.Status(http.StatusInternalServerError)
		return
	}

	versionCompareConstraint, err := semver.NewConstraint("< " + latestVersion.Version)
	if err != nil {
		ctx.Status(http.StatusInternalServerError)
		return
	}

	reqCurrVerString := ctx.Param("currentVersion")
	reqCurrVer, err := semver.NewVersion(reqCurrVerString)
	if err != nil {
		ctx.Status(http.StatusBadRequest)
		return
	}

	if versionCompareConstraint.Check(reqCurrVer) {
		buildTimestamp, err := strconv.ParseInt(latestVersion.BuildTimeStamp, 10, 64)
		if err != nil {
			ctx.Status(http.StatusInternalServerError)
			return
		}

		pubDate := time.Unix(buildTimestamp, 0).UTC().Format("2006-01-02T15:04:05") + "+00:00"

		ctx.JSONP(http.StatusOK, gin.H{
			"version":   latestVersion.Version,
			"pub_date":  pubDate,
			"url":       "https://github.com/jakevr360action/VR360ActionSoftware/releases/download/VRGameLauncher/vr-game-launcher_" + latestVersion.Version + "_x64_en-US.msi.zip",
			"signature": latestVersion.Signature,
			"notes":     "There is a latest version.",
		})
		return
	}

	ctx.Status(http.StatusNoContent)
}

func updateLatestVersion(ctx *gin.Context) {
	authHeaderValue := ctx.GetHeader("Authorization")
	fmt.Println(authHeaderValue)
	if len(authHeaderValue) == 0 || authHeaderValue != "vr360action_vr-game-launcher_update_latest_version" {
		ctx.Status(http.StatusForbidden)
		return
	}

	ctx.String(http.StatusOK, "utf8", "Welcome to update latest version")

	//var latestVersion LatestVersion
	//if err := ctx.ShouldBind(&latestVersion); err != nil {
	//	ctx.Status(http.StatusBadRequest)
	//	return
	//}
	//jsonFileContent, err := json.MarshalIndent(latestVersion, "", "")
	//if err != nil {
	//	ctx.Status(http.StatusInternalServerError)
	//	return
	//}
	//if err := os.WriteFile("latest_version.json", jsonFileContent, 0644); err != nil {
	//	ctx.Status(http.StatusInternalServerError)
	//	return
	//}
}
