// Package longportwhale is the Go SDK for Longbridge WhaleAPI.
//
// It provides only what the WhaleAPI exposes: a generic HTTP client and a
// trade push websocket (TradeContext). See the http and trade sub-packages.
package longportwhale

// Language identifier.
type Language string

const (
	// LanguageZHCN is zh-CN.
	LanguageZHCN Language = "zh-CN"
	// LanguageZHHK is zh-HK.
	LanguageZHHK Language = "zh-HK"
	// LanguageEN is en.
	LanguageEN Language = "en"
)
