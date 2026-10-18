use crate::types::StockInfo;

pub struct DfaFund {
    pub ticker: &'static str,
    pub name: &'static str,
    pub category: &'static str,
    pub benchmark: &'static str,
}

pub fn get_dfa_funds() -> Vec<StockInfo> {
    DFA_FUNDS
        .iter()
        .map(|f| StockInfo {
            ticker: f.ticker.to_string(),
            company: f.name.to_string(),
            sector: f.category.to_string(),
            benchmark: Some(f.benchmark.to_string()),
        })
        .collect()
}

// Benchmark key:
//   SPY  = S&P 500
//   VTV  = Vanguard Value (Russell 1000 Value)
//   VTI  = Vanguard Total Stock Market (CRSP US Total)
//   IWM  = iShares Russell 2000
//   IJS  = iShares S&P 600 Small-Cap Value
//   VNQ  = Vanguard Real Estate (MSCI US REIT)
//   EFA  = iShares MSCI EAFE
//   EFV  = iShares MSCI EAFE Value
//   SCZ  = iShares MSCI EAFE Small-Cap
//   EEM  = iShares MSCI Emerging Markets
//   VT   = Vanguard Total World Stock
//   AGG  = iShares Core US Aggregate Bond
//   SHY  = iShares 1-3 Year Treasury Bond
//   SHV  = iShares Short Treasury Bond
//   IEF  = iShares 7-10 Year Treasury Bond
//   LQD  = iShares Investment Grade Corporate Bond
//   TIP  = iShares TIPS Bond
//   BNDX = Vanguard Total International Bond
//   MUB  = iShares National Muni Bond
//   VNQI = Vanguard Global ex-US Real Estate

