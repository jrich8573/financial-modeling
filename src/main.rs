mod db;
mod dfa_funds;
mod factors;
mod models;
mod regression;
mod sp500;
mod stock_data;
mod types;

use anyhow::Result;
use comfy_table::{presets::UTF8_FULL, Cell, Color, ContentArrangement, Table};
use futures::stream::{self, StreamExt};
use std::sync::Arc;
use tokio::sync::Semaphore;

use crate::types::{FactorRecord, StockAnalysis, StockInfo};

async fn analyze_securities(
    label: &str,
    securities: Vec<StockInfo>,
    factors: &Arc<Vec<FactorRecord>>,
    client: &Arc<reqwest::Client>,
    semaphore: &Arc<Semaphore>,
) -> Vec<StockAnalysis> {
    let count = securities.len();
    println!(
        "  Fetching price data and computing models for {} {}...",
        count, label
    );

    let results: Vec<Option<StockAnalysis>> = stream::iter(securities.into_iter().enumerate())
        .map(|(i, info)| {
            let client = client.clone();
            let factors = factors.clone();
            let semaphore = semaphore.clone();
            let ticker = info.ticker.clone();

            async move {
                let _permit = semaphore.acquire().await.unwrap();

                if i > 0 && i % 5 == 0 {
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                }

                let prices = match stock_data::fetch_historical_prices(&client, &ticker, 1890).await
                {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("  [SKIP] {}: {}", ticker, e);
                        return None;
                    }
                };

                if prices.len() < 30 {
                    eprintln!(
                        "  [SKIP] {}: insufficient price data ({} days)",
                        ticker,
                        prices.len()
                    );
                    return None;
                }

                let returns = stock_data::compute_returns(&prices);

                let last_price = prices.last().map(|p| p.close).unwrap_or(0.0);

                match models::compute_stock_analysis(info, &returns, &factors) {
                    Some(mut analysis) => {
                        analysis.current_price = last_price;
                        if (i + 1) % 50 == 0 {
                            println!("  Processed {}/{}", i + 1, count);
                        }
                        Some(analysis)
                    }
                    None => {
                        eprintln!("  [SKIP] {}: model computation failed", ticker);
                        None
                    }
                }
            }
        })
        .buffer_unordered(10)
        .collect()
        .await;

    let mut analyses: Vec<StockAnalysis> = results.into_iter().flatten().collect();
    analyses.sort_by(|a, b| a.info.ticker.cmp(&b.info.ticker));
    println!("  Successfully analyzed {} {}\n", analyses.len(), label);
    analyses
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== S&P 500 & DFA Funds Financial Models Analysis ===\n");

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    // Step 1: Fetch S&P 500 constituents
    println!("[1/6] Fetching S&P 500 constituent list...");
    let stocks = sp500::fetch_sp500_constituents(&client).await?;

    // Step 2: Load DFA fund list
    println!("[2/6] Loading DFA fund list...");
    let dfa = dfa_funds::get_dfa_funds();
    println!("  Loaded {} DFA funds", dfa.len());

    // Step 3: Fetch Fama-French factor data
    println!("[3/6] Downloading Fama-French factor data...");
    let factors = factors::fetch_factor_data(&client).await?;
    let factors = Arc::new(factors);

    let semaphore = Arc::new(Semaphore::new(5));
    let client = Arc::new(client);

    // Step 4: Analyze S&P 500 stocks
    println!("[4/6] Analyzing S&P 500 stocks...");
    let stock_analyses = analyze_securities("stocks", stocks, &factors, &client, &semaphore).await;

    // Step 5: Analyze DFA funds
    println!("[5/6] Analyzing DFA funds...");
    let mut dfa_analyses = analyze_securities("funds", dfa, &factors, &client, &semaphore).await;

    // Step 6: Fetch AUM for DFA funds
    println!("[6/6] Fetching AUM data for DFA funds...");
    {
        let mut aum_count = 0usize;
        for analysis in &mut dfa_analyses {
            let _permit = semaphore.acquire().await.unwrap();
            if let Some(aum) = stock_data::fetch_aum(&client, &analysis.info.ticker).await {
                analysis.aum = Some(aum);
                aum_count += 1;
            }
        }
        println!("  Retrieved AUM for {} of {} funds\n", aum_count, dfa_analyses.len());
    }

    // Print S&P 500 tables
    println!("\n--- S&P 500 Results ---\n");
    print_capm_table(&stock_analyses);
    println!();
    print_ff6_table(&stock_analyses);
    println!();
    print_summary_table(&stock_analyses);

    // Print DFA tables
    println!("\n--- DFA Fund Results ---\n");
    print_capm_table(&dfa_analyses);
    println!();
    print_ff6_table(&dfa_analyses);
    println!();
    print_summary_table(&dfa_analyses);

    // Write JSON for the web frontend
    let json_dir = std::path::Path::new("web/public");
    std::fs::create_dir_all(json_dir)?;

    let json_path = json_dir.join("data.json");
    let json = serde_json::to_string_pretty(&stock_analyses)?;
    std::fs::write(&json_path, &json)?;
    println!("\nS&P 500 JSON written to {}", json_path.display());

    let dfa_json_path = json_dir.join("dfa_data.json");
    let dfa_json = serde_json::to_string_pretty(&dfa_analyses)?;
    std::fs::write(&dfa_json_path, &dfa_json)?;
    println!("DFA funds JSON written to {}", dfa_json_path.display());

    // Write to PostgreSQL if DATABASE_URL is set
    if let Ok(database_url) = std::env::var("DATABASE_URL") {
        println!("\n[DB] Connecting to PostgreSQL...");
        let db_client = db::connect(&database_url).await?;

        println!("[DB] Creating tables...");
        db::create_tables(&db_client).await?;

        println!("[DB] Writing S&P 500 analyses...");
        let sp500_count = db::insert_analyses(&db_client, "sp500", &stock_analyses).await?;
        println!("[DB] Wrote {} S&P 500 records", sp500_count);

        println!("[DB] Writing DFA fund analyses...");
        let dfa_count = db::insert_analyses(&db_client, "dfa", &dfa_analyses).await?;
        println!("[DB] Wrote {} DFA fund records", dfa_count);

        println!("[DB] Done — all data written to PostgreSQL");
    } else {
        println!("\nNo DATABASE_URL set — skipping PostgreSQL write");
    }

    Ok(())
}

