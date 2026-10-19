#!/usr/bin/env python3
"""
Compute fund analytics from the CRSP spreadsheet and output JSON for the web frontend.
Metrics: Sharpe ratio, max drawdown, annualized performance (1/3/5/10yr),
standard deviation, management fee, fee after waiver, return over benchmark.
"""
import json
import math
import sys
from datetime import datetime

import openpyxl

XLSX_PATH = sys.argv[1] if len(sys.argv) > 1 else "/Users/jasonrich/Downloads/(OG13)AUM_TNA_FEES_FLOWS.xlsx"
OUTPUT_PATH = sys.argv[2] if len(sys.argv) > 2 else "web/public/fund_analytics.json"

RF_ANNUAL = 0.05  # approximate risk-free rate for Sharpe

def load_monthly_returns(wb, ticker):
    """Load monthly returns from the per-fund sheet."""
    if ticker not in wb.sheetnames:
        return []
    ws = wb[ticker]
    headers = [c.value for c in next(ws.iter_rows(min_row=1, max_row=1))]
    month_idx = headers.index("month")
    ret_idx = headers.index("monthly_return")

    records = []
    for row in ws.iter_rows(min_row=2, values_only=True):
        month_str = row[month_idx]
        ret = row[ret_idx]
        if month_str and ret is not None:
            records.append({"month": str(month_str), "return": float(ret)})
    return records


def load_fund_summary(wb):
    """Load Fund_Summary sheet."""
    ws = wb["Fund_Summary"]
    headers = [c.value for c in next(ws.iter_rows(min_row=1, max_row=1))]
    funds = []
    for row in ws.iter_rows(min_row=2, values_only=True):
        d = dict(zip(headers, row))
        if d.get("ticker"):
            funds.append(d)
    return funds


def load_prospectus_fees(wb):
    """Load latest prospectus fee data per fund."""
    ws = wb["Prospectus_Fee_Tables"]
    headers = [c.value for c in next(ws.iter_rows(min_row=1, max_row=1))]
    latest = {}
    for row in ws.iter_rows(min_row=2, values_only=True):
        d = dict(zip(headers, row))
        ticker = d.get("ticker")
        if not ticker:
            continue
        # Keep the latest entry per ticker
        if ticker not in latest or str(d.get("date", "")) > str(latest[ticker].get("date", "")):
            latest[ticker] = d
    return latest


def load_annual_revenue(wb):
    """Load Annual_Revenue sheet."""
    ws = wb["Annual_Revenue"]
    headers = [c.value for c in next(ws.iter_rows(min_row=1, max_row=1))]
    records = {}
    for row in ws.iter_rows(min_row=2, values_only=True):
        d = dict(zip(headers, row))
        ticker = d.get("ticker")
        if not ticker:
            continue
        if ticker not in records:
            records[ticker] = []
        records[ticker].append(d)
    return records


def annualized_return(monthly_returns, years):
    """Compute annualized return from the last N years of monthly returns."""
    n_months = years * 12
    if len(monthly_returns) < n_months:
        return None
    recent = monthly_returns[-n_months:]
    cumulative = 1.0
    for r in recent:
        cumulative *= (1.0 + r)
    return cumulative ** (12.0 / n_months) - 1.0


def annualized_std_dev(monthly_returns, years=None):
    """Compute annualized standard deviation."""
    if years:
        n_months = years * 12
        if len(monthly_returns) < n_months:
            return None
        rets = monthly_returns[-n_months:]
    else:
        rets = monthly_returns
    if len(rets) < 2:
        return None
    mean = sum(rets) / len(rets)
    var = sum((r - mean) ** 2 for r in rets) / (len(rets) - 1)
    return math.sqrt(var) * math.sqrt(12)


