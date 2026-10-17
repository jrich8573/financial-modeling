import { useEffect, useMemo, useState } from "react";
import type { StockAnalysis } from "./types";
import "./App.css";

type Page = "sp500" | "dfa";
type Tab = "summary" | "capm" | "ff6" | "dictionary";
type SortDir = "asc" | "desc";
interface SortState {
  key: string;
  dir: SortDir;
}

function pct(v: number): string {
  return (v * 100).toFixed(2) + "%";
}

function num(v: number, decimals = 3): string {
  return v.toFixed(decimals);
}

function usd(v: number): string {
  return "$" + v.toFixed(2);
}

function colorClass(v: number): string {
  if (v > 0.0001) return "positive";
  if (v < -0.0001) return "negative";
  return "";
}

function irUrl(company: string): string {
  return `https://www.google.com/search?btnI=1&q=${encodeURIComponent(company + " investor relations")}`;
}

function TickerCell({ ticker, company }: { ticker: string; company: string }) {
  return (
    <td className="ticker">
      <a href={irUrl(company)} target="_blank" rel="noopener noreferrer">
        {ticker}
      </a>
    </td>
  );
}

function getValue(stock: StockAnalysis, key: string): string | number {
  switch (key) {
    case "ticker":      return stock.info.ticker;
    case "company":     return stock.info.company;
    case "sector":      return stock.info.sector;
    case "price":       return stock.current_price ?? 0;
    case "capm_beta":   return stock.capm.beta;
    case "capm_alpha":  return stock.capm.alpha;
    case "capm_er":     return stock.capm.expected_return;
    case "capm_r2":     return stock.capm.r_squared;
    case "ff6_alpha":   return stock.ff6.alpha;
    case "ff6_mkt":     return stock.ff6.beta_mkt;
    case "ff6_smb":     return stock.ff6.beta_smb;
    case "ff6_hml":     return stock.ff6.beta_hml;
    case "ff6_rmw":     return stock.ff6.beta_rmw;
    case "ff6_cma":     return stock.ff6.beta_cma;
    case "ff6_mom":     return stock.ff6.beta_mom;
    case "ff6_er":      return stock.ff6.expected_return;
    case "ff6_r2":      return stock.ff6.r_squared;
    case "ann_return":  return stock.annualized_return;
    case "ann_std":     return stock.annualized_std_dev;
    default:            return "";
  }
}

function App() {
  const [page, setPage] = useState<Page>("sp500");
  const [sp500Data, setSp500Data] = useState<StockAnalysis[] | null>(null);
  const [dfaData, setDfaData] = useState<StockAnalysis[] | null>(null);
  const [sp500Error, setSp500Error] = useState<string | null>(null);
  const [dfaError, setDfaError] = useState<string | null>(null);

  useEffect(() => {
    fetch("/data.json")
      .then((r) => { if (!r.ok) throw new Error(`HTTP ${r.status}`); return r.json(); })
      .then(setSp500Data)
      .catch((e) => setSp500Error(e.message));

    fetch("/dfa_data.json")
      .then((r) => { if (!r.ok) throw new Error(`HTTP ${r.status}`); return r.json(); })
      .then(setDfaData)
      .catch((e) => setDfaError(e.message));
  }, []);

  return (
    <>
      <div className="page-nav">
        <button
          className={`page-nav-btn ${page === "sp500" ? "active" : ""}`}
          onClick={() => setPage("sp500")}
        >
          S&P 500
        </button>
        <button
          className={`page-nav-btn ${page === "dfa" ? "active" : ""}`}
          onClick={() => setPage("dfa")}
        >
          DFA Funds
        </button>
      </div>

      {page === "sp500" && (
        <AnalysisView
          title="S&P 500 Financial Models"
          subtitle="CAPM, Fama-French 6-Factor, expected returns & volatility"
          data={sp500Data}
          error={sp500Error}
          itemLabel="stocks"
          sectorLabel="Sector"
        />
      )}

      {page === "dfa" && (
        <AnalysisView
          title="DFA Funds Financial Models"
          subtitle="Dimensional Fund Advisors — CAPM, Fama-French 6-Factor, expected returns & volatility"
          data={dfaData}
          error={dfaError}
          itemLabel="funds"
          sectorLabel="Category"
        />
      )}
    </>
  );
}