fn print_capm_table(analyses: &[StockAnalysis]) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Ticker").fg(Color::Cyan),
            Cell::new("Company").fg(Color::Cyan),
            Cell::new("Sector").fg(Color::Cyan),
            Cell::new("CAPM β").fg(Color::Yellow),
            Cell::new("CAPM α (ann.)").fg(Color::Yellow),
            Cell::new("CAPM E(R)").fg(Color::Yellow),
            Cell::new("R²").fg(Color::Yellow),
        ]);

    for a in analyses {
        table.add_row(vec![
            Cell::new(&a.info.ticker),
            Cell::new(truncate(&a.info.company, 25)),
            Cell::new(truncate(&a.info.sector, 20)),
            Cell::new(format!("{:.3}", a.capm.beta)),
            Cell::new(format!("{:.2}%", a.capm.alpha * 100.0)),
            Cell::new(format!("{:.2}%", a.capm.expected_return * 100.0)),
            Cell::new(format!("{:.3}", a.capm.r_squared)),
        ]);
    }

    println!("=== CAPM Results ===");
    println!("{table}");
}

fn print_ff6_table(analyses: &[StockAnalysis]) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Ticker").fg(Color::Cyan),
            Cell::new("α (ann.)").fg(Color::Green),
            Cell::new("β Mkt").fg(Color::Green),
            Cell::new("β SMB").fg(Color::Green),
            Cell::new("β HML").fg(Color::Green),
            Cell::new("β RMW").fg(Color::Green),
            Cell::new("β CMA").fg(Color::Green),
            Cell::new("β Mom").fg(Color::Green),
            Cell::new("FF6 E(R)").fg(Color::Green),
            Cell::new("R²").fg(Color::Green),
        ]);

    for a in analyses {
        table.add_row(vec![
            Cell::new(&a.info.ticker),
            Cell::new(format!("{:.2}%", a.ff6.alpha * 100.0)),
            Cell::new(format!("{:.3}", a.ff6.beta_mkt)),
            Cell::new(format!("{:.3}", a.ff6.beta_smb)),
            Cell::new(format!("{:.3}", a.ff6.beta_hml)),
            Cell::new(format!("{:.3}", a.ff6.beta_rmw)),
            Cell::new(format!("{:.3}", a.ff6.beta_cma)),
            Cell::new(format!("{:.3}", a.ff6.beta_mom)),
            Cell::new(format!("{:.2}%", a.ff6.expected_return * 100.0)),
            Cell::new(format!("{:.3}", a.ff6.r_squared)),
        ]);
    }

    println!("=== Fama-French 6-Factor Model Results ===");
    println!("{table}");
}

fn print_summary_table(analyses: &[StockAnalysis]) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Ticker").fg(Color::Cyan),
            Cell::new("Company").fg(Color::Cyan),
            Cell::new("Sector").fg(Color::Cyan),
            Cell::new("Ann. Return").fg(Color::Magenta),
            Cell::new("Ann. Std Dev").fg(Color::Magenta),
            Cell::new("CAPM E(R)").fg(Color::Yellow),
            Cell::new("FF6 E(R)").fg(Color::Green),
            Cell::new("CAPM β").fg(Color::Yellow),
        ]);

    for a in analyses {
        table.add_row(vec![
            Cell::new(&a.info.ticker),
            Cell::new(truncate(&a.info.company, 25)),
            Cell::new(truncate(&a.info.sector, 20)),
            Cell::new(format!("{:.2}%", a.annualized_return * 100.0)),
            Cell::new(format!("{:.2}%", a.annualized_std_dev * 100.0)),
            Cell::new(format!("{:.2}%", a.capm.expected_return * 100.0)),
            Cell::new(format!("{:.2}%", a.ff6.expected_return * 100.0)),
            Cell::new(format!("{:.3}", a.capm.beta)),
        ]);
    }

    println!("=== Summary: Returns & Risk ===");
    println!("{table}");
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}