static DFA_FUNDS: &[DfaFund] = &[
    // US Equity - Large Cap
    DfaFund { ticker: "DFUSX", name: "US Large Company Portfolio", category: "US Large Cap", benchmark: "SPY" },
    DfaFund { ticker: "DFLVX", name: "US Large Cap Value Portfolio", category: "US Large Cap Value", benchmark: "VTV" },
    DfaFund { ticker: "DUSQX", name: "US Large Cap Equity Portfolio", category: "US Large Cap", benchmark: "SPY" },
    DfaFund { ticker: "DFLEX", name: "US Large Company Series", category: "US Large Cap", benchmark: "SPY" },

    // US Equity - Core
    DfaFund { ticker: "DFCEX", name: "US Core Equity 1 Portfolio", category: "US Core Equity", benchmark: "VTI" },
    DfaFund { ticker: "DFTCX", name: "US Core Equity 2 Portfolio", category: "US Core Equity", benchmark: "VTI" },
    DfaFund { ticker: "DFEOX", name: "US Equity Portfolio", category: "US Core Equity", benchmark: "VTI" },
    DfaFund { ticker: "DFQTX", name: "US Vector Equity Portfolio", category: "US Core Equity", benchmark: "VTI" },

    // US Equity - Small Cap
    DfaFund { ticker: "DFSTX", name: "US Micro Cap Portfolio", category: "US Small Cap", benchmark: "IWM" },
    DfaFund { ticker: "DFSEX", name: "US Small Cap Portfolio", category: "US Small Cap", benchmark: "IWM" },
    DfaFund { ticker: "DFSVX", name: "US Small Cap Value Portfolio", category: "US Small Cap Value", benchmark: "IJS" },
    DfaFund { ticker: "DFTSX", name: "US Targeted Value Portfolio", category: "US Small Cap Value", benchmark: "IJS" },

    // US Equity - Other
    DfaFund { ticker: "DFREX", name: "Real Estate Securities Portfolio", category: "US Real Estate", benchmark: "VNQ" },
    DfaFund { ticker: "DSCLX", name: "US Social Core Equity 2 Portfolio", category: "US Social Core", benchmark: "VTI" },
    DfaFund { ticker: "DFUEX", name: "US Sustainability Core 1 Portfolio", category: "US Sustainability", benchmark: "VTI" },

    // International Developed
    DfaFund { ticker: "DFALX", name: "International Core Equity Portfolio", category: "Intl Developed", benchmark: "EFA" },
    DfaFund { ticker: "DFIVX", name: "International Value Portfolio", category: "Intl Developed Value", benchmark: "EFV" },
    DfaFund { ticker: "DFISX", name: "International Small Company Portfolio", category: "Intl Small Cap", benchmark: "SCZ" },
    DfaFund { ticker: "DFICX", name: "International Small Cap Value Portfolio", category: "Intl Small Cap Value", benchmark: "SCZ" },
    DfaFund { ticker: "DFVQX", name: "International Vector Equity Portfolio", category: "Intl Developed", benchmark: "EFA" },
    DfaFund { ticker: "DILPX", name: "International Large Cap Portfolio", category: "Intl Large Cap", benchmark: "EFA" },
    DfaFund { ticker: "DFIEX", name: "International Real Estate Securities Portfolio", category: "Intl Real Estate", benchmark: "VNQI" },
    DfaFund { ticker: "DFIHX", name: "International High Relative Profitability Portfolio", category: "Intl Developed", benchmark: "EFA" },
    DfaFund { ticker: "DFSPX", name: "International Sustainability Core 1 Portfolio", category: "Intl Sustainability", benchmark: "EFA" },

    // Emerging Markets
    DfaFund { ticker: "DFEMX", name: "Emerging Markets Portfolio", category: "Emerging Markets", benchmark: "EEM" },
    DfaFund { ticker: "DFEVX", name: "Emerging Markets Value Portfolio", category: "Emerging Markets Value", benchmark: "EEM" },
    DfaFund { ticker: "DEMSX", name: "Emerging Markets Small Cap Portfolio", category: "Emerging Markets Small", benchmark: "EEM" },
    DfaFund { ticker: "DFECX", name: "Emerging Markets Core Equity Portfolio", category: "Emerging Markets Core", benchmark: "EEM" },
    DfaFund { ticker: "DFEPX", name: "Emerging Markets ex China Portfolio", category: "Emerging Markets", benchmark: "EEM" },

    // Global Equity
    DfaFund { ticker: "DGEIX", name: "Global Equity Portfolio", category: "Global Equity", benchmark: "VT" },
    DfaFund { ticker: "DGSIX", name: "Global Small Company Portfolio", category: "Global Small Cap", benchmark: "VT" },
    DfaFund { ticker: "DREIX", name: "Global Real Estate Securities Portfolio", category: "Global Real Estate", benchmark: "VNQ" },

    // Fixed Income - Short Term
    DfaFund { ticker: "DFIGX", name: "Intermediate Government Fixed Income Portfolio", category: "US Fixed Income", benchmark: "IEF" },
    DfaFund { ticker: "DFEQX", name: "Short-Term Government Portfolio", category: "US Short-Term Fixed", benchmark: "SHY" },
    DfaFund { ticker: "DFGFX", name: "One-Year Fixed Income Portfolio", category: "US Short-Term Fixed", benchmark: "SHV" },
    DfaFund { ticker: "DFSGX", name: "Short-Duration Real Return Portfolio", category: "US Short-Term Fixed", benchmark: "SHY" },

    // Fixed Income - Intermediate/Long
    DfaFund { ticker: "DFAPX", name: "Intermediate-Term Extended Quality Portfolio", category: "US Intermediate Fixed", benchmark: "AGG" },
    DfaFund { ticker: "DFIDX", name: "Investment Grade Portfolio", category: "US Investment Grade", benchmark: "LQD" },
    DfaFund { ticker: "DFILX", name: "Inflation-Protected Securities Portfolio", category: "US TIPS", benchmark: "TIP" },

    // Fixed Income - Global
    DfaFund { ticker: "DFGBX", name: "World ex US Government Fixed Income Portfolio", category: "Global Fixed Income", benchmark: "BNDX" },
    DfaFund { ticker: "DFLBX", name: "Five-Year Global Fixed Income Portfolio", category: "Global Fixed Income", benchmark: "BNDX" },

    // Fixed Income - Municipal
    DfaFund { ticker: "DFTEX", name: "National Municipal Bond Portfolio", category: "Municipal Bonds", benchmark: "MUB" },
    DfaFund { ticker: "DFSMX", name: "Short-Term Municipal Bond Portfolio", category: "Municipal Bonds", benchmark: "MUB" },
    DfaFund { ticker: "DFLMX", name: "Intermediate-Term Municipal Bond Portfolio", category: "Municipal Bonds", benchmark: "MUB" },

    // DFA ETFs - US
    DfaFund { ticker: "DFAC", name: "Dimensional US Core Equity Market ETF", category: "ETF - US Core", benchmark: "VTI" },
    DfaFund { ticker: "DFAU", name: "Dimensional US Core Equity 2 ETF", category: "ETF - US Core", benchmark: "VTI" },
    DfaFund { ticker: "DFAS", name: "Dimensional US Small Cap ETF", category: "ETF - US Small Cap", benchmark: "IWM" },
    DfaFund { ticker: "DFAT", name: "Dimensional US Targeted Value ETF", category: "ETF - US Value", benchmark: "VTV" },
    DfaFund { ticker: "DFUV", name: "Dimensional US Marketwide Value ETF", category: "ETF - US Value", benchmark: "VTV" },
    DfaFund { ticker: "DFSV", name: "Dimensional US Small Cap Value ETF", category: "ETF - US Small Value", benchmark: "IJS" },
    DfaFund { ticker: "DFUS", name: "Dimensional US Equity ETF", category: "ETF - US Core", benchmark: "VTI" },
    DfaFund { ticker: "DFLC", name: "Dimensional US Large Cap ETF", category: "ETF - US Large Cap", benchmark: "SPY" },
    DfaFund { ticker: "DFSU", name: "Dimensional US Sustainability Core 1 ETF", category: "ETF - US Sustainability", benchmark: "VTI" },
    DfaFund { ticker: "DFRE", name: "Dimensional US Real Estate ETF", category: "ETF - US Real Estate", benchmark: "VNQ" },

    // DFA ETFs - International
    DfaFund { ticker: "DFAI", name: "Dimensional International Core Equity Market ETF", category: "ETF - Intl Core", benchmark: "EFA" },
    DfaFund { ticker: "DFIV", name: "Dimensional International Value ETF", category: "ETF - Intl Value", benchmark: "EFV" },
    DfaFund { ticker: "DFIS", name: "Dimensional International Small Cap ETF", category: "ETF - Intl Small", benchmark: "SCZ" },
    DfaFund { ticker: "DISV", name: "Dimensional International Small Cap Value ETF", category: "ETF - Intl Small Value", benchmark: "SCZ" },
    DfaFund { ticker: "DFAX", name: "Dimensional World ex US Core Equity 2 ETF", category: "ETF - Intl Core", benchmark: "EFA" },
    DfaFund { ticker: "DFSI", name: "Dimensional International Sustainability Core 1 ETF", category: "ETF - Intl Sustainability", benchmark: "EFA" },

    // DFA ETFs - Emerging Markets
    DfaFund { ticker: "DFAE", name: "Dimensional Emerging Core Equity Market ETF", category: "ETF - EM Core", benchmark: "EEM" },
    DfaFund { ticker: "DFEV", name: "Dimensional Emerging Markets Value ETF", category: "ETF - EM Value", benchmark: "EEM" },
    DfaFund { ticker: "DEXC", name: "Dimensional Emerging Markets ex China Core Equity ETF", category: "ETF - EM ex China", benchmark: "EEM" },

    // DFA ETFs - Global
    DfaFund { ticker: "DFAW", name: "Dimensional World Equity ETF", category: "ETF - Global Equity", benchmark: "VT" },
    DfaFund { ticker: "DFGR", name: "Dimensional Global Real Estate ETF", category: "ETF - Global Real Estate", benchmark: "VNQ" },

    // DFA ETFs - Fixed Income
    DfaFund { ticker: "DFSD", name: "Dimensional Short-Duration Fixed Income ETF", category: "ETF - Short Duration", benchmark: "SHY" },
    DfaFund { ticker: "DFIP", name: "Dimensional Inflation-Protected Securities ETF", category: "ETF - TIPS", benchmark: "TIP" },
    DfaFund { ticker: "DFNM", name: "Dimensional National Municipal Bond ETF", category: "ETF - Muni Bonds", benchmark: "MUB" },
    DfaFund { ticker: "DFIX", name: "Dimensional International Core Fixed Income ETF", category: "ETF - Intl Fixed", benchmark: "BNDX" },
    DfaFund { ticker: "DGCB", name: "Dimensional Global Core Fixed Income ETF", category: "ETF - Global Fixed", benchmark: "AGG" },
    DfaFund { ticker: "DUSB", name: "Dimensional Ultra-Short Fixed Income ETF", category: "ETF - Ultra Short", benchmark: "SHV" },
    DfaFund { ticker: "DCOR", name: "Dimensional US Core Fixed Income ETF", category: "ETF - US Core Fixed", benchmark: "AGG" },
    DfaFund { ticker: "DSBB", name: "Dimensional Short-Duration Corporate Bond ETF", category: "ETF - Short Corp", benchmark: "SHY" },
];
