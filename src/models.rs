use chrono::NaiveDate;
use std::collections::HashMap;

use crate::regression::{ols_regression, OLSResult};
use crate::types::{
    CAPMResult, CoefficientDetail, DetailedRegressionResult, FF6Result, FactorRecord, RatioStats,
    RegressionDetail, ReturnRecord, StockAnalysis, StockInfo,
};

fn build_detailed_result(
    ols: &OLSResult,
    model_name: &str,
    labels: &[&str],
) -> DetailedRegressionResult {
    let coefficients = labels
        .iter()
        .enumerate()
        .map(|(i, &name)| CoefficientDetail {
            name: name.to_string(),
            value: ols.coefficients[i],
            std_error: ols.std_errors[i],
            t_statistic: ols.t_statistics[i],
            p_value: ols.p_values[i],
        })
        .collect();

    DetailedRegressionResult {
        model_name: model_name.to_string(),
        coefficients,
        r_squared: ols.r_squared,
        adj_r_squared: ols.adj_r_squared,
        f_statistic: ols.f_statistic,
        f_p_value: ols.f_p_value,
        se_regression: ols.se_regression,
        n_observations: ols.n_observations,
        df_model: ols.df_model,
        df_residual: ols.df_residual,
    }
}

pub fn compute_stock_analysis(
    info: StockInfo,
    returns: &[ReturnRecord],
    factors: &[FactorRecord],
) -> Option<StockAnalysis> {
    // Build a date-indexed factor map
    let factor_map: HashMap<NaiveDate, &FactorRecord> =
        factors.iter().map(|f| (f.date, f)).collect();

    // Align stock returns with factor dates
    let mut raw_returns = Vec::new();
    let mut excess_returns = Vec::new();
    let mut mkt_rf = Vec::new();
    let mut mkt_total = Vec::new();
    let mut smb = Vec::new();
    let mut hml = Vec::new();
    let mut rmw = Vec::new();
    let mut cma = Vec::new();
    let mut mom = Vec::new();
    let mut rf_values = Vec::new();

    for ret in returns {
        if let Some(factor) = factor_map.get(&ret.date) {
            raw_returns.push(ret.daily_return);
            excess_returns.push(ret.daily_return - factor.rf);
            mkt_rf.push(factor.mkt_rf);
            mkt_total.push(factor.mkt_rf + factor.rf);
            smb.push(factor.smb);
            hml.push(factor.hml);
            rmw.push(factor.rmw);
            cma.push(factor.cma);
            mom.push(factor.mom);
            rf_values.push(factor.rf);
        }
    }

    if excess_returns.len() < 30 {
        return None;
    }

    // ===== Existing CAPM/FF6 for backward compatibility =====

    let capm_result = ols_regression(&excess_returns, &[&mkt_rf])?;
    let capm_alpha = capm_result.coefficients[0];
    let capm_beta = capm_result.coefficients[1];

    let avg_mkt_rf = mkt_rf.iter().sum::<f64>() / mkt_rf.len() as f64;
    let avg_rf = rf_values.iter().sum::<f64>() / rf_values.len() as f64;

    let capm_expected_return = (avg_rf + capm_beta * avg_mkt_rf) * 252.0;

    let capm = CAPMResult {
        alpha: capm_alpha * 252.0,
        beta: capm_beta,
        expected_return: capm_expected_return,
        r_squared: capm_result.r_squared,
    };

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

    // ===== Risk metrics =====

    let daily_returns: Vec<f64> = returns.iter().map(|r| r.daily_return).collect();
    let mean_return = daily_returns.iter().sum::<f64>() / daily_returns.len() as f64;
    let variance = daily_returns
        .iter()
        .map(|r| (r - mean_return).powi(2))
        .sum::<f64>()
        / (daily_returns.len() as f64 - 1.0);
    let annualized_std_dev = variance.sqrt() * (252.0_f64).sqrt();

    let total_return: f64 = daily_returns.iter().map(|r| 1.0 + r).product();
    let annualized_return = total_return.powf(252.0 / daily_returns.len() as f64) - 1.0;

    // ===== Detailed regressions for the new tab =====

    // Benchmark: raw returns vs total market return
    let benchmark_ols = ols_regression(&raw_returns, &[&mkt_total])?;
    let benchmark_detail =
        build_detailed_result(&benchmark_ols, "Benchmark", &["Alpha", "Market"]);

    // CAPM: excess returns vs Mkt-RF
    let capm_detail =
        build_detailed_result(&capm_result, "CAPM", &["Alpha", "Mkt-RF"]);

    // FF3: excess returns vs Mkt-RF, SMB, HML
    let ff3_ols = ols_regression(&excess_returns, &[&mkt_rf, &smb, &hml])?;
    let ff3_detail = build_detailed_result(
        &ff3_ols,
        "Fama-French 3-Factor",
        &["Alpha", "Mkt-RF", "SMB", "HML"],
    );

    // FF5: excess returns vs Mkt-RF, SMB, HML, RMW, CMA
    let ff5_ols = ols_regression(&excess_returns, &[&mkt_rf, &smb, &hml, &rmw, &cma])?;
    let ff5_detail = build_detailed_result(
        &ff5_ols,
        "Fama-French 5-Factor",
        &["Alpha", "Mkt-RF", "SMB", "HML", "RMW", "CMA"],
    );

    // FF6: excess returns vs all 6 factors (reuse existing result)
    let ff6_detail = build_detailed_result(
        &ff6_result,
        "Fama-French 6-Factor",
        &["Alpha", "Mkt-RF", "SMB", "HML", "RMW", "CMA", "Mom"],
    );

    // ===== Ratios =====

    let annualized_rf = avg_rf * 252.0;
    let sharpe_ratio = if annualized_std_dev > 0.0 {
        (annualized_return - annualized_rf) / annualized_std_dev
    } else {
        0.0
    };

    // Information ratio: annualized CAPM alpha / annualized tracking error
    // Tracking error = std dev of CAPM residuals, annualized
    let n_aligned = excess_returns.len();
    let capm_residuals: Vec<f64> = (0..n_aligned)
        .map(|i| {
            excess_returns[i] - (capm_result.coefficients[0] + capm_result.coefficients[1] * mkt_rf[i])
        })
        .collect();
    let resid_mean = capm_residuals.iter().sum::<f64>() / n_aligned as f64;
    let resid_var = capm_residuals
        .iter()
        .map(|r| (r - resid_mean).powi(2))
        .sum::<f64>()
        / (n_aligned as f64 - 1.0);
    let tracking_error = resid_var.sqrt() * (252.0_f64).sqrt();

    let information_ratio = if tracking_error > 0.0 {
        (capm_alpha * 252.0) / tracking_error
    } else {
        0.0
    };

    let ratios = RatioStats {
        sharpe_ratio,
        information_ratio,
    };

    let regression_detail = RegressionDetail {
        benchmark: benchmark_detail,
        capm: capm_detail,
        ff3: ff3_detail,
        ff5: ff5_detail,
        ff6: ff6_detail,
        ratios,
    };

    Some(StockAnalysis {
        info,
        current_price: 0.0,
        capm,
        ff6,
        annualized_std_dev,
        annualized_return,
        regression_detail,
        aum: None,
    })
}
