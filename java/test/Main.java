import com.longportwhale.*;
import com.longportwhale.trade.*;

public class Main {
    public static void main(String[] args) throws Exception {
        try (Config conf = Config.fromEnv(); TradeContext tctx = TradeContext.create(conf).get(); HttpClient cli = HttpClient.fromEnv()) {
          // do sth 
        }
    }
}
