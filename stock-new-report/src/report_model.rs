use chrono::Utc;
use serde::{Deserialize, Serialize};

use actix_web::{web};
use bson::doc;
use chrono;
use serde_json;

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

impl Report {
    fn compute_optional_if_required(&mut self, last_stock: &Option<&Report>){
        self.income_statement.compute_optional_if_required();
        self.balance_sheet.compute_optional_if_required();
        self.cash_flow_statement.compute_optional_if_required(&self.income_statement);
        self.financial_ratios.compute_optional_if_required(
            &self.cash_flow_statement,
            &self.income_statement,
            &self.balance_sheet,
            &last_stock,
        );

        if let Some(last) = last_stock {
            let _ = self.income_statement_yoy.insert(IncomeStatement::from_as_yoy(&self.income_statement, &last.income_statement));
            let _ = self.balance_sheet_yoy.insert(BalanceSheet::from_as_yoy(&self.balance_sheet, &last.balance_sheet));
            let _ = self.cash_flow_statement_yoy.insert(CashFlowStatement::from_as_yoy(&self.cash_flow_statement, &last.cash_flow_statement));
        }
    }
}

impl IncomeStatement {
    fn compute_optional_if_required(&mut self) {
        let _ = self.gross_profit.insert(self.revenue - self.total_cogs);

        let _ = self
            .gross_profit_margin
            .get_or_insert(self.gross_profit.unwrap() - self.total_cogs);
        let operating_income = self
            .operating_income
            .get_or_insert(self.gross_profit.unwrap() - self.operating_expense)
            .clone();
        let _ = self
            .operating_profit_margin
            .get_or_insert(self.operating_income.unwrap() / self.revenue);
        let _ = self
            .net_profit_margin
            .get_or_insert(self.net_income / self.revenue);
    }

    fn from_as_yoy(current: &IncomeStatement, last: &IncomeStatement) -> IncomeStatement {
        IncomeStatement {
            revenue: current.revenue / last.revenue - 1.0,
            total_cogs: current.total_cogs / last.total_cogs - 1.0,
            gross_profit: Some(current.gross_profit.unwrap() / last.gross_profit.unwrap() - 1.0),
            gross_profit_margin: Some(current.gross_profit_margin.unwrap() / last.gross_profit_margin.unwrap() - 1.0),
            operating_expense: current.operating_expense / last.operating_expense - 1.0,
            operating_income: Some(current.operating_income.unwrap() / last.operating_income.unwrap() - 1.0),
            operating_profit_margin: Some(current.operating_profit_margin.unwrap() / last.operating_profit_margin.unwrap() - 1.0),
            interest_expense: current.interest_expense / last.interest_expense - 1.0,
            net_income: current.net_income / last.net_income - 1.0,
            net_profit_margin: Some(current.net_profit_margin.unwrap() / last.net_profit_margin.unwrap() - 1.),
            eps_basic: current.eps_basic / last.eps_basic - 1.0,
            shares_outstanding_basic: current.shares_outstanding_basic / last.shares_outstanding_basic - 1.0,
        }
    }
}

impl BalanceSheet {
    fn compute_optional_if_required(&mut self) {
        let _ = self
            .total_debt
            .insert(self.short_term_debt + self.long_term_debt);

        let total_equity = self
            .total_equity
            .insert(self.total_assets + self.total_liabilities)
            .clone();

        let _ = self.debt_to_capital.insert(
            self.total_debt.unwrap() / (self.total_debt.unwrap() + self.total_equity.unwrap()),
        );
    }

    fn from_as_yoy(current: &BalanceSheet, last: &BalanceSheet) -> BalanceSheet{
        BalanceSheet {
            cash_and_equivalents: (current.cash_and_equivalents
                / last.cash_and_equivalents
                - 1.0),
            total_assets: current.total_assets / last.total_assets - 1.0,
            short_term_debt: current.short_term_debt / last.short_term_debt - 1.0,
            long_term_debt: current.long_term_debt / last.long_term_debt - 1.0,
            total_liabilities: current.total_liabilities / last.total_liabilities - 1.0,
            total_debt: Some(current.total_debt.unwrap() / last.total_debt.unwrap() - 1.0),
            total_equity: Some(
                current.total_equity.unwrap() / last.total_equity.unwrap() - 1.0,
            ),
            debt_to_capital: Some(
                current.debt_to_capital.unwrap() / last.debt_to_capital.unwrap() - 1.0,
            ),
        }
    }
}

