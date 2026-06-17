package com.longportwhale.trade;

public interface OrderChangedHandler {
    void onOrderChanged(PushOrderChanged orderChanged);
}
