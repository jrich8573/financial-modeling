use nalgebra::{DMatrix, DVector};
use statrs::distribution::{ContinuousCDF, FisherSnedecor, StudentsT};

pub struct OLSResult {
    /// Coefficients: [intercept, β1, β2, ...]
    pub coefficients: Vec<f64>,
    /// Standard errors for each coefficient
    pub std_errors: Vec<f64>,
    /// t-statistics for each coefficient
    pub t_statistics: Vec<f64>,
    /// Two-tailed p-values for each coefficient
    pub p_values: Vec<f64>,
    /// R-squared
    pub r_squared: f64,
    /// Adjusted R-squared
    pub adj_r_squared: f64,
    /// F-statistic for overall model significance
    pub f_statistic: f64,
    /// p-value for the F-statistic
    pub f_p_value: f64,
    /// Standard error of the regression (root MSE)
    pub se_regression: f64,
    /// Number of observations
    pub n_observations: usize,
    /// Degrees of freedom for the model (number of regressors, excluding intercept)
    pub df_model: usize,
    /// Degrees of freedom for residuals (n - k - 1)
    pub df_residual: usize,
}

/// Performs OLS regression: y = α + β1*x1 + β2*x2 + ... + βk*xk
/// `x_columns` is a slice of column vectors (each is one factor's time series).
/// `y` is the dependent variable.
/// Returns intercept (alpha) followed by factor betas, plus full inference statistics.
pub fn ols_regression(y: &[f64], x_columns: &[&[f64]]) -> Option<OLSResult> {
    let n = y.len();
    if n == 0 || x_columns.is_empty() {
        return None;
    }

    for col in x_columns {
        if col.len() != n {
            return None;
        }
    }

    let k = x_columns.len(); // number of regressors (excluding intercept)
    let cols = k + 1; // +1 for intercept

    // Need at least k+2 observations for valid inference
    if n <= cols {
        return None;
    }

    let df_residual = n - cols;

    // Build X matrix: [1, x1, x2, ..., xk] for each observation
    let mut x_data = Vec::with_capacity(n * cols);
    for i in 0..n {
        x_data.push(1.0); // intercept
        for col in x_columns {
            x_data.push(col[i]);
        }
    }

    let x = DMatrix::from_row_slice(n, cols, &x_data);
    let y_vec = DVector::from_column_slice(y);

    // β = (X'X)^(-1) X'y
    let xt = x.transpose();
    let xtx = &xt * &x;
    let xtx_inv = xtx.try_inverse()?;
    let xty = &xt * &y_vec;
    let beta = &xtx_inv * &xty;

    // Residuals and sums of squares
    let y_mean = y.iter().sum::<f64>() / n as f64;
    let ss_tot: f64 = y.iter().map(|yi| (yi - y_mean).powi(2)).sum();
    let y_hat = &x * &beta;
    let ss_res: f64 = y
        .iter()
        .enumerate()
        .map(|(i, yi)| (yi - y_hat[i]).powi(2))
        .sum();
    let ss_reg = ss_tot - ss_res;

    let r_squared = if ss_tot > 0.0 {
        1.0 - (ss_res / ss_tot)
    } else {
        0.0
    };

    // Adjusted R-squared
    let adj_r_squared = 1.0 - (1.0 - r_squared) * ((n - 1) as f64) / (df_residual as f64);

    // Residual variance (σ²) and standard error of regression
    let sigma2 = ss_res / df_residual as f64;
    let se_regression = sigma2.sqrt();

    // Coefficient standard errors from diagonal of (X'X)^-1 * σ²
    let var_beta = &xtx_inv * sigma2;
    let std_errors: Vec<f64> = (0..cols).map(|j| var_beta[(j, j)].max(0.0).sqrt()).collect();

    // t-statistics and p-values
    let t_dist = StudentsT::new(0.0, 1.0, df_residual as f64).ok();
    let t_statistics: Vec<f64> = (0..cols)
        .map(|j| {
            if std_errors[j] > 0.0 {
                beta[j] / std_errors[j]
            } else {
                0.0
            }
        })
        .collect();

    let p_values: Vec<f64> = t_statistics
        .iter()
        .map(|&t| {
            if let Some(ref dist) = t_dist {
                2.0 * (1.0 - dist.cdf(t.abs()))
            } else {
                f64::NAN
            }
        })
        .collect();

    // F-statistic and its p-value
    let (f_statistic, f_p_value) = if k > 0 && df_residual > 0 {
        let f_stat = (ss_reg / k as f64) / (ss_res / df_residual as f64);
        let f_p = FisherSnedecor::new(k as f64, df_residual as f64)
            .map(|d| 1.0 - d.cdf(f_stat.max(0.0)))
            .unwrap_or(f64::NAN);
        (f_stat, f_p)
    } else {
        (0.0, 1.0)
    };

    let coefficients = beta.iter().copied().collect();

    Some(OLSResult {
        coefficients,
        std_errors,
        t_statistics,
        p_values,
        r_squared,
        adj_r_squared,
        f_statistic,
        f_p_value,
        se_regression,
        n_observations: n,
        df_model: k,
        df_residual,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() < tol
    }

    #[test]
    fn test_simple_linear_regression() {
        // y = 2 + 3*x (perfect fit)
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y: Vec<f64> = x.iter().map(|&xi| 2.0 + 3.0 * xi).collect();

        let result = ols_regression(&y, &[&x]).expect("regression should succeed");

        assert!(approx_eq(result.coefficients[0], 2.0, 1e-6), "intercept should be ~2.0, got {}", result.coefficients[0]);
        assert!(approx_eq(result.coefficients[1], 3.0, 1e-6), "slope should be ~3.0, got {}", result.coefficients[1]);
        assert!(approx_eq(result.r_squared, 1.0, 1e-6), "R² should be ~1.0 for perfect fit, got {}", result.r_squared);
        assert_eq!(result.n_observations, 5);
        assert_eq!(result.df_model, 1);
        assert_eq!(result.df_residual, 3);
    }

    #[test]
    fn test_multiple_regression() {
        // y = 1 + 2*x1 + 3*x2
        let x1 = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let x2 = vec![2.0, 1.0, 3.0, 2.0, 4.0, 3.0];
        let y: Vec<f64> = x1.iter().zip(x2.iter()).map(|(&a, &b)| 1.0 + 2.0 * a + 3.0 * b).collect();

        let result = ols_regression(&y, &[&x1, &x2]).expect("regression should succeed");

        assert!(approx_eq(result.coefficients[0], 1.0, 1e-6), "intercept should be ~1.0");
        assert!(approx_eq(result.coefficients[1], 2.0, 1e-6), "beta_x1 should be ~2.0");
        assert!(approx_eq(result.coefficients[2], 3.0, 1e-6), "beta_x2 should be ~3.0");
        assert!(approx_eq(result.r_squared, 1.0, 1e-6));
        assert_eq!(result.df_model, 2);
    }

    #[test]
    fn test_regression_with_noise() {
        // y ≈ 1 + 2*x with some noise
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let noise = vec![0.1, -0.2, 0.15, -0.1, 0.05, -0.15, 0.2, -0.05, 0.1, -0.1];
        let y: Vec<f64> = x.iter().zip(noise.iter()).map(|(&xi, &ni)| 1.0 + 2.0 * xi + ni).collect();

        let result = ols_regression(&y, &[&x]).expect("regression should succeed");

        assert!(approx_eq(result.coefficients[0], 1.0, 0.5), "intercept should be near 1.0");
        assert!(approx_eq(result.coefficients[1], 2.0, 0.1), "slope should be near 2.0");
        assert!(result.r_squared > 0.99, "R² should be high, got {}", result.r_squared);
    }

    #[test]
    fn test_too_few_observations() {
        let x = vec![1.0];
        let y = vec![2.0];
        assert!(ols_regression(&y, &[&x]).is_none(), "should fail with n <= k+1");
    }

    #[test]
    fn test_empty_input() {
        let y: Vec<f64> = vec![];
        let x: Vec<f64> = vec![];
        assert!(ols_regression(&y, &[&x]).is_none());
    }

    #[test]
    fn test_mismatched_lengths() {
        let y = vec![1.0, 2.0, 3.0];
        let x = vec![1.0, 2.0];
        assert!(ols_regression(&y, &[&x]).is_none());
    }
}
