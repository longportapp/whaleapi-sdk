package com.longportwhale;

public class OpenApiException extends Exception {
    private Long code;
    private String message;
    private String traceId;

    public OpenApiException(Long code, String message, String traceId) {
        this.code = code;
        this.message = message;
        this.traceId = traceId;
    }

    public Long getCode() {
        return code;
    }

    public String getMessage() {
        return message;
    }
  
    
    public String getTraceId() {
        return traceId;
    }

    @Override
    public String toString() {
        return "OpenApiException [code=" + code + ", message=" + message + ", trace-id=" + traceId + "]";
    }
}
