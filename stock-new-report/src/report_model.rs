use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Debug, Serialize, Deserialize)]
pub struct StockReport {
    ticker: String,
    version: i32, // Add the numeric version field
    data: HashMap<String, Report>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
    #[serde(rename = "income-statement")]
    income_statement: IncomeStatement,
    #[serde(rename = "balance-sheet")]
    balance_sheet: BalanceSheet,
    #[serde(rename = "cash-flow-statement")]
    cash_flow_statement: CashFlowStatement,
    #[serde(rename = "financial-ratios")]
    financial_ratios: FinancialRatios,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeStatement {
    revenue: f64,
    #[serde(rename = "total-cogs")]
    total_cogs: f64,
    #[serde(rename = "operating-expense")]
    operating_expense: f64,
    #[serde(rename = "interest-expense")]
    interest_expense: f64,
    #[serde(rename = "net-income")]
    net_income: f64,
    #[serde(rename = "eps-basic")]
    eps_basic: f64,
    #[serde(rename = "shares-outstanding-basic")]
    shares_outstanding_basic: f64,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct BalanceSheet {
    #[serde(rename = "cash-and-equivalents")]
    cash_and_equivalents: f64,
    #[serde(rename = "total-assets")]
    total_assets: f64,
    #[serde(rename = "short-term-debt")]
    short_term_debt: f64,
    #[serde(rename = "long-term-debt")]
    long_term_debt: f64,
    #[serde(rename = "total-liabilities")]
    total_liabilities: f64,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct CashFlowStatement {
    #[serde(rename = "operating-cash-flow")]
    operating_cash_flow: f64,
    #[serde(rename = "investing-cash-flow")]
    investing_cash_flow: f64,
    #[serde(rename = "capital-expenditure")]
    capital_expenditure: f64,
    #[serde(rename = "financing-cash-flow")]
    financing_cash_flow: f64,
    #[serde(rename = "dividends-paid")]
    dividends_paid: f64,
    #[serde(rename = "dividends-per-share")]
    dividends_per_share: f64,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct FinancialRatios {
    #[serde(rename = "avg-share-price")]
    avg_share_price: f64,
    #[serde(rename = "PE-ratio")]
    pe_ratio: Option<f64>, // Make it optional
}