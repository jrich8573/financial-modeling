use std::sync::Arc;

use anyhow::{Context, Result};
use rustls::ClientConfig;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::pem::PemObject;
use tokio_postgres::Client;
use tokio_postgres_rustls::MakeRustlsConnect;

use crate::types::StockAnalysis;

/// Environment variable naming a PEM file of extra CA certificates to trust
/// (e.g. the AWS RDS bundle or a self-signed server CA).
pub const CA_CERT_ENV: &str = "DATABASE_CA_CERT";

/// Connect to PostgreSQL, negotiating TLS according to the URL's `sslmode`.
///
/// - `sslmode=disable` — plaintext only.
/// - `sslmode=prefer` (default) — TLS if the server offers it, else plaintext.
/// - `sslmode=require` — TLS mandatory; the connection fails otherwise.
///
/// Whenever TLS is used, the server certificate and hostname are verified
/// against the Mozilla root store plus any CAs in `$DATABASE_CA_CERT`.
pub async fn connect(database_url: &str) -> Result<Client> {
    let ca_path = std::env::var(CA_CERT_ENV).ok().filter(|p| !p.is_empty());
    let tls = make_tls_connector(ca_path.as_deref())?;

    let (client, connection) = tokio_postgres::connect(database_url, tls)
        .await
        .context("failed to connect to PostgreSQL")?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("DB connection error: {}", e);
        }
    });

    Ok(client)
}

/// Build a rustls connector trusting webpki roots plus an optional PEM CA bundle.
fn make_tls_connector(ca_path: Option<&str>) -> Result<MakeRustlsConnect> {
    let mut roots = rustls::RootCertStore::empty();
    roots.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

    if let Some(path) = ca_path {
        let certs = CertificateDer::pem_file_iter(path)
            .with_context(|| format!("failed to read {CA_CERT_ENV} file {path}"))?
            .collect::<Result<Vec<_>, _>>()
            .with_context(|| format!("invalid PEM in {CA_CERT_ENV} file {path}"))?;
        anyhow::ensure!(!certs.is_empty(), "no certificates found in {CA_CERT_ENV} file {path}");
        for cert in certs {
            roots
                .add(cert)
                .with_context(|| format!("invalid CA certificate in {path}"))?;
        }
    }

    // Pin the ring provider explicitly so we never depend on process-wide
    // provider selection (which panics if multiple providers are compiled in).
    let config = ClientConfig::builder_with_provider(Arc::new(
        rustls::crypto::ring::default_provider(),
    ))
    .with_safe_default_protocol_versions()
    .context("failed to configure TLS protocol versions")?
    .with_root_certificates(roots)
    .with_no_client_auth();

    Ok(MakeRustlsConnect::new(config))
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
                regression_detail JSONB,
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
                ff6_expected_return, ff6_r_squared,
                regression_detail
            ) VALUES (
                $1, $2, $3, $4, $5,
                $6, $7,
                $8, $9, $10, $11,
                $12, $13, $14, $15,
                $16, $17, $18,
                $19, $20,
                $21
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
                    &serde_json::to_value(&a.regression_detail).unwrap_or_default(),
                ],
            )
            .await?;
        count += 1;
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_connector_with_default_roots() {
        assert!(make_tls_connector(None).is_ok());
    }

    #[test]
    fn builds_connector_with_custom_ca() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/test-ca.pem");
        assert!(make_tls_connector(Some(path)).is_ok());
    }

    #[test]
    fn missing_ca_file_is_an_error() {
        let err = make_tls_connector(Some("/nonexistent/ca.pem")).err().unwrap();
        assert!(err.to_string().contains(CA_CERT_ENV));
    }

    #[test]
    fn empty_ca_file_is_an_error() {
        let dir = std::env::temp_dir().join(format!("db_tls_empty_{}", std::process::id()));
        std::fs::write(&dir, "not a cert\n").unwrap();
        let res = make_tls_connector(dir.to_str());
        std::fs::remove_file(&dir).ok();
        assert!(res.is_err());
    }
}
