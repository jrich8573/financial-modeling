use anyhow::{Context, Result};
use chrono::NaiveDate;
use std::io::Read;

use crate::types::FactorRecord;

/// Downloads and parses the Fama-French 5 factors (daily) + Momentum factor.
/// Returns merged daily factor records.
pub async fn fetch_factor_data(client: &reqwest::Client) -> Result<Vec<FactorRecord>> {
    let ff5_url = "https://mba.tuck.dartmouth.edu/pages/faculty/ken.french/ftp/F-F_Research_Data_5_Factors_2x3_daily_CSV.zip";
    let mom_url = "https://mba.tuck.dartmouth.edu/pages/faculty/ken.french/ftp/F-F_Momentum_Factor_daily_CSV.zip";

    let (ff5_data, mom_data) = tokio::try_join!(
        download_and_extract_csv(client, ff5_url),
        download_and_extract_csv(client, mom_url),
    )?;

    let ff5_records = parse_ff5_csv(&ff5_data)?;
    let mom_records = parse_momentum_csv(&mom_data)?;

    let merged = merge_factors(ff5_records, mom_records);
    println!("Loaded {} daily factor records", merged.len());
    Ok(merged)
}

async fn download_and_extract_csv(client: &reqwest::Client, url: &str) -> Result<String> {
    let bytes = client
        .get(url)
        .header("User-Agent", "financial-models-rust/0.1")
        .send()
        .await
        .with_context(|| format!("Failed to download {}", url))?
        .bytes()
        .await?;

    let cursor = std::io::Cursor::new(bytes);
    let mut archive = zip::ZipArchive::new(cursor).context("Failed to open ZIP archive")?;

    // Find the CSV file in the archive
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        if file.name().ends_with(".CSV") || file.name().ends_with(".csv") {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            return Ok(contents);
        }
    }

    anyhow::bail!("No CSV file found in ZIP archive from {}", url)
}

/// Parses FF 5-factor daily CSV.
/// Format: Date, Mkt-RF, SMB, HML, RMW, CMA, RF
/// Values are in percent (e.g., 0.05 means 0.05%).
fn parse_ff5_csv(data: &str) -> Result<Vec<(NaiveDate, f64, f64, f64, f64, f64, f64)>> {
    let mut records = Vec::new();
    let mut in_data = false;

    for line in data.lines() {
        let trimmed = line.trim();

        // Skip empty lines and headers
        if trimmed.is_empty() {
            if in_data {
                break; // End of daily data section (annual data follows)
            }
            continue;
        }

        // Detect start of data (first line with a date-like number)
        let first_field = trimmed.split(',').next().unwrap_or("").trim();
        if first_field.len() == 8 && first_field.chars().all(|c| c.is_ascii_digit()) {
            in_data = true;
        }

        if !in_data {
            continue;
        }

        let fields: Vec<&str> = trimmed.split(',').map(|s| s.trim()).collect();
        if fields.len() < 7 {
            continue;
        }

        let date = parse_date_yyyymmdd(fields[0]);
        let date = match date {
            Some(d) => d,
            None => continue,
        };

        let mkt_rf = fields[1].parse::<f64>().unwrap_or(0.0) / 100.0;
        let smb = fields[2].parse::<f64>().unwrap_or(0.0) / 100.0;
        let hml = fields[3].parse::<f64>().unwrap_or(0.0) / 100.0;
        let rmw = fields[4].parse::<f64>().unwrap_or(0.0) / 100.0;
        let cma = fields[5].parse::<f64>().unwrap_or(0.0) / 100.0;
        let rf = fields[6].parse::<f64>().unwrap_or(0.0) / 100.0;

        records.push((date, mkt_rf, smb, hml, rmw, cma, rf));
    }

    Ok(records)
}

/// Parses momentum factor daily CSV.
/// Format: Date, Mom
fn parse_momentum_csv(data: &str) -> Result<Vec<(NaiveDate, f64)>> {
    let mut records = Vec::new();
    let mut in_data = false;

    for line in data.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            if in_data {
                break;
            }
            continue;
        }

        let first_field = trimmed.split(',').next().unwrap_or("").trim();
        if first_field.len() == 8 && first_field.chars().all(|c| c.is_ascii_digit()) {
            in_data = true;
        }

        if !in_data {
            continue;
        }

        let fields: Vec<&str> = trimmed.split(',').map(|s| s.trim()).collect();
        if fields.len() < 2 {
            continue;
        }

        let date = match parse_date_yyyymmdd(fields[0]) {
            Some(d) => d,
            None => continue,
        };

        let mom = fields[1].parse::<f64>().unwrap_or(0.0) / 100.0;
        records.push((date, mom));
    }

    Ok(records)
}

fn merge_factors(
    ff5: Vec<(NaiveDate, f64, f64, f64, f64, f64, f64)>,
    mom: Vec<(NaiveDate, f64)>,
) -> Vec<FactorRecord> {
    use std::collections::HashMap;
    let mom_map: HashMap<NaiveDate, f64> = mom.into_iter().collect();

    ff5.into_iter()
        .filter_map(|(date, mkt_rf, smb, hml, rmw, cma, rf)| {
            let mom = mom_map.get(&date)?;
            Some(FactorRecord {
                date,
                mkt_rf,
                smb,
                hml,
                rmw,
                cma,
                mom: *mom,
                rf,
            })
        })
        .collect()
}

fn parse_date_yyyymmdd(s: &str) -> Option<NaiveDate> {
    if s.len() != 8 {
        return None;
    }
    let year = s[0..4].parse::<i32>().ok()?;
    let month = s[4..6].parse::<u32>().ok()?;
    let day = s[6..8].parse::<u32>().ok()?;
    NaiveDate::from_ymd_opt(year, month, day)
}
