mod buy;
mod get;
mod sell;
mod test;

use buy::buy;
use get::check;
use get::get_kline_stats;
use sell::sell;

use chrono::{DateTime, Utc};
use colored::*;

use std::thread::sleep;
use std::time::Duration;

// change these values

const PERC_UP: f64 = 0.16;
const PERC_DOWN: f64 = -1.15;

// time measured in intervals of 5m
const TIME_UP: usize = 151;
const TIME_DOWN: usize = 168;

const TOKEN: &str = "TOKEN HERE";
const SECRET: &str = "SECRET HERE";

const DETERMINANT: &str = "BTCUSDT";
const FIAT: &str = "USDT";
const CURRENCY: &str = "ETH";
const PAIR: &str = "ETHUSDT";

// ^^^^^^^^^^^^^^^

fn nothing() {
    // self explanatory
}

fn mainloop() {
    let now: DateTime<Utc> = Utc::now();
    println!("[START] [{}]", now.to_rfc3339().bright_magenta());

    loop {
        let (curr, down, up) = get_kline_stats(TIME_DOWN, TIME_UP, DETERMINANT);

        if curr == -1.0 {
            // if failed to get result, try again next iter
            continue;
        }

        let res = check(curr, down, up, PERC_UP, PERC_DOWN);
        match res {
            1 => sell(TOKEN, SECRET, CURRENCY, PAIR),
            2 => buy(TOKEN, SECRET, FIAT, PAIR),
            _ => nothing(),
        }

        // wait 5 minutes before checking again
        sleep(Duration::from_secs(300));
    }
}
fn main() {
    mainloop()
    // test::run(DETERMINANT, PAIR, 1609459200000);
}
