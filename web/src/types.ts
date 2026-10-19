export interface StockInfo {
  ticker: string;
  company: string;
  sector: string;
  benchmark?: string;
}

export interface CAPMResult {
  alpha: number;
  beta: number;
  expected_return: number;
  r_squared: number;
}

export interface FF6Result {
  alpha: number;
  beta_mkt: number;
  beta_smb: number;
  beta_hml: number;
  beta_rmw: number;
  beta_cma: number;
  beta_mom: number;
  expected_return: number;
  r_squared: number;
}

export interface CoefficientDetail {
  name: string;
  value: number;
  std_error: number;
  t_statistic: number;
  p_value: number;
}

export interface DetailedRegressionResult {
  model_name: string;
  coefficients: CoefficientDetail[];
  r_squared: number;
  adj_r_squared: number;
  f_statistic: number;
  f_p_value: number;
  se_regression: number;
  n_observations: number;
  df_model: number;
  df_residual: number;
}

export interface RatioStats {
  sharpe_ratio: number;
  information_ratio: number;
}

export interface RegressionDetail {
  benchmark: DetailedRegressionResult;
  capm: DetailedRegressionResult;
  ff3: DetailedRegressionResult;
  ff5: DetailedRegressionResult;
  ff6: DetailedRegressionResult;
  ratios: RatioStats;
}

export interface AnnualRevenue {
  year: string;
  avg_aum: number | null;
  year_end_tna: number | null;
  net_flow: number | null;
  market_effect: number | null;
  mgmt_fee_pct: number | null;
  exp_ratio_pct: number | null;
  rev_mgmt_fee: number | null;
  rev_total_expense: number | null;
  months: number | null;
}

export interface FundAnalytics {
  ticker: string;
  fund_name: string;
  vehicle: string;
  benchmark: string;
  first_offer_dt: string;
  latest_tna: number | null;
  mgmt_fee_pct: number | null;
  gross_expense_ratio_pct: number | null;
  waiver_pct: number | null;
  net_expense_ratio_pct: number | null;
  fee_after_waiver_pct: number | null;
  sharpe_ratio: number | null;
  max_drawdown: number | null;
  annualized_return_1yr: number | null;
  annualized_return_3yr: number | null;
  annualized_return_5yr: number | null;
  annualized_return_10yr: number | null;
  std_dev_1yr: number | null;
  std_dev_3yr: number | null;
  std_dev_5yr: number | null;
  std_dev_10yr: number | null;
  total_months: number;
  annual_revenue: AnnualRevenue[];
  lifetime_rev_mgmt_fee: number | null;
  lifetime_rev_total_expense: number | null;
}

export interface StockAnalysis {
  info: StockInfo;
  current_price: number;
  capm: CAPMResult;
  ff6: FF6Result;
  annualized_std_dev: number;
  annualized_return: number;
  regression_detail: RegressionDetail;
  aum?: number;
}
