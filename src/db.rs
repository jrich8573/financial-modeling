use anyhow::Result;
use tokio_postgres::{Client, NoTls};

use crate::types::StockAnalysis;

pub async fn connect(database_url: &str) -> Result<Client> {
    let (client, connection) = tokio_postgres::connect(database_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("DB connection error: {}", e);
        }
    });

    Ok(client)
}

pub async fn create_tables(client: &Client) -> Result<()> {
    client
        .batch_execute(
            "
            CREATE TABLE IF NOT EXISTS stock_analyses (
                id SERIAL PRIMARY KEY,
                dataset TEXT NOT NULL,
                ticker TEXT NOT NULL,
                company TEXT NOT NULL,
                sector TEXT NOT NULL,
                current_price DOUBLE PRECISION NOT NULL,
                annualized_return DOUBLE PRECISION NOT NULL,
                annualized_std_dev DOUBLE PRECISION NOT NULL,
                capm_alpha DOUBLE PRECISION NOT NULL,
                capm_beta DOUBLE PRECISION NOT NULL,
                capm_expected_return DOUBLE PRECISION NOT NULL,
                capm_r_squared DOUBLE PRECISION NOT NULL,
                ff6_alpha DOUBLE PRECISION NOT NULL,
                ff6_beta_mkt DOUBLE PRECISION NOT NULL,
                ff6_beta_smb DOUBLE PRECISION NOT NULL,
                ff6_beta_hml DOUBLE PRECISION NOT NULL,
                ff6_beta_rmw DOUBLE PRECISION NOT NULL,
                ff6_beta_cma DOUBLE PRECISION NOT NULL,
                ff6_beta_mom DOUBLE PRECISION NOT NULL,
                ff6_expected_return DOUBLE PRECISION NOT NULL,
                ff6_r_squared DOUBLE PRECISION NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                UNIQUE (dataset, ticker)
            );
            ",
        )
        .await?;

    Ok(())
}

pub async fn insert_analyses(
    client: &Client,
    dataset: &str,
    analyses: &[StockAnalysis],
) -> Result<usize> {
    // Clear previous data for this dataset
    client
        .execute(
            "DELETE FROM stock_analyses WHERE dataset = $1",
            &[&dataset],
        )
        .await?;

    let stmt = client
        .prepare(
            "INSERT INTO stock_analyses (
                dataset, ticker, company, sector, current_price,
                annualized_return, annualized_std_dev,
                capm_alpha, capm_beta, capm_expected_return, capm_r_squared,
                ff6_alpha, ff6_beta_mkt, ff6_beta_smb, ff6_beta_hml,
                ff6_beta_rmw, ff6_beta_cma, ff6_beta_mom,
                ff6_expected_return, ff6_r_squared
            ) VALUES (
                $1, $2, $3, $4, $5,
                $6, $7,
                $8, $9, $10, $11,
                $12, $13, $14, $15,
                $16, $17, $18,
                $19, $20
            )",
        )
        .await?;

    let mut count = 0;
    for a in analyses {
        client
            .execute(
                &stmt,
                &[
                    &dataset,
                    &a.info.ticker,
                    &a.info.company,
                    &a.info.sector,
                    &a.current_price,
                    &a.annualized_return,
                    &a.annualized_std_dev,
                    &a.capm.alpha,
                    &a.capm.beta,
                    &a.capm.expected_return,
                    &a.capm.r_squared,
                    &a.ff6.alpha,
                    &a.ff6.beta_mkt,
                    &a.ff6.beta_smb,
                    &a.ff6.beta_hml,
                    &a.ff6.beta_rmw,
                    &a.ff6.beta_cma,
                    &a.ff6.beta_mom,
                    &a.ff6.expected_return,
                    &a.ff6.r_squared,
                ],
            )
            .await?;
        count += 1;
    }

    Ok(count)
}
