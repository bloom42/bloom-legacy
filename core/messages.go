package core

import (
	"encoding/json"

	"gitlab.com/bloom42/bloom/core/domain/groups"
)

type MessageIn struct {
	Method string          `json:"method"`
	Params json.RawMessage `json:"params"`
}

type MessageOut struct {
	Data  interface{} `json:"data"`
	Error *ErrorData  `json:"error"`
}

type ErrorData struct {
	Code    string      `json:"code"`
	Message string      `json:"message"`
	Meta    interface{} `json:"meta"`
}

type InitParams struct {
	Preferences    []string `json:"preferences"`
	DBKey          *string  `json:"dbKey"`
	Env            string   `json:"env"`
	BackgroundSync bool     `json:"backgroundSync"`
}

type InitRes struct {
	Preferences map[string]interface{} `json:"preferences"`
	Groups      []groups.Group         `json:"groups"`
}