def sharpe_ratio(monthly_returns, rf_annual=RF_ANNUAL):
    """Compute Sharpe ratio using all available returns."""
    if len(monthly_returns) < 12:
        return None
    ann_ret = annualized_return(monthly_returns, len(monthly_returns) // 12)
    if ann_ret is None:
        return None
    ann_std = annualized_std_dev(monthly_returns)
    if ann_std is None or ann_std == 0:
        return None
    return (ann_ret - rf_annual) / ann_std


def max_drawdown(monthly_returns):
    """Compute maximum drawdown from monthly returns."""
    if not monthly_returns:
        return None
    cumulative = 1.0
    peak = 1.0
    max_dd = 0.0
    for r in monthly_returns:
        cumulative *= (1.0 + r)
        if cumulative > peak:
            peak = cumulative
        dd = (peak - cumulative) / peak
        if dd > max_dd:
            max_dd = dd
    return -max_dd  # negative convention


def main():
    print(f"Loading {XLSX_PATH}...")
    wb = openpyxl.load_workbook(XLSX_PATH, data_only=True)

    fund_summaries = load_fund_summary(wb)
    prospectus_fees = load_prospectus_fees(wb)
    annual_revenue = load_annual_revenue(wb)

    results = []

    for fund in fund_summaries:
        ticker = fund["ticker"]
        print(f"  Processing {ticker}...")

        monthly = load_monthly_returns(wb, ticker)
        rets = [r["return"] for r in monthly]

        # Fee data
        fees = prospectus_fees.get(ticker, {})
        mgmt_fee = fund.get("latest_mgmt_fee_pct")
        exp_ratio = fund.get("latest_exp_ratio_pct")
        gross_er = fees.get("gross_er_pct")
        waiver = fees.get("waiver_pct", 0) or 0
        net_er = fees.get("net_er_pct")

        # Annual revenue data
        rev_data = annual_revenue.get(ticker, [])
        annual_rev = []
        for yr in rev_data:
            annual_rev.append({
                "year": str(yr.get("year", "")),
                "avg_aum": yr.get("avg_aum_usd"),
                "year_end_tna": yr.get("year_end_tna_usd"),
                "net_flow": yr.get("net_flow_usd"),
                "market_effect": yr.get("market_effect_usd"),
                "mgmt_fee_pct": yr.get("mgmt_fee_pct"),
                "exp_ratio_pct": yr.get("exp_ratio_pct"),
                "rev_mgmt_fee": yr.get("rev_mgmt_fee_usd"),
                "rev_total_expense": yr.get("rev_total_expense_usd"),
                "months": yr.get("months"),
            })

        result = {
            "ticker": ticker,
            "fund_name": fund.get("fund_name", ""),
            "vehicle": fund.get("vehicle", ""),
            "benchmark": fund.get("benchmark", ""),
            "first_offer_dt": str(fund.get("first_offer_dt", "")),
            "latest_tna": fund.get("latest_tna_usd"),
            "mgmt_fee_pct": mgmt_fee,
            "gross_expense_ratio_pct": gross_er,
            "waiver_pct": waiver,
            "net_expense_ratio_pct": net_er,
            "fee_after_waiver_pct": (gross_er - waiver) if gross_er is not None else net_er,
            "sharpe_ratio": sharpe_ratio(rets) if len(rets) >= 12 else None,
            "max_drawdown": max_drawdown(rets),
            "annualized_return_1yr": annualized_return(rets, 1),
            "annualized_return_3yr": annualized_return(rets, 3),
            "annualized_return_5yr": annualized_return(rets, 5),
            "annualized_return_10yr": annualized_return(rets, 10),
            "std_dev_1yr": annualized_std_dev(rets, 1),
            "std_dev_3yr": annualized_std_dev(rets, 3),
            "std_dev_5yr": annualized_std_dev(rets, 5),
            "std_dev_10yr": annualized_std_dev(rets, 10),
            "total_months": len(rets),
            "annual_revenue": annual_rev,
            "lifetime_rev_mgmt_fee": fund.get("lifetime_rev_mgmt_fee_usd"),
            "lifetime_rev_total_expense": fund.get("lifetime_rev_total_expense_usd"),
        }

        results.append(result)

    # Write JSON
    with open(OUTPUT_PATH, "w") as f:
        json.dump(results, f, indent=2, default=str)

    print(f"\nWrote {len(results)} funds to {OUTPUT_PATH}")


if __name__ == "__main__":
    main()
