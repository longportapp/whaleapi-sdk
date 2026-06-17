package util

import (
	"unsafe"
)

// UnsafeStringToBytes converts a string to bytes without memory allocation.
func UnsafeStringToBytes(s string) []byte {
	if len(s) == 0 {
		return nil
	}
	return unsafe.Slice(unsafe.StringData(s), len(s))
}

// UnsafeBytesToString converts bytes to a string without memory allocation.
func UnsafeBytesToString(b []byte) string {
	if len(b) == 0 {
		return ""
	}
	return unsafe.String(unsafe.SliceData(b), len(b))
}
