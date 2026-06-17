package http

import (
	"context"
	nhttp "net/http"
	"strconv"
	"time"

	"github.com/longportapp/whaleapi-sdk/go/internal/signer"
	"github.com/longportapp/whaleapi-sdk/go/internal/util"
)

const headerTimestamp = "x-timestamp"

var sign = &signer.Signer{}

// signature signs the request in place using the WhaleAPI HMAC-SHA256 scheme.
//
// NOTE: WhaleAPI expects the x-timestamp header in seconds (unlike some other
// LongPort endpoints which use milliseconds).
func signature(req *nhttp.Request, secret string, body []byte) error {
	if v := req.Header.Get(headerTimestamp); v == "" {
		req.Header.Set(headerTimestamp, strconv.FormatInt(time.Now().Unix(), 10))
	}

	req.Header.Set("x-api-signature", "HMAC-SHA256 SignedHeaders=authorization;x-api-key;x-timestamp")
	signstr, _, _, err := sign.Sign(context.Background(), util.UnsafeStringToBytes(secret), req, body)
	if err != nil {
		return err
	}
	req.Header.Set("x-api-signature", "HMAC-SHA256 SignedHeaders=authorization;x-api-key;x-timestamp, Signature="+signstr)
	return nil
}
