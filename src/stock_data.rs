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

/// Fetch total net assets (AUM) for a fund/ETF by scraping Yahoo Finance.
/// The page embeds JSON with totalAssets. Returns AUM in dollars, or None.
pub async fn fetch_aum(client: &reqwest::Client, ticker: &str) -> Option<f64> {
    let url = format!("https://finance.yahoo.com/quote/{}/", ticker);

    let resp = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)")
        .send()
        .await
        .ok()?;

    if !resp.status().is_success() {
        return None;
    }

    let body = resp.text().await.ok()?;

    // Look for: totalAssets":{"raw":38353334272,
    if let Some(pos) = body.find("totalAssets") {
        let after = &body[pos..];
        if let Some(raw_start) = after.find("\"raw\":") {
            let num_start = pos + raw_start + 6; // skip past "raw":
            let num_str = &body[num_start..];
            let end = num_str.find(|c: char| !c.is_ascii_digit() && c != '.').unwrap_or(num_str.len());
            if let Ok(val) = num_str[..end].parse::<f64>() {
                if val > 0.0 {
                    return Some(val);
                }
            }
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
