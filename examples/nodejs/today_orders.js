const { Config, TradeContext } = require("longbridge");
let config = Config.fromEnv();
let ctx = new TradeContext(config);

ctx
  .open()
  .then(() => ctx.todayOrders())
  .then((resp) => {
    for (let obj of resp) {
      console.log(obj.toString());
    }
  });