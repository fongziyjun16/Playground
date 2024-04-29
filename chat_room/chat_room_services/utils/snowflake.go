package utils

import (
	"github.com/godruoyi/go-snowflake"
	"strconv"
)

func NextSnowflakeId() uint64 {
	snowflake.SetMachineID(360)
	id := snowflake.ID()
	return id
}

func NextSnowflakeIdString() string {
	return strconv.FormatUint(NextSnowflakeId(), 10)
}
