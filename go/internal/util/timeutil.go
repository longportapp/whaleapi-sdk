package util

import "time"

const (
	SimpleDateLayout   = "20060102"
	DateLayout         = "2006-01-02"
	SimpleMinuteLayout = "1504"
)

// FormatDate formats a time as "2006-01-02".
func FormatDate(t *time.Time) string {
	if t == nil || t.IsZero() {
		return ""
	}
	return t.Format(DateLayout)
}
