export interface StockInfo {
  ticker: string;
  company: string;
  sector: string;
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

export interface StockAnalysis {
  info: StockInfo;
  current_price: number;
  capm: CAPMResult;
  ff6: FF6Result;
  annualized_std_dev: number;
  annualized_return: number;
  regression_detail: RegressionDetail;
}
