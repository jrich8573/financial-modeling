use chrono::NaiveDate;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct StockInfo {
    pub ticker: String,
    pub company: String,
    pub sector: String,
}

#[derive(Debug, Clone)]
pub struct PriceRecord {
    pub date: NaiveDate,
    pub close: f64,
}

#[derive(Debug, Clone)]
pub struct ReturnRecord {
    pub date: NaiveDate,
    pub daily_return: f64,
}

#[derive(Debug, Clone)]
pub struct FactorRecord {
    pub date: NaiveDate,
    pub mkt_rf: f64,
    pub smb: f64,
    pub hml: f64,
    pub rmw: f64,
    pub cma: f64,
    pub mom: f64,
    pub rf: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct CAPMResult {
    pub alpha: f64,
    pub beta: f64,
    pub expected_return: f64,
    pub r_squared: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct FF6Result {
    pub alpha: f64,
    pub beta_mkt: f64,
    pub beta_smb: f64,
    pub beta_hml: f64,
    pub beta_rmw: f64,
    pub beta_cma: f64,
    pub beta_mom: f64,
    pub expected_return: f64,
    pub r_squared: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct StockAnalysis {
    pub info: StockInfo,
    pub current_price: f64,
    pub capm: CAPMResult,
    pub ff6: FF6Result,
    pub annualized_std_dev: f64,
    pub annualized_return: f64,
}