function AnalysisView({
  title,
  subtitle,
  data,
  error,
  itemLabel,
  sectorLabel,
}: {
  title: string;
  subtitle: string;
  data: StockAnalysis[] | null;
  error: string | null;
  itemLabel: string;
  sectorLabel: string;
}) {
  const [tab, setTab] = useState<Tab>("summary");
  const [search, setSearch] = useState("");
  const [sectorFilter, setSectorFilter] = useState("all");
  const [sort, setSort] = useState<SortState>({ key: "ticker", dir: "asc" });

  const sectors = useMemo(() => {
    if (!data) return [];
    const s = new Set(data.map((d) => d.info.sector));
    return Array.from(s).sort();
  }, [data]);

  const filtered = useMemo(() => {
    if (!data) return [];
    let items = data;
    if (search) {
      const q = search.toLowerCase();
      items = items.filter(
        (s) =>
          s.info.ticker.toLowerCase().includes(q) ||
          s.info.company.toLowerCase().includes(q)
      );
    }
    if (sectorFilter !== "all") {
      items = items.filter((s) => s.info.sector === sectorFilter);
    }
    items = [...items].sort((a, b) => {
      const av = getValue(a, sort.key);
      const bv = getValue(b, sort.key);
      let cmp: number;
      if (typeof av === "number" && typeof bv === "number") {
        cmp = av - bv;
      } else {
        cmp = String(av).localeCompare(String(bv));
      }
      return sort.dir === "asc" ? cmp : -cmp;
    });
    return items;
  }, [data, search, sectorFilter, sort]);

  const handleSort = (key: string) => {
    setSort((prev) =>
      prev.key === key
        ? { key, dir: prev.dir === "asc" ? "desc" : "asc" }
        : { key, dir: "asc" }
    );
  };

  const avgBeta = useMemo(() => {
    if (!filtered.length) return 0;
    return filtered.reduce((s, d) => s + d.capm.beta, 0) / filtered.length;
  }, [filtered]);

  const avgReturn = useMemo(() => {
    if (!filtered.length) return 0;
    return filtered.reduce((s, d) => s + d.annualized_return, 0) / filtered.length;
  }, [filtered]);

  const avgStdDev = useMemo(() => {
    if (!filtered.length) return 0;
    return filtered.reduce((s, d) => s + d.annualized_std_dev, 0) / filtered.length;
  }, [filtered]);

  if (error) {
    return (
      <div className="error">
        <h2>Failed to load data</h2>
        <p>
          Run <code>cargo run --release</code> first to generate the data files,
          then start the dev server.
        </p>
        <p>Error: {error}</p>
      </div>
    );
  }

  if (!data) {
    return (
      <div className="loading">
        <div className="spinner" />
        <span>Loading {itemLabel} data...</span>
      </div>
    );
  }

  const SortArrow = ({ col }: { col: string }) => (
    <span className="sort-arrow">
      {sort.key === col ? (sort.dir === "asc" ? "\u25B2" : "\u25BC") : "\u25B2"}
    </span>
  );

  const thClass = (col: string) => (sort.key === col ? "sorted" : "");

  return (
    <>
      <h1>{title}</h1>
      <p className="subtitle">
        {subtitle} for {data.length} {itemLabel}
      </p>

      <div className="stats-bar">
        <div className="stat-card">
          <div className="label">{itemLabel.charAt(0).toUpperCase() + itemLabel.slice(1)}</div>
          <div className="value">{filtered.length}</div>
        </div>
        <div className="stat-card">
          <div className="label">Avg Beta</div>
          <div className="value">{num(avgBeta)}</div>
        </div>
        <div className="stat-card">
          <div className="label">Avg Ann. Return</div>
          <div className="value">{pct(avgReturn)}</div>
        </div>
        <div className="stat-card">
          <div className="label">Avg Volatility</div>
          <div className="value">{pct(avgStdDev)}</div>
        </div>
      </div>

      <div className="tabs">
        <button className={`tab ${tab === "summary" ? "active" : ""}`} onClick={() => setTab("summary")}>
          Summary
        </button>
        <button className={`tab ${tab === "capm" ? "active" : ""}`} onClick={() => setTab("capm")}>
          CAPM
        </button>
        <button className={`tab ${tab === "ff6" ? "active" : ""}`} onClick={() => setTab("ff6")}>
          Fama-French 6-Factor
        </button>
        <button className={`tab ${tab === "dictionary" ? "active" : ""}`} onClick={() => setTab("dictionary")}>
          Data Dictionary
        </button>
      </div>

      {tab === "dictionary" && <DataDictionary />}

      {tab !== "dictionary" && (
        <>
          <div className="search-bar">
            <input
              type="text"
              placeholder={`Search ticker or ${itemLabel === "funds" ? "fund name" : "company"}...`}
              value={search}
              onChange={(e) => setSearch(e.target.value)}
            />
            <select value={sectorFilter} onChange={(e) => setSectorFilter(e.target.value)}>
              <option value="all">All {sectorLabel}s</option>
              {sectors.map((s) => (
                <option key={s} value={s}>{s}</option>
              ))}
            </select>
          </div>

          <div className="table-container">
            {tab === "summary" && (
              <table>
                <thead>
                  <tr>
                    <th className={thClass("ticker")} onClick={() => handleSort("ticker")}>Ticker <SortArrow col="ticker" /></th>
                    <th className={thClass("company")} onClick={() => handleSort("company")}>{itemLabel === "funds" ? "Fund Name" : "Company"} <SortArrow col="company" /></th>
                    <th className={thClass("sector")} onClick={() => handleSort("sector")}>{sectorLabel} <SortArrow col="sector" /></th>
                    <th className={thClass("price")} onClick={() => handleSort("price")}>Price <SortArrow col="price" /></th>
                    <th className={thClass("ann_return")} onClick={() => handleSort("ann_return")}>Ann. Return <SortArrow col="ann_return" /></th>
                    <th className={thClass("ann_std")} onClick={() => handleSort("ann_std")}>Ann. Std Dev <SortArrow col="ann_std" /></th>
                    <th className={thClass("capm_er")} onClick={() => handleSort("capm_er")}>CAPM E(R) <SortArrow col="capm_er" /></th>
                    <th className={thClass("ff6_er")} onClick={() => handleSort("ff6_er")}>FF6 E(R) <SortArrow col="ff6_er" /></th>
                    <th className={thClass("capm_beta")} onClick={() => handleSort("capm_beta")}>Beta <SortArrow col="capm_beta" /></th>
                  </tr>
                </thead>
                <tbody>
                  {filtered.map((s) => (
                    <tr key={s.info.ticker}>
                      <TickerCell ticker={s.info.ticker} company={s.info.company} />
                      <td className="company">{s.info.company}</td>
                      <td className="sector">{s.info.sector}</td>
                      <td className="numeric">{s.current_price != null ? usd(s.current_price) : "—"}</td>
                      <td className={`numeric ${colorClass(s.annualized_return)}`}>{pct(s.annualized_return)}</td>
                      <td className="numeric">{pct(s.annualized_std_dev)}</td>
                      <td className={`numeric ${colorClass(s.capm.expected_return)}`}>{pct(s.capm.expected_return)}</td>
                      <td className={`numeric ${colorClass(s.ff6.expected_return)}`}>{pct(s.ff6.expected_return)}</td>
                      <td className="numeric">{num(s.capm.beta)}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            )}

            {tab === "capm" && (
              <table>
                <thead>
                  <tr>
                    <th className={thClass("ticker")} onClick={() => handleSort("ticker")}>Ticker <SortArrow col="ticker" /></th>
                    <th className={thClass("company")} onClick={() => handleSort("company")}>{itemLabel === "funds" ? "Fund Name" : "Company"} <SortArrow col="company" /></th>
                    <th className={thClass("sector")} onClick={() => handleSort("sector")}>{sectorLabel} <SortArrow col="sector" /></th>
                    <th className={thClass("capm_beta")} onClick={() => handleSort("capm_beta")}>Beta <SortArrow col="capm_beta" /></th>
                    <th className={thClass("capm_alpha")} onClick={() => handleSort("capm_alpha")}>Alpha (ann.) <SortArrow col="capm_alpha" /></th>
                    <th className={thClass("capm_er")} onClick={() => handleSort("capm_er")}>Expected Return <SortArrow col="capm_er" /></th>
                    <th className={thClass("capm_r2")} onClick={() => handleSort("capm_r2")}>R² <SortArrow col="capm_r2" /></th>
                  </tr>
                </thead>
                <tbody>
                  {filtered.map((s) => (
                    <tr key={s.info.ticker}>
                      <TickerCell ticker={s.info.ticker} company={s.info.company} />
                      <td className="company">{s.info.company}</td>
                      <td className="sector">{s.info.sector}</td>
                      <td className="numeric">{num(s.capm.beta)}</td>
                      <td className={`numeric ${colorClass(s.capm.alpha)}`}>{pct(s.capm.alpha)}</td>
                      <td className={`numeric ${colorClass(s.capm.expected_return)}`}>{pct(s.capm.expected_return)}</td>
                      <td className="numeric">{num(s.capm.r_squared)}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            )}

            {tab === "ff6" && (
              <table>
                <thead>
                  <tr>
                    <th className={thClass("ticker")} onClick={() => handleSort("ticker")}>Ticker <SortArrow col="ticker" /></th>
                    <th className={thClass("ff6_alpha")} onClick={() => handleSort("ff6_alpha")}>Alpha (ann.) <SortArrow col="ff6_alpha" /></th>
                    <th className={thClass("ff6_mkt")} onClick={() => handleSort("ff6_mkt")}>Mkt <SortArrow col="ff6_mkt" /></th>
                    <th className={thClass("ff6_smb")} onClick={() => handleSort("ff6_smb")}>SMB <SortArrow col="ff6_smb" /></th>
                    <th className={thClass("ff6_hml")} onClick={() => handleSort("ff6_hml")}>HML <SortArrow col="ff6_hml" /></th>
                    <th className={thClass("ff6_rmw")} onClick={() => handleSort("ff6_rmw")}>RMW <SortArrow col="ff6_rmw" /></th>
                    <th className={thClass("ff6_cma")} onClick={() => handleSort("ff6_cma")}>CMA <SortArrow col="ff6_cma" /></th>
                    <th className={thClass("ff6_mom")} onClick={() => handleSort("ff6_mom")}>Mom <SortArrow col="ff6_mom" /></th>
                    <th className={thClass("ff6_er")} onClick={() => handleSort("ff6_er")}>E(R) <SortArrow col="ff6_er" /></th>
                    <th className={thClass("ff6_r2")} onClick={() => handleSort("ff6_r2")}>R² <SortArrow col="ff6_r2" /></th>
                  </tr>
                </thead>
                <tbody>
                  {filtered.map((s) => (
                    <tr key={s.info.ticker}>
                      <TickerCell ticker={s.info.ticker} company={s.info.company} />
                      <td className={`numeric ${colorClass(s.ff6.alpha)}`}>{pct(s.ff6.alpha)}</td>
                      <td className="numeric">{num(s.ff6.beta_mkt)}</td>
                      <td className="numeric">{num(s.ff6.beta_smb)}</td>
                      <td className="numeric">{num(s.ff6.beta_hml)}</td>
                      <td className="numeric">{num(s.ff6.beta_rmw)}</td>
                      <td className="numeric">{num(s.ff6.beta_cma)}</td>
                      <td className="numeric">{num(s.ff6.beta_mom)}</td>
                      <td className={`numeric ${colorClass(s.ff6.expected_return)}`}>{pct(s.ff6.expected_return)}</td>
                      <td className="numeric">{num(s.ff6.r_squared)}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            )}
          </div>
        </>
      )}
    </>
  );
}

function DataDictionary() {
  return (
    <div className="dictionary">
      <section className="dict-section">
        <h2>Models</h2>
        <div className="dict-card">
          <h3>CAPM &mdash; Capital Asset Pricing Model</h3>
          <p className="dict-formula">E(R<sub>i</sub>) = R<sub>f</sub> + &beta;<sub>i</sub> &times; (E(R<sub>m</sub>) &minus; R<sub>f</sub>)</p>
          <p>
            The CAPM describes the relationship between systematic risk and expected return.
            It assumes investors are compensated for time value of money (risk-free rate) and
            the additional risk of holding the market portfolio (market premium), scaled by
            the stock's sensitivity to market movements (beta).
          </p>
        </div>
        <div className="dict-card">
          <h3>Fama-French 6-Factor Model</h3>
          <p className="dict-formula">R<sub>i</sub> &minus; R<sub>f</sub> = &alpha;<sub>i</sub> + &beta;<sub>1</sub>(R<sub>m</sub>&minus;R<sub>f</sub>) + &beta;<sub>2</sub>SMB + &beta;<sub>3</sub>HML + &beta;<sub>4</sub>RMW + &beta;<sub>5</sub>CMA + &beta;<sub>6</sub>Mom + &epsilon;<sub>i</sub></p>
          <p>
            An extension of CAPM that explains stock returns using six risk factors instead of one.
            This model captures size, value, profitability, investment, and momentum effects
            that CAPM alone cannot explain. Higher R&sup2; values indicate better explanatory power.
          </p>
        </div>
      </section>

      <section className="dict-section">
        <h2>Summary Tab Fields</h2>
        <div className="dict-table-wrap">
          <table className="dict-table">
            <thead><tr><th>Field</th><th>Description</th><th>How It's Computed</th></tr></thead>
            <tbody>
              <tr><td className="dict-field">Ticker</td><td>Exchange symbol</td><td>S&P 500 tickers scraped from Wikipedia; DFA tickers from a curated fund list. Dots converted to dashes for Yahoo Finance (e.g. BRK.B &rarr; BRK-B)</td></tr>
              <tr><td className="dict-field">Company / Fund Name</td><td>Full name</td><td>S&P 500 names from Wikipedia; DFA fund names from Dimensional's fund catalog</td></tr>
              <tr><td className="dict-field">Sector / Category</td><td>Classification</td><td>GICS sector for stocks; asset class category for DFA funds</td></tr>
              <tr><td className="dict-field">Ann. Return</td><td>Annualized realized geometric return over the lookback period</td><td>Product of (1 + daily return) raised to the power of (252 / trading days), minus 1</td></tr>
              <tr><td className="dict-field">Ann. Std Dev</td><td>Annualized volatility &mdash; a measure of total risk</td><td>Standard deviation of daily returns &times; &radic;252</td></tr>
              <tr><td className="dict-field">CAPM E(R)</td><td>Expected annual return implied by the CAPM</td><td>R<sub>f</sub> + &beta; &times; (R<sub>m</sub> &minus; R<sub>f</sub>), annualized &times; 252</td></tr>
              <tr><td className="dict-field">FF6 E(R)</td><td>Expected annual return implied by the 6-factor model</td><td>Sum of risk-free rate plus each factor loading &times; that factor's average premium, annualized &times; 252</td></tr>
              <tr><td className="dict-field">Beta</td><td>CAPM beta &mdash; sensitivity to market movements</td><td>OLS slope from regressing excess returns on market excess returns</td></tr>
            </tbody>
          </table>
        </div>
      </section>

      <section className="dict-section">
        <h2>CAPM Tab Fields</h2>
        <div className="dict-table-wrap">
          <table className="dict-table">
            <thead><tr><th>Field</th><th>Description</th><th>Interpretation</th></tr></thead>
            <tbody>
              <tr><td className="dict-field">Beta (&beta;)</td><td>Market sensitivity coefficient</td><td>&beta; = 1.0 moves with the market; &gt; 1.0 is more volatile; &lt; 1.0 is less volatile</td></tr>
              <tr><td className="dict-field">Alpha (&alpha;)</td><td>Annualized intercept from the CAPM regression</td><td>Positive alpha suggests outperformance vs. CAPM prediction; negative suggests underperformance</td></tr>
              <tr><td className="dict-field">Expected Return</td><td>CAPM-implied annual expected return</td><td>The return an investor should demand given the systematic risk level</td></tr>
              <tr><td className="dict-field">R&sup2;</td><td>Coefficient of determination</td><td>Proportion of return variance explained by market movements. R&sup2; = 0.60 means 60% of variance is market-driven</td></tr>
            </tbody>
          </table>
        </div>
      </section>

      <section className="dict-section">
        <h2>Fama-French 6-Factor Tab Fields</h2>
        <div className="dict-table-wrap">
          <table className="dict-table">
            <thead><tr><th>Field</th><th>Full Name</th><th>What It Captures</th></tr></thead>
            <tbody>
              <tr><td className="dict-field">Alpha (&alpha;)</td><td>Annualized intercept</td><td>Abnormal return not explained by any of the six factors</td></tr>
              <tr><td className="dict-field">Mkt</td><td>Market excess return (R<sub>m</sub> &minus; R<sub>f</sub>)</td><td>Sensitivity to the broad equity market premium</td></tr>
              <tr><td className="dict-field">SMB</td><td>Small Minus Big</td><td>Size factor. Positive loading = small-cap behavior; negative = large-cap</td></tr>
              <tr><td className="dict-field">HML</td><td>High Minus Low</td><td>Value factor. Positive = value characteristics; negative = growth</td></tr>
              <tr><td className="dict-field">RMW</td><td>Robust Minus Weak</td><td>Profitability factor. Positive = co-moves with profitable firms</td></tr>
              <tr><td className="dict-field">CMA</td><td>Conservative Minus Aggressive</td><td>Investment factor. Positive = co-moves with conservative investors</td></tr>
              <tr><td className="dict-field">Mom</td><td>Momentum (UMD)</td><td>Momentum factor. Positive = follows recent winners</td></tr>
              <tr><td className="dict-field">E(R)</td><td>Expected return</td><td>Annual expected return from all six factor premiums weighted by loadings</td></tr>
              <tr><td className="dict-field">R&sup2;</td><td>Coefficient of determination</td><td>Proportion of return variance explained by all six factors together</td></tr>
            </tbody>
          </table>
        </div>
      </section>

      <section className="dict-section">
        <h2>Methodology</h2>
        <div className="dict-card">
          <h3>Regression Method</h3>
          <p>All factor loadings are estimated via Ordinary Least Squares (OLS): <span className="dict-mono">&beta; = (X'X)<sup>&minus;1</sup>X'y</span>. The dependent variable is the daily excess return (return minus risk-free rate). Independent variables are the relevant factor return series.</p>
        </div>
        <div className="dict-card">
          <h3>Data Sources</h3>
          <ul>
            <li><strong>Stock/fund prices</strong> &mdash; Yahoo Finance daily adjusted close, ~1 year lookback</li>
            <li><strong>Factor data</strong> &mdash; Kenneth French Data Library (Dartmouth), daily frequency</li>
            <li><strong>Risk-free rate</strong> &mdash; 1-month U.S. Treasury bill rate, included in the French data files</li>
            <li><strong>S&P 500 list</strong> &mdash; Wikipedia "List of S&P 500 companies"</li>
            <li><strong>DFA fund list</strong> &mdash; Curated list of Dimensional Fund Advisors mutual funds and ETFs</li>
          </ul>
        </div>
        <div className="dict-card">
          <h3>Annualization</h3>
          <ul>
            <li><strong>Returns</strong> &mdash; geometric compounding: (1+r)<sup>252/n</sup> &minus; 1</li>
            <li><strong>Volatility</strong> &mdash; &sigma;<sub>daily</sub> &times; &radic;252</li>
            <li><strong>Alpha</strong> &mdash; daily alpha &times; 252</li>
            <li><strong>Expected returns</strong> &mdash; daily expected return &times; 252</li>
          </ul>
          <p>252 trading days per year is the standard convention.</p>
        </div>
      </section>
    </div>
  );
}

export default App;
