use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StockReport {
    #[serde(rename = "latest-update")]
    pub latest_update: Option<i64>,
    pub ticker: String,
    pub version: i32, // Add the numeric version field
    pub data: Vec<Report>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Report {
    #[serde(rename = "year")]
    pub year: i32,
    #[serde(rename = "income-statement")]
    pub income_statement: IncomeStatement,

    #[serde(rename = "income-statement-yoy")]
    pub income_statement_yoy: Option<IncomeStatement>,

    #[serde(rename = "balance-sheet")]
    pub balance_sheet: BalanceSheet,

    #[serde(rename = "balance-sheet-yoy")]
    pub balance_sheet_yoy: Option<BalanceSheet>,
    
    #[serde(rename = "cash-flow-statement")]
    pub cash_flow_statement: CashFlowStatement,
    
    #[serde(rename = "cash-flow-statement-yoy")]
    pub cash_flow_statement_yoy: Option<CashFlowStatement>,
    
    #[serde(rename = "financial-ratios")]
    pub financial_ratios: FinancialRatios,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct IncomeStatement {
    #[serde(rename = "revenue")]
    pub revenue: f64,

    #[serde(rename = "total-cogs")]
    pub total_cogs: f64,

    #[serde(rename = "gross-profit")]
    pub gross_profit: Option<f64>, // revenue - total cogs

    #[serde(rename = "gross-profit-margin")]
    pub gross_profit_margin: Option<f64>, // gross_profit/revenue

    #[serde(rename = "operating-expense")]
    pub operating_expense: f64,

    #[serde(rename = "operating-income")]
    pub operating_income: Option<f64>, // gross_profit - operating_expense

    #[serde(rename = "operating-profit-margin")]
    pub operating_profit_margin: Option<f64>, // operating_income/revenue

    #[serde(rename = "interest-expense")]
    pub interest_expense: f64,

    #[serde(rename = "net-income")]
    pub net_income: f64,

    #[serde(rename = "net-profit-margin")]
    pub net_profit_margin: Option<f64>, //net_income/revenue

    #[serde(rename = "eps-basic")]
    pub eps_basic: f64,

    #[serde(rename = "shares-outstanding-basic")]
    pub shares_outstanding_basic: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct BalanceSheet {
    #[serde(rename = "cash-and-equivalents")]
    pub cash_and_equivalents: f64,
    #[serde(rename = "total-assets")]
    pub total_assets: f64,
    #[serde(rename = "short-term-debt")]
    pub short_term_debt: f64,
    #[serde(rename = "long-term-debt")]
    pub long_term_debt: f64,
    #[serde(rename = "total-liabilities")]
    pub total_liabilities: f64,

    #[serde(rename = "total-debt")]
    pub total_debt: Option<f64>,
    #[serde(rename = "total-equity")]
    pub total_equity: Option<f64>,
    #[serde(rename = "debt-to-capital")]
    pub debt_to_capital: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct CashFlowStatement {
    #[serde(rename = "operating-cash-flow")]
    pub operating_cash_flow: f64,
    #[serde(rename = "investing-cash-flow")]
    pub investing_cash_flow: f64,
    #[serde(rename = "capital-expenditure")]
    pub capital_expenditure: f64,
    #[serde(rename = "financing-cash-flow")]
    pub financing_cash_flow: f64,
    #[serde(rename = "dividends-paid")]
    pub dividends_paid: f64,
    #[serde(rename = "dividends-per-share")]
    pub dividends_per_share: f64,

    #[serde(rename = "free-cash-flow")]
    pub free_cash_flow: Option<f64>,
    #[serde(rename = "fcf-per-share")]
    pub fcf_per_share: Option<f64>,

}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct FinancialRatios {
    #[serde(rename = "avg-share-price")]
    pub avg_share_price: f64,
    
    #[serde(rename = "avg-yield")]
    pub avg_yield: Option<f64>, 

    #[serde(rename = "dividend-growth-rate")]
    pub dividend_growth_rate: Option<f64>, 

    #[serde(rename = "eps-payout-ratio")]
    pub eps_payout_ratio: Option<f64>, 

    #[serde(rename = "fcf-payout-ratio")]
    pub fcf_payout_ratio: Option<f64>, 

    #[serde(rename = "pe-ratio")]
    pub pe_ratio: Option<f64>, 

    #[serde(rename = "return-on-equity")]
    pub return_on_equity: Option<f64>, 

    #[serde(rename = "price-to-ebit")]
    pub price_to_ebit: Option<f64>, 

    #[serde(rename = "price-to-opcf")]
    pub price_to_opcf: Option<f64>, 

    #[serde(rename = "price-to-fcf")]
    pub price_to_fcf: Option<f64>, 

}
