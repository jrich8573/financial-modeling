use anyhow::{Context, Result};
use scraper::{Html, Selector};

use crate::types::StockInfo;

pub async fn fetch_sp500_constituents(client: &reqwest::Client) -> Result<Vec<StockInfo>> {
    let url = "https://en.wikipedia.org/wiki/List_of_S%26P_500_companies";
    let html = client
        .get(url)
        .header("User-Agent", "financial-models-rust/0.1")
        .send()
        .await
        .context("Failed to fetch S&P 500 list from Wikipedia")?
        .text()
        .await?;

    let document = Html::parse_document(&html);
    let table_selector = Selector::parse("table.wikitable tbody tr").unwrap();
    let td_selector = Selector::parse("td").unwrap();
    let a_selector = Selector::parse("a").unwrap();

    let mut stocks = Vec::new();

    for row in document.select(&table_selector) {
        let cells: Vec<_> = row.select(&td_selector).collect();
        if cells.len() < 5 {
            continue;
        }

        let ticker = cells[0]
            .select(&a_selector)
            .next()
            .map(|a| a.text().collect::<String>())
            .unwrap_or_else(|| cells[0].text().collect::<String>())
            .trim()
            .to_string();

        if ticker.is_empty() {
            continue;
        }

        let company = cells[1]
            .select(&a_selector)
            .next()
            .map(|a| a.text().collect::<String>())
            .unwrap_or_else(|| cells[1].text().collect::<String>())
            .trim()
            .to_string();

        let sector = cells[3].text().collect::<String>().trim().to_string();

        // Normalize tickers: Wikipedia uses dots (BRK.B) but Yahoo uses dashes (BRK-B)
        let ticker = ticker.replace('.', "-");

        stocks.push(StockInfo {
            ticker,
            company,
            sector,
        });
    }

    if stocks.is_empty() {
        anyhow::bail!("Failed to parse any stocks from Wikipedia");
    }

    println!("Fetched {} S&P 500 constituents", stocks.len());
    Ok(stocks)
}
