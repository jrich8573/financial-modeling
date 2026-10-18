use anyhow::{Context, Result};
use chrono::NaiveDate;

use crate::types::{PriceRecord, ReturnRecord};

pub async fn fetch_historical_prices(
    client: &reqwest::Client,
    ticker: &str,
    days_back: u32,
) -> Result<Vec<PriceRecord>> {
    let now = chrono::Utc::now().timestamp();
    let start = now - (days_back as i64 * 86400);

    let url = format!(
        "https://query1.finance.yahoo.com/v8/finance/chart/{}?period1={}&period2={}&interval=1d",
        ticker, start, now
    );

    let resp = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)")
        .send()
        .await
        .with_context(|| format!("Failed to fetch prices for {}", ticker))?;

    if !resp.status().is_success() {
        anyhow::bail!("Yahoo Finance returned {} for {}", resp.status(), ticker);
    }

    let body: serde_json::Value = resp.json().await?;

    let chart = body["chart"]["result"]
        .as_array()
        .and_then(|a| a.first())
        .with_context(|| format!("No chart data for {}", ticker))?;

    let timestamps = chart["timestamp"]
        .as_array()
        .with_context(|| format!("No timestamps for {}", ticker))?;

    let closes = chart["indicators"]["quote"]
        .as_array()
        .and_then(|a| a.first())
        .and_then(|q| q["close"].as_array())
        .with_context(|| format!("No close prices for {}", ticker))?;

    let mut prices = Vec::new();
    for (ts, close) in timestamps.iter().zip(closes.iter()) {
        if let (Some(ts), Some(close)) = (ts.as_i64(), close.as_f64()) {
            let date = chrono::DateTime::from_timestamp(ts, 0)
                .map(|dt| dt.date_naive())
                .unwrap_or_else(|| NaiveDate::from_ymd_opt(2000, 1, 1).unwrap());
            prices.push(PriceRecord { date, close });
        }
    }

    Ok(prices)
}

/// Fetch total net assets (AUM) for a fund/ETF from Yahoo Finance quote API.
/// Returns the AUM in dollars, or None if unavailable.
pub async fn fetch_aum(client: &reqwest::Client, ticker: &str) -> Option<f64> {
    let url = format!(
        "https://query1.finance.yahoo.com/v8/finance/chart/{}?interval=1d&range=1d",
        ticker
    );

    let resp = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)")
        .send()
        .await
        .ok()?;

    if !resp.status().is_success() {
        return None;
    }

    let body: serde_json::Value = resp.json().await.ok()?;
    let meta = &body["chart"]["result"].as_array()?.first()?["meta"];

    // Try totalAssets field (available for ETFs/funds)
    if let Some(assets) = meta["totalAssets"].as_f64() {
        if assets > 0.0 {
            return Some(assets);
        }
    }

    // Fallback: try netAssets
    if let Some(assets) = meta["netAssets"].as_f64() {
        if assets > 0.0 {
            return Some(assets);
        }
    }

    None
}

pub fn compute_returns(prices: &[PriceRecord]) -> Vec<ReturnRecord> {
    prices
        .windows(2)
        .filter_map(|w| {
            let prev = w[0].close;
            let curr = w[1].close;
            if prev > 0.0 {
                Some(ReturnRecord {
                    date: w[1].date,
                    daily_return: (curr - prev) / prev,
                })
            } else {
                None
            }
        })
        .collect()
}
