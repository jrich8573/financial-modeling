use nalgebra::{DMatrix, DVector};

pub struct OLSResult {
    /// Coefficients: [intercept, β1, β2, ...]
    pub coefficients: Vec<f64>,
    /// R-squared
    pub r_squared: f64,
}

/// Performs OLS regression: y = α + β1*x1 + β2*x2 + ... + βk*xk
/// `x_columns` is a slice of column vectors (each is one factor's time series).
/// `y` is the dependent variable.
/// Returns intercept (alpha) followed by factor betas.
pub fn ols_regression(y: &[f64], x_columns: &[&[f64]]) -> Option<OLSResult> {
    let n = y.len();
    if n == 0 || x_columns.is_empty() {
        return None;
    }

    // All columns must have the same length as y
    for col in x_columns {
        if col.len() != n {
            return None;
        }
    }

    let k = x_columns.len(); // number of regressors (excluding intercept)
    let cols = k + 1; // +1 for intercept

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

    // R-squared
    let y_mean = y.iter().sum::<f64>() / n as f64;
    let ss_tot: f64 = y.iter().map(|yi| (yi - y_mean).powi(2)).sum();
    let y_hat = &x * &beta;
    let ss_res: f64 = y
        .iter()
        .enumerate()
        .map(|(i, yi)| (yi - y_hat[i]).powi(2))
        .sum();

    let r_squared = if ss_tot > 0.0 {
        1.0 - (ss_res / ss_tot)
    } else {
        0.0
    };

    let coefficients = beta.iter().copied().collect();

    Some(OLSResult {
        coefficients,
        r_squared,
    })
}
