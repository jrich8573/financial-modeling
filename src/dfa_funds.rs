use crate::types::StockInfo;

pub struct DfaFund {
    pub ticker: &'static str,
    pub name: &'static str,
    pub category: &'static str,
}

pub fn get_dfa_funds() -> Vec<StockInfo> {
    DFA_FUNDS
        .iter()
        .map(|f| StockInfo {
            ticker: f.ticker.to_string(),
            company: f.name.to_string(),
            sector: f.category.to_string(),
        })
        .collect()
}

static DFA_FUNDS: &[DfaFund] = &[
    // US Equity - Large Cap
    DfaFund { ticker: "DFUSX", name: "US Large Company Portfolio", category: "US Large Cap" },
    DfaFund { ticker: "DFLVX", name: "US Large Cap Value Portfolio", category: "US Large Cap Value" },
    DfaFund { ticker: "DUSQX", name: "US Large Cap Equity Portfolio", category: "US Large Cap" },
    DfaFund { ticker: "DFLEX", name: "US Large Company Series", category: "US Large Cap" },

    // US Equity - Core
    DfaFund { ticker: "DFCEX", name: "US Core Equity 1 Portfolio", category: "US Core Equity" },
    DfaFund { ticker: "DFTCX", name: "US Core Equity 2 Portfolio", category: "US Core Equity" },
    DfaFund { ticker: "DFEOX", name: "US Equity Portfolio", category: "US Core Equity" },
    DfaFund { ticker: "DFQTX", name: "US Vector Equity Portfolio", category: "US Core Equity" },

    // US Equity - Small Cap
    DfaFund { ticker: "DFSTX", name: "US Micro Cap Portfolio", category: "US Small Cap" },
    DfaFund { ticker: "DFSEX", name: "US Small Cap Portfolio", category: "US Small Cap" },
    DfaFund { ticker: "DFSVX", name: "US Small Cap Value Portfolio", category: "US Small Cap Value" },
    DfaFund { ticker: "DFTSX", name: "US Targeted Value Portfolio", category: "US Small Cap Value" },

    // US Equity - Other
    DfaFund { ticker: "DFREX", name: "Real Estate Securities Portfolio", category: "US Real Estate" },
    DfaFund { ticker: "DSCLX", name: "US Social Core Equity 2 Portfolio", category: "US Social Core" },
    DfaFund { ticker: "DFUEX", name: "US Sustainability Core 1 Portfolio", category: "US Sustainability" },

    // International Developed
    DfaFund { ticker: "DFALX", name: "International Core Equity Portfolio", category: "Intl Developed" },
    DfaFund { ticker: "DFIVX", name: "International Value Portfolio", category: "Intl Developed Value" },
    DfaFund { ticker: "DFISX", name: "International Small Company Portfolio", category: "Intl Small Cap" },
    DfaFund { ticker: "DFICX", name: "International Small Cap Value Portfolio", category: "Intl Small Cap Value" },
    DfaFund { ticker: "DFVQX", name: "International Vector Equity Portfolio", category: "Intl Developed" },
    DfaFund { ticker: "DILPX", name: "International Large Cap Portfolio", category: "Intl Large Cap" },
    DfaFund { ticker: "DFIEX", name: "International Real Estate Securities Portfolio", category: "Intl Real Estate" },
    DfaFund { ticker: "DFIHX", name: "International High Relative Profitability Portfolio", category: "Intl Developed" },
    DfaFund { ticker: "DFSPX", name: "International Sustainability Core 1 Portfolio", category: "Intl Sustainability" },

    // Emerging Markets
    DfaFund { ticker: "DFEMX", name: "Emerging Markets Portfolio", category: "Emerging Markets" },
    DfaFund { ticker: "DFEVX", name: "Emerging Markets Value Portfolio", category: "Emerging Markets Value" },
    DfaFund { ticker: "DEMSX", name: "Emerging Markets Small Cap Portfolio", category: "Emerging Markets Small" },
    DfaFund { ticker: "DFECX", name: "Emerging Markets Core Equity Portfolio", category: "Emerging Markets Core" },
    DfaFund { ticker: "DFEPX", name: "Emerging Markets ex China Portfolio", category: "Emerging Markets" },

    // Global Equity
    DfaFund { ticker: "DGEIX", name: "Global Equity Portfolio", category: "Global Equity" },
    DfaFund { ticker: "DGSIX", name: "Global Small Company Portfolio", category: "Global Small Cap" },
    DfaFund { ticker: "DREIX", name: "Global Real Estate Securities Portfolio", category: "Global Real Estate" },

    // Fixed Income - Short Term
    DfaFund { ticker: "DFIHX", name: "Intermediate Government Fixed Income Portfolio", category: "US Fixed Income" },
    DfaFund { ticker: "DFEQX", name: "Short-Term Government Portfolio", category: "US Short-Term Fixed" },
    DfaFund { ticker: "DFGFX", name: "One-Year Fixed Income Portfolio", category: "US Short-Term Fixed" },
    DfaFund { ticker: "DFSGX", name: "Short-Duration Real Return Portfolio", category: "US Short-Term Fixed" },

    // Fixed Income - Intermediate/Long
    DfaFund { ticker: "DFAPX", name: "Intermediate-Term Extended Quality Portfolio", category: "US Intermediate Fixed" },
    DfaFund { ticker: "DFIDX", name: "Investment Grade Portfolio", category: "US Investment Grade" },
    DfaFund { ticker: "DFILX", name: "Inflation-Protected Securities Portfolio", category: "US TIPS" },

    // Fixed Income - Global
    DfaFund { ticker: "DFGBX", name: "World ex US Government Fixed Income Portfolio", category: "Global Fixed Income" },
    DfaFund { ticker: "DFLBX", name: "Five-Year Global Fixed Income Portfolio", category: "Global Fixed Income" },

    // Fixed Income - Municipal
    DfaFund { ticker: "DFTEX", name: "National Municipal Bond Portfolio", category: "Municipal Bonds" },
    DfaFund { ticker: "DFSMX", name: "Short-Term Municipal Bond Portfolio", category: "Municipal Bonds" },
    DfaFund { ticker: "DFLMX", name: "Intermediate-Term Municipal Bond Portfolio", category: "Municipal Bonds" },

    // DFA ETFs - US
    DfaFund { ticker: "DFAC", name: "Dimensional US Core Equity Market ETF", category: "ETF - US Core" },
    DfaFund { ticker: "DFAU", name: "Dimensional US Core Equity 2 ETF", category: "ETF - US Core" },
    DfaFund { ticker: "DFAS", name: "Dimensional US Small Cap ETF", category: "ETF - US Small Cap" },
    DfaFund { ticker: "DFAT", name: "Dimensional US Targeted Value ETF", category: "ETF - US Value" },
    DfaFund { ticker: "DFUV", name: "Dimensional US Marketwide Value ETF", category: "ETF - US Value" },
    DfaFund { ticker: "DFSV", name: "Dimensional US Small Cap Value ETF", category: "ETF - US Small Value" },
    DfaFund { ticker: "DFUS", name: "Dimensional US Equity ETF", category: "ETF - US Core" },
    DfaFund { ticker: "DFLC", name: "Dimensional US Large Cap ETF", category: "ETF - US Large Cap" },
    DfaFund { ticker: "DFSU", name: "Dimensional US Sustainability Core 1 ETF", category: "ETF - US Sustainability" },
    DfaFund { ticker: "DFRE", name: "Dimensional US Real Estate ETF", category: "ETF - US Real Estate" },

    // DFA ETFs - International
    DfaFund { ticker: "DFAI", name: "Dimensional International Core Equity Market ETF", category: "ETF - Intl Core" },
    DfaFund { ticker: "DFIV", name: "Dimensional International Value ETF", category: "ETF - Intl Value" },
    DfaFund { ticker: "DFIS", name: "Dimensional International Small Cap ETF", category: "ETF - Intl Small" },
    DfaFund { ticker: "DISV", name: "Dimensional International Small Cap Value ETF", category: "ETF - Intl Small Value" },
    DfaFund { ticker: "DFAX", name: "Dimensional World ex US Core Equity 2 ETF", category: "ETF - Intl Core" },
    DfaFund { ticker: "DFSI", name: "Dimensional International Sustainability Core 1 ETF", category: "ETF - Intl Sustainability" },

    // DFA ETFs - Emerging Markets
    DfaFund { ticker: "DFAE", name: "Dimensional Emerging Core Equity Market ETF", category: "ETF - EM Core" },
    DfaFund { ticker: "DFEV", name: "Dimensional Emerging Markets Value ETF", category: "ETF - EM Value" },
    DfaFund { ticker: "DEXC", name: "Dimensional Emerging Markets ex China Core Equity ETF", category: "ETF - EM ex China" },

    // DFA ETFs - Global
    DfaFund { ticker: "DFAW", name: "Dimensional World Equity ETF", category: "ETF - Global Equity" },
    DfaFund { ticker: "DFGR", name: "Dimensional Global Real Estate ETF", category: "ETF - Global Real Estate" },

    // DFA ETFs - Fixed Income
    DfaFund { ticker: "DFSD", name: "Dimensional Short-Duration Fixed Income ETF", category: "ETF - Short Duration" },
    DfaFund { ticker: "DFIP", name: "Dimensional Inflation-Protected Securities ETF", category: "ETF - TIPS" },
    DfaFund { ticker: "DFNM", name: "Dimensional National Municipal Bond ETF", category: "ETF - Muni Bonds" },
    DfaFund { ticker: "DFIX", name: "Dimensional International Core Fixed Income ETF", category: "ETF - Intl Fixed" },
    DfaFund { ticker: "DGCB", name: "Dimensional Global Core Fixed Income ETF", category: "ETF - Global Fixed" },
    DfaFund { ticker: "DUSB", name: "Dimensional Ultra-Short Fixed Income ETF", category: "ETF - Ultra Short" },
    DfaFund { ticker: "DCOR", name: "Dimensional US Core Fixed Income ETF", category: "ETF - US Core Fixed" },
    DfaFund { ticker: "DSBB", name: "Dimensional Short-Duration Corporate Bond ETF", category: "ETF - Short Corp" },
];
