package utils

import (
	"crypto/sha256"
	"encoding/hex"
)

func SHA256ToString(content string) string {
	hash := sha256.Sum256([]byte(content))
	return hex.EncodeToString(hash[:])
}
