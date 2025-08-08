use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Input {
    symbol: String,
    price: f64,
    price_24h: f64,
    volume_24h: f64,
    klines_1h: Vec<Kline>,
}

#[derive(Deserialize)]
struct Kline {
    close: f64,
}

#[derive(Serialize)]
struct Output {
    symbol: String,
    timeframe: String,
    price: f64,
    change_24h: f64,
    volume_24h: f64,
    ma5: f64,
    ma7: f64,
    ma14: f64,
    ma25: f64,
    ma50: f64,
    macd: f64,
    macd_signal: f64,
    rsi: f64,
    tradingview_url: String,
}

#[wasm_bindgen]
pub fn reveal(input_json: &str) -> String {
    let error_response = serde_json::json!({
        "error": "Invalid input format"
    });

    let input: Input = match serde_json::from_str(input_json) {
        Ok(v) => v,
        Err(_) => return error_response.to_string(),
    };

    if input.klines_1h.is_empty() {
        return serde_json::json!({ "error": "No 1h kline data provided" }).to_string();
    }

    let closes: Vec<f64> = input.klines_1h.iter().map(|k| k.close).collect();
    let price = input.price;

    let ma5 = sma(&closes, 5);
    let ma7 = sma(&closes, 7);
    let ma14 = sma(&closes, 14);
    let ma25 = sma(&closes, 25);
    let ma50 = sma(&closes, 50);

    let (macd_line, signal_line) = macd(&closes, 12, 26, 9);

    let rsi_value = rsi(&closes, 14);

    let change_24h = ((price / input.price_24h - 1.0) * 10000.0).round() / 100.0;

    let tv_url = format!("https://www.tradingview.com/chart/?symbol=BINANCE:{}", input.symbol.to_uppercase());

    let output = Output {
        symbol: input.symbol,
        timeframe: "1h".to_string(),
        price,
        change_24h,
        volume_24h: input.volume_24h,
        ma5,
        ma7,
        ma14,
        ma25,
        ma50,
        macd: macd_line,
        macd_signal: signal_line,
        rsi: rsi_value,
        tradingview_url: tv_url,
    };

    match serde_json::to_string(&output) {
        Ok(json) => json,
        Err(_) => serde_json::json!({ "error": "Serialization failed" }).to_string(),
    }
}

fn sma(prices: &[f64], period: usize) -> f64 {
    if prices.len() < period { return 0.0; }
    let sum: f64 = prices[prices.len()-period..].iter().sum();
    sum / period as f64
}

fn ema(prices: &[f64], period: usize) -> f64 {
    if prices.len() < period { return 0.0; }
    let k = 2.0 / (period as f64 + 1.0);
    let mut ema = prices[0];
    for &price in &prices[1..] {
        ema = price * k + ema * (1.0 - k);
    }
    ema
}

fn macd(prices: &[f64], fast: usize, slow: usize, signal_period: usize) -> (f64, f64) {
    let ema_fast = ema(prices, fast);
    let ema_slow = ema(prices, slow);
    let macd_line = ema_fast - ema_slow;

    let mut signal = macd_line;
    let k = 2.0 / (signal_period as f64 + 1.0);
    for _ in 0..signal_period {
        signal = macd_line * k + signal * (1.0 - k);
    }

    (macd_line, signal)
}

fn rsi(prices: &[f64], period: usize) -> f64 {
    if prices.len() < period + 1 { return 50.0; }

    let changes: Vec<f64> = prices.windows(2).map(|w| w[1] - w[0]).collect();
    let gains: Vec<f64> = changes.iter().map(|&c| if c > 0.0 { c } else { 0.0 }).collect();
    let losses: Vec<f64> = changes.iter().map(|&c| if c < 0.0 { -c } else { 0.0 }).collect();

    let avg_gain = gains.iter().take(period).sum::<f64>() / period as f64;
    let avg_loss = losses.iter().take(period).sum::<f64>() / period as f64;

    if avg_loss == 0.0 { return 100.0; }
    let rs = avg_gain / avg_loss;
    100.0 - (100.0 / (1.0 + rs))
}