impl CashFlowStatement {
    fn compute_optional_if_required(&mut self, in_state: &IncomeStatement) {
        let fcf: f64 = self
            .free_cash_flow
            .insert(self.operating_cash_flow - self.capital_expenditure)
            .clone();

        let _ = self
            .fcf_per_share
            .insert(fcf - in_state.shares_outstanding_basic);
    }

    fn from_as_yoy(current: &CashFlowStatement, last: &CashFlowStatement) -> CashFlowStatement {
        CashFlowStatement {
            operating_cash_flow: (current.operating_cash_flow
                / last.operating_cash_flow),
            investing_cash_flow: (current.investing_cash_flow
                / last.investing_cash_flow),
            capital_expenditure: (current.capital_expenditure
                / last.capital_expenditure),
            financing_cash_flow: (current.financing_cash_flow
                / last.financing_cash_flow),
            dividends_paid: (current.dividends_paid / last.dividends_paid),
            dividends_per_share: (current.dividends_per_share
                / last.dividends_per_share),
            free_cash_flow: (current.free_cash_flow.unwrap()
                / last.free_cash_flow.unwrap())
            .into(),
            fcf_per_share: (current.fcf_per_share.unwrap()
                / last.fcf_per_share.unwrap())
            .into(),
        }
    }
}

impl FinancialRatios {
    fn compute_optional_if_required(
        &mut self,
        current_cfs: &CashFlowStatement,
        in_state: &IncomeStatement,
        bl_sheet: &BalanceSheet,
        last_result: &Option<&Report>,
    ) {
        let _ = self
            .avg_yield
            .insert(current_cfs.dividends_per_share / self.avg_share_price);

        if let Some(lst_stock) = last_result {
            let _ = self.dividend_growth_rate.insert(
                current_cfs.dividends_per_share / lst_stock.cash_flow_statement.dividends_per_share,
            );
        }

        let _ = self
            .eps_payout_ratio
            .insert(current_cfs.dividends_per_share / in_state.eps_basic);

        let _ = self
            .fcf_payout_ratio
            .insert(current_cfs.dividends_per_share / current_cfs.free_cash_flow.unwrap());
        let _ = self
            .pe_ratio
            .insert(self.avg_share_price / in_state.eps_basic);
        let _ = self
            .return_on_equity
            .insert(in_state.net_income / bl_sheet.total_equity.unwrap());
        let _ = self
            .price_to_ebit
            .insert(self.avg_share_price / in_state.operating_income.unwrap());
        let _ = self
            .price_to_opcf
            .insert(self.avg_share_price / current_cfs.operating_cash_flow);
        let _ = self
            .price_to_fcf
            .insert(self.avg_share_price / current_cfs.free_cash_flow.unwrap());
    }
}


impl StockReport {
    pub fn from(json: web::Json<StockReport>) -> StockReport {
        let mut report = json.into_inner();
        let _ = report.latest_update.get_or_insert(Utc::now().timestamp());
        report.compute_optional_if_required();
        return report;
    } 

    pub fn compute_optional_if_required(&mut self) {
        let mut last_stock: Option<&Report> = Option::None;

        for stock in self.data.iter_mut() {
            // let in_state = &mut stock.income_statement;
            // let bl_sh = &mut stock.balance_sheet;
            // let cash_state = &mut stock.cash_flow_statement;
            // let ratio = &mut stock.financial_ratios;
            stock.compute_optional_if_required(&last_stock);

           
            println!("{}", serde_json::to_string_pretty(&stock).unwrap());
            let _ = last_stock.insert(stock);
        }
    }
}
