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

export interface StockAnalysis {
  info: StockInfo;
  current_price: number;
  capm: CAPMResult;
  ff6: FF6Result;
  annualized_std_dev: number;
  annualized_return: number;
}
