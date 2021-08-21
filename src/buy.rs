use binance::account::*;
use binance::api::*;

use binance::market::*;

use chrono::{DateTime, Utc};
use colored::*;

pub fn buy(token: &str, secret: &str, fiat: &str, pair: &str) {
    // function to buy eth from usdt
    let now: DateTime<Utc> = Utc::now();
    let api_key = Some(token.into());
    let secret_key = Some(secret.into());
    let account: Account = Binance::new(api_key, secret_key);
    let amt = account.get_balance(fiat);

    match amt {
        Ok(answer) => {
            let amount: f64 = answer.free.parse().unwrap();
            // some threshold to ensure you have a decent balance of usdt
            if amount > 50.0 {
                let market: Market = Binance::new(None, None);

                match market.get_price(pair) {
                    Ok(btc_price) => {
                        let rounded = ((amount / btc_price.price) * 1000.0).round() / 1000.0;
                        match account.market_buy(pair, rounded) {
                            Ok(answer) => println!(
                                "[{}] {} ${} :: {}",
                                now.to_rfc2822().bright_white(),
                                "[BUY]".bright_green(),
                                amount,
                                answer.status
                            ),
                            Err(e) => println!("Error: {:?}", e),
                        }
                    }
                    Err(e) => println!("Error: {:?}", e),
                }
            } else {
                println!(
                    "[{}] {}",
                    now.to_rfc2822().bright_white(),
                    "[POOR]".bright_purple()
                );
            }
        }

        Err(e) => {
            println!("ERROR1: {:?}", e);
        }
    }
}
