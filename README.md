# 🚀 Universal Binance Spot Analyzer

**On-chain technical analysis agent for UOMI Network**  
Analyzes any USDT trading pair on Binance Spot.  
Timeframe: 1h.  
No trading signals — only raw, verifiable data.

Created by: **@catsmile**  
For: **UOMI Network – The AI-Native Blockchain**

> *"Where AI meets decentralization"*

---

## ✅ Features
- Moving Averages: MA5, MA7, MA14, MA25, MA50
- MACD & Signal Line (12, 26, 9)
- RSI (14-period)
- 24h Price Change (%)
- 24h Volume (in USDT)
- Direct TradingView chart link
- Fully on-chain compatible with **OPoC** and **TEE Oracles**
- No opinions — only objective, verifiable technical data

---

## 🔧 Built for UOMI Infrastructure

This agent is designed to work seamlessly with UOMI’s core technologies:

| UOMI Feature | How This Agent Uses It |
|-------------|------------------------|
| **OPoC (Optimistic Proof of Computation)** | Results are verifiable — validators can challenge incorrect computations. |
| **TEE Oracles** | Fetches real-time klines and 24h stats from Binance API securely and trustlessly. |
| **TSS (Threshold Signature Scheme)** | Agent can be upgraded later to sign cross-chain transactions (e.g. DEX trades). |
| **EVM Compatible** | Built with Rust → WASM, but fully integrable with Solidity-based UOMI smart contracts. |
| **Autonomous NFT** | Each agent is an NFT with logic, capable of evolving and interacting across ecosystems. |

---

{
  "symbol": "JTOUSDT",
  "price": 2.774,
  "price_24h": 2.662,
  "volume_24h": 87654321,
  "klines_1h": [
    { "close": 2.650 },
    { "close": 2.660 },
    { "close": 2.670 },
    { "close": 2.690 },
    { "close": 2.710 },
    { "close": 2.730 },
    { "close": 2.750 },
    { "close": 2.774 }
  ]
}

Output Schema (JSON)

{
  "symbol": "JTOUSDT",
  "timeframe": "1h",
  "price": 2.774,
  "change_24h": 4.15,
  "volume_24h": 87654321,
  "ma5": 2.750,
  "ma7": 2.740,
  "ma14": 2.720,
  "ma25": 2.690,
  "ma50": 2.650,
  "macd": 0.048,
  "macd_signal": 0.047,
  "rsi": 68.5,
  "tradingview_url": "https://www.tradingview.com/chart/?symbol=BINANCE:JTOUSDT"
}

📄 License

MIT License – feel free to use, modify, and integrate into your dApps.
