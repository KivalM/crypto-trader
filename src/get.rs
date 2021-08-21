use std::cmp::max;

use binance::api::*;

use binance::market::*;

fn get_perc(old: f64, new: f64) -> f64 {
    let percent = ((new - old) / old) * 100.0;
    return percent;
}

pub fn get_kline_stats(time_down: usize, time_up: usize, determinant: &str) -> (f64, f64, f64) {
    // Gets the current price of the determinant, the price TIME_UP ago, and TIME_DOWN ago
    let market: Market = Binance::new(None, None);

    let max = max(time_up, time_down);
    let lim = (max + 1) as u16;

    match market.get_klines(determinant, "5m", lim, None, None) {
        Ok(klines) => match klines {
            binance::model::KlineSummaries::AllKlineSummaries(klines) => {
                let currprice = klines[max].to_owned().open;
                let downprice = klines[max - time_down].to_owned().open;
                let upprice = klines[max - time_up].to_owned().open;
                return (currprice, downprice, upprice);
            }
        },
        Err(e) => {
            println!("FAILED {:?}", e);
            return (-1.0, -1.0, -1.0);
        }
    }
}

pub fn check(curr: f64, down: f64, up: f64, perc_up: f64, perc_down: f64) -> u32 {
    // KEY:
    // buy = 2
    // sell = 1
    // none = 0

    let buy = get_perc(up, curr) >= perc_up;
    let sell = get_perc(down, curr) <= perc_down;

    // prints the current percentages to stderr
    eprintln!("[{}] [{}]", get_perc(down, curr), get_perc(up, curr));

    if buy && sell {
        return 0;
    } else if buy {
        return 2;
    } else if sell {
        return 1;
    }

    return 0;
}
