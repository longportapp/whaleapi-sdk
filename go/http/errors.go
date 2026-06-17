package http

import (
	"fmt"
)

// ApiError represents a WhaleAPI error response.
type ApiError struct {
	HttpStatus int
	Code       int
	Message    string
	TraceID    string
}

func (ae *ApiError) Error() string {
	return fmt.Sprintf("whaleapi error, httpStatus:%d code:%d message:%s trace:%s", ae.HttpStatus, ae.Code, ae.Message, ae.TraceID)
}

// NewError builds an ApiError from a response.
func NewError(httpStatus int, resp *apiResponse) error {
	return &ApiError{
		HttpStatus: httpStatus,
		Code:       resp.Code,
		Message:    resp.Message,
		TraceID:    resp.TraceID,
	}
}
