use chrono::NaiveDate;
use std::collections::HashMap;

use crate::regression::ols_regression;
use crate::types::{CAPMResult, FF6Result, FactorRecord, ReturnRecord, StockAnalysis, StockInfo};

pub fn compute_stock_analysis(
    info: StockInfo,
    returns: &[ReturnRecord],
    factors: &[FactorRecord],
) -> Option<StockAnalysis> {
    // Build a date-indexed factor map
    let factor_map: HashMap<NaiveDate, &FactorRecord> =
        factors.iter().map(|f| (f.date, f)).collect();

    // Align stock returns with factor dates
    let mut excess_returns = Vec::new();
    let mut mkt_rf = Vec::new();
    let mut smb = Vec::new();
    let mut hml = Vec::new();
    let mut rmw = Vec::new();
    let mut cma = Vec::new();
    let mut mom = Vec::new();

    for ret in returns {
        if let Some(factor) = factor_map.get(&ret.date) {
            excess_returns.push(ret.daily_return - factor.rf);
            mkt_rf.push(factor.mkt_rf);
            smb.push(factor.smb);
            hml.push(factor.hml);
            rmw.push(factor.rmw);
            cma.push(factor.cma);
            mom.push(factor.mom);
        }
    }

    if excess_returns.len() < 30 {
        return None; // Not enough data points for meaningful regression
    }

    // CAPM: excess_return = α + β * mkt_rf
    let capm_result = ols_regression(&excess_returns, &[&mkt_rf])?;
    let capm_alpha = capm_result.coefficients[0];
    let capm_beta = capm_result.coefficients[1];

    // Average daily market premium and risk-free rate for expected return calc
    let avg_mkt_rf = mkt_rf.iter().sum::<f64>() / mkt_rf.len() as f64;
    let avg_rf = returns
        .iter()
        .filter_map(|r| factor_map.get(&r.date).map(|f| f.rf))
        .sum::<f64>()
        / excess_returns.len() as f64;

    // Annualize: daily average * 252
    let capm_expected_return = (avg_rf + capm_beta * avg_mkt_rf) * 252.0;

    let capm = CAPMResult {
        alpha: capm_alpha * 252.0, // annualize alpha
        beta: capm_beta,
        expected_return: capm_expected_return,
        r_squared: capm_result.r_squared,
    };

    // FF6: excess_return = α + β1*mkt_rf + β2*smb + β3*hml + β4*rmw + β5*cma + β6*mom
    let ff6_result = ols_regression(
        &excess_returns,
        &[&mkt_rf, &smb, &hml, &rmw, &cma, &mom],
    )?;

    let ff6_alpha = ff6_result.coefficients[0];
    let ff6_beta_mkt = ff6_result.coefficients[1];
    let ff6_beta_smb = ff6_result.coefficients[2];
    let ff6_beta_hml = ff6_result.coefficients[3];
    let ff6_beta_rmw = ff6_result.coefficients[4];
    let ff6_beta_cma = ff6_result.coefficients[5];
    let ff6_beta_mom = ff6_result.coefficients[6];

    // Average daily factor premiums
    let avg_smb = smb.iter().sum::<f64>() / smb.len() as f64;
    let avg_hml = hml.iter().sum::<f64>() / hml.len() as f64;
    let avg_rmw = rmw.iter().sum::<f64>() / rmw.len() as f64;
    let avg_cma = cma.iter().sum::<f64>() / cma.len() as f64;
    let avg_mom = mom.iter().sum::<f64>() / mom.len() as f64;

    let ff6_expected_return = (avg_rf
        + ff6_beta_mkt * avg_mkt_rf
        + ff6_beta_smb * avg_smb
        + ff6_beta_hml * avg_hml
        + ff6_beta_rmw * avg_rmw
        + ff6_beta_cma * avg_cma
        + ff6_beta_mom * avg_mom)
        * 252.0;

    let ff6 = FF6Result {
        alpha: ff6_alpha * 252.0,
        beta_mkt: ff6_beta_mkt,
        beta_smb: ff6_beta_smb,
        beta_hml: ff6_beta_hml,
        beta_rmw: ff6_beta_rmw,
        beta_cma: ff6_beta_cma,
        beta_mom: ff6_beta_mom,
        expected_return: ff6_expected_return,
        r_squared: ff6_result.r_squared,
    };

    // Annualized standard deviation of daily returns
    let daily_returns: Vec<f64> = returns.iter().map(|r| r.daily_return).collect();
    let mean_return = daily_returns.iter().sum::<f64>() / daily_returns.len() as f64;
    let variance =
        daily_returns.iter().map(|r| (r - mean_return).powi(2)).sum::<f64>()
            / (daily_returns.len() as f64 - 1.0);
    let annualized_std_dev = variance.sqrt() * (252.0_f64).sqrt();

    // Annualized return (geometric)
    let total_return: f64 = daily_returns.iter().map(|r| 1.0 + r).product();
    let annualized_return = total_return.powf(252.0 / daily_returns.len() as f64) - 1.0;

    Some(StockAnalysis {
        info,
        current_price: 0.0, // set by caller from price data
        capm,
        ff6,
        annualized_std_dev,
        annualized_return,
    })
}
