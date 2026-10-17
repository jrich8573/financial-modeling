# S&P 500 & DFA Funds Financial Models

A Rust CLI application with a React web frontend that analyzes S&P 500 stocks and Dimensional Fund Advisors (DFA) funds. Computes CAPM, Fama-French 6-Factor models, expected returns, and volatility for each security using live market data.

## What It Does

1. **Scrapes the S&P 500 stock list** from Wikipedia, capturing each stock's ticker, company name, and sector.

2. **Loads ~70 DFA funds** (mutual funds and ETFs) across US equity, international, emerging markets, global equity, fixed income, and municipal bonds.

3. **Downloads Fama-French factor data** from Kenneth French's Data Library (Dartmouth). This includes the 5 factors (Market, SMB, HML, RMW, CMA) plus the Momentum factor, as well as the daily risk-free rate (1-month T-bill).

4. **Fetches ~1 year of daily closing prices** for each security from the Yahoo Finance API. Computes daily returns from the price series.

5. **Runs financial models** for each security:

   - **CAPM (Capital Asset Pricing Model)** — Regresses excess returns against the market excess return to compute beta. Expected return: `Rf + β × (Rm - Rf)`.

   - **Fama-French 6-Factor Model** — Multivariate OLS regression on six factors:
     - **Mkt-RF** — Market excess return
     - **SMB** — Small Minus Big (size)
     - **HML** — High Minus Low (value)
     - **RMW** — Robust Minus Weak (profitability)
     - **CMA** — Conservative Minus Aggressive (investment)
     - **Mom** — Momentum (UMD)

   - **Expected Returns** — Annualized expected returns from both CAPM and FF6, plus the realized annualized geometric return.

   - **Annualized Standard Deviation** — Daily return volatility scaled by √252.

6. **Outputs formatted tables** to the terminal and writes JSON data files for the web frontend.

## Web Frontend

A React (Vite + TypeScript) static site that displays the analysis results with:

- **Page navigation** — switch between S&P 500 and DFA Funds views
- **Tabbed views** — Summary, CAPM, Fama-French 6-Factor, and Data Dictionary
- **Sortable columns** — click any header to sort ascending/descending
- **Search** — filter by ticker or company/fund name
- **Sector/Category filter** — dropdown to narrow by sector or fund category
- **Stats bar** — aggregate metrics (count, avg beta, avg return, avg volatility)
- **Linked tickers** — each ticker links to that company's or fund's investor relations page
- **Data Dictionary** — reference for all metrics, formulas, and methodology

## Requirements

- **Rust** (stable toolchain) — install via [rustup](https://rustup.rs/)
- **Node.js** (v20+) — for the web frontend
- **Internet connection** — the app fetches live data from Wikipedia, Yahoo Finance, and the French Data Library

No API keys are required.

## How to Run

### Generate data (Rust CLI)

```bash
cargo build --release
cargo run --release
```

This fetches live data and writes `web/public/data.json` (S&P 500) and `web/public/dfa_data.json` (DFA funds). The full run takes several minutes due to rate limiting across ~570 securities.

### Start the web frontend

```bash
cd web
npm install
npm run dev
```

### Build a static site

```bash
cd web
npm run build    # output in web/dist/
npm run preview  # preview locally
```

## Docker

The entire stack runs via Docker Compose:

```bash
docker compose up --build
```

This starts three services:

| Service | Description | Port |
|---------|-------------|------|
| `db` | PostgreSQL 17 (Alpine) | 5432 |
| `backend` | Rust CLI — fetches data, runs models, writes JSON + DB | — |
| `web` | Vite/React frontend served via nginx | 8080 |

The backend generates `data.json` and `dfa_data.json` into a shared volume, which nginx serves to the frontend. PostgreSQL stores the analysis results for querying.

## CI/CD

A GitHub Actions workflow (`.github/workflows/deploy-containers.yml`) automatically builds and pushes container images on every push to `main`:

| Image | Registry |
|-------|----------|
| Backend | `ghcr.io/jrich8573/financial-modeling/backend:latest` |
| Web | `ghcr.io/jrich8573/financial-modeling/web:latest` |

Each image is also tagged with the commit SHA. The workflow can be triggered manually via `workflow_dispatch`.

To pull and run the published images:

```bash
docker pull ghcr.io/jrich8573/financial-modeling/backend:latest
docker pull ghcr.io/jrich8573/financial-modeling/web:latest
```

## Project Structure

```
financial-models/
├── Cargo.toml
├── Dockerfile               # Multi-stage Rust backend image
├── docker-compose.yml       # Full stack: db + backend + web
├── .github/
│   └── workflows/
│       └── deploy-containers.yml  # CI: build & push to GHCR
├── src/
│   ├── main.rs          # Entry point, orchestration, table output, JSON export
│   ├── types.rs         # Shared data structures
│   ├── sp500.rs         # Scrapes S&P 500 constituent list from Wikipedia
│   ├── dfa_funds.rs     # DFA mutual fund and ETF catalog (~70 funds)
│   ├── stock_data.rs    # Fetches historical daily prices from Yahoo Finance
│   ├── factors.rs       # Downloads and parses Fama-French factor data
│   ├── regression.rs    # OLS regression using nalgebra
│   ├── db.rs            # PostgreSQL persistence
│   └── models.rs        # CAPM and FF6 model computation
└── web/
    ├── Dockerfile           # Multi-stage Node + nginx image
    ├── nginx.conf           # Serves frontend + JSON data
    ├── package.json
    ├── index.html
    ├── public/
    │   ├── data.json        # Generated S&P 500 analysis (git-ignored)
    │   └── dfa_data.json    # Generated DFA fund analysis (git-ignored)
    └── src/
        ├── main.tsx
        ├── App.tsx          # Page nav, analysis tables, data dictionary
        ├── types.ts         # TypeScript interfaces matching Rust structs
        └── index.css        # Dark theme styles
```

## Key Dependencies

### Rust

| Crate | Purpose |
|-------|---------|
| `reqwest` + `tokio` | Async HTTP requests |
| `scraper` | HTML parsing for Wikipedia |
| `nalgebra` | Matrix algebra for OLS regression |
| `csv` + `zip` | Parsing French Data Library CSV files from ZIP archives |
| `chrono` | Date handling |
| `comfy-table` | Formatted terminal tables |
| `serde` + `serde_json` | Serialization for Yahoo Finance responses and JSON export |

### Web

| Package | Purpose |
|---------|---------|
| `react` | UI components |
| `vite` | Build tool and dev server |
| `typescript` | Type safety |

## Notes

- Tickers with dots (e.g., `BRK.B`) are automatically converted to Yahoo Finance format (`BRK-B`).
- Securities with fewer than 30 overlapping data points between price history and factor data are skipped.
- Factor data values from the French Data Library are in percent and are converted to decimal form internally.
- All regressions use standard OLS: `β = (X'X)⁻¹X'y`.
- DFA fund categories include US Large Cap, US Small Cap Value, International, Emerging Markets, Fixed Income, and ETF variants.
