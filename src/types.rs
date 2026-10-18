use chrono::NaiveDate;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct StockInfo {
    pub ticker: String,
    pub company: String,
    pub sector: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub benchmark: Option<String>,
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
pub struct CoefficientDetail {
    pub name: String,
    pub value: f64,
    pub std_error: f64,
    pub t_statistic: f64,
    pub p_value: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct DetailedRegressionResult {
    pub model_name: String,
    pub coefficients: Vec<CoefficientDetail>,
    pub r_squared: f64,
    pub adj_r_squared: f64,
    pub f_statistic: f64,
    pub f_p_value: f64,
    pub se_regression: f64,
    pub n_observations: usize,
    pub df_model: usize,
    pub df_residual: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct RatioStats {
    pub sharpe_ratio: f64,
    pub information_ratio: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct RegressionDetail {
    pub benchmark: DetailedRegressionResult,
    pub capm: DetailedRegressionResult,
    pub ff3: DetailedRegressionResult,
    pub ff5: DetailedRegressionResult,
    pub ff6: DetailedRegressionResult,
    pub ratios: RatioStats,
}

#[derive(Debug, Clone, Serialize)]
pub struct StockAnalysis {
    pub info: StockInfo,
    pub current_price: f64,
    pub capm: CAPMResult,
    pub ff6: FF6Result,
    pub annualized_std_dev: f64,
    pub annualized_return: f64,
    pub regression_detail: RegressionDetail,
}
