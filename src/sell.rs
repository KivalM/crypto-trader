use binance::account::*;
use binance::api::*;

use binance::market::*;

use chrono::{DateTime, Utc};
use colored::*;

pub fn sell(token: &str, secret: &str, currency: &str, pair: &str) {
    // function to sell eth to usdt
    let now: DateTime<Utc> = Utc::now();
    let api_key = Some(token.into());
    let secret_key = Some(secret.into());
    let account: Account = Binance::new(api_key, secret_key);
    let amt = account.get_balance(currency);

    match amt {
        Ok(answer) => {
            let amount: f64 = answer.free.parse().unwrap();
            // some threshhold to ensure you have a decent balance of eth
            if amount > 0.004 {
                let market: Market = Binance::new(None, None);

                match market.get_price(pair) {
                    Ok(_btc_price) => {
                        match account.market_sell(pair, (amount * 1000.0).floor() / 1000.0) {
                            Ok(answer) => println!(
                                "[{}] {} {} {} :: {}",
                                now.to_rfc2822().bright_white(),
                                "[SELL]".bright_red(),
                                currency,
                                amount,
                                answer.status
                            ),
                            Err(e) => println!("Error: {:?}", e),
                        }
                    }
                    Err(e) => println!("Error: {:?}", e),
                }
            } else {
                println!("SELL FAILED :: {}", "[POOR]".yellow());
            }
        }

        Err(e) => {
            println!("ERROR1: {:?}", e);
        }
    }
}
