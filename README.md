# Binance Trader

This is an auto trading bot for the binance.com exchange

# How it works

A simple rule governs cryptocurrency; Everything follows Bitcoin. When Bitcoin dips, they all dip; when Bitcoin spikes, they all do. The critical factor is the time offset.

So if everything follows Bitcoin with some delay, we can pretty much use Bitcoin's statistics to determine the best next move with another cryptocurrency of our choice.

All of this is done by trend trading with specific thresholds set on Bitcoin's price. If Bitcoin's price goes up x % in y minutes, we can assume that our crypto will be on an upward trend, thus purchasing it. However, if it goes down z % in t minutes, we can consider it downward and sell it.

The time offset helps us maximise profits while minimising losses. It's pretty much like predicting the future.

## However

The problem occurs when determining the optimal values for x and y that maximise the profit on the rise and the optimal z and t values that minimise losses on a dip. However, computers today are powerful, and we can simulate transactions based on past data to determine the best values. (See /src/test/README.md)

The Optimal values i have discovered based on the last 6 months of data are as follows:

- UP = 0.16%;
- DOWN= -1.15%;
- TIME_UP = 151;(TIME IN 5m Intervals)
- TIME_DOWN = 168;

These lead to an approximate return of 845.5% within the period of January to July 2021. To put it in perspective, if you bought ETH this year at its lowest (750 USD) and sold at its highest(~4000 USD). It is only a return of 533.3%

# Binance Setup

- Create a [Binance Account](http://binance.com/)
- Enable Two-factor Authentication.
- Create a new API key.
- Get a cryptocurrency, and edit the values in main.rs to reflect the currency of your choice.
- By default the program works with ETH and USDT with BTC as the determining factor. These can be adjusted to whatever you choose

# Program Setup

- Install [Rust](https://www.rust-lang.org/tools/install)
- Enter the program directory within a terminal
- Run `cargo run --release`
- Have it running on a dedicated device forever. AWS free tier works well due to the program's light footprint
