import com.longportwhale.*;
import com.longportwhale.trade.*;

class Main {
    public static void main(String[] args) throws Exception {
        try (TradeContext tctx = TradeContext.create(conf).get(); HttpClient cli = HttpClient.fromEnv()) {
          // do sth 
        }
    }
}
