use actix_web::{get, post, web, App, HttpServer, Result};
use bson::doc;
use chrono;
use chrono::Utc;
use mongodb::{options::ClientOptions, Client};
use serde::Serialize;
use serde_json;
use serde_json::json;
use std::sync::Arc;

mod report_model;
use report_model::{
    BalanceSheet, CashFlowStatement, FinancialRatios, IncomeStatement, StockReport,
};

#[derive(Clone)]
struct Database {
    client: Client,
    db_name: String,
}

#[post("/push_initial_report")]
async fn create_item(
    item: web::Json<StockReport>,
    db: web::Data<Arc<Database>>,
) -> impl actix_web::Responder {
    println!("{:?}", item);

    let collection = db
        .client
        .database(db.db_name.as_str())
        .collection::<StockReport>("stock_reports");

    // Insert the received JSON data into the MongoDB collection
    let mut stock_report = item.into_inner();
    let _ = stock_report
        .latest_update
        .get_or_insert(Utc::now().timestamp());
    let mut last_stock: Option<&report_model::Report> = Option::None;

    println!("{:?}", stock_report);
    for stock in stock_report.data.iter_mut() {
        let in_state = &mut stock.income_statement;
        let bl_sh = &mut stock.balance_sheet;
        let cash_state = &mut stock.cash_flow_statement;

        let _ = in_state
            .gross_profit
            .insert(in_state.revenue - in_state.total_cogs);
        let _ = in_state
            .gross_profit_margin
            .insert(in_state.gross_profit.unwrap() - in_state.total_cogs);
        let _ = in_state
            .operating_income
            .insert(in_state.gross_profit.unwrap() - in_state.operating_expense);
        let _ = in_state
            .operating_profit_margin
            .insert(in_state.operating_income.unwrap() / in_state.revenue);
        let _ = in_state
            .net_profit_margin
            .insert(in_state.net_income / in_state.revenue);

        let _ = bl_sh
            .total_debt
            .insert(bl_sh.short_term_debt + bl_sh.long_term_debt);
        let _ = bl_sh
            .total_equity
            .insert(bl_sh.total_assets + bl_sh.total_liabilities);
        let _ = bl_sh
            .debt_to_capital
            .insert(bl_sh.total_debt.unwrap() / (bl_sh.total_debt.unwrap() + bl_sh.total_equity.unwrap()));

        let fcf: f64 = cash_state.free_cash_flow.insert(cash_state.operating_cash_flow-cash_state.capital_expenditure).clone();
        let _ = cash_state.fcf_per_share.insert(fcf-in_state.shares_outstanding_basic);

        if let Some(lst_stock) = last_stock {
            let in_state_l = &lst_stock.income_statement;
            let bl_sh_l = &lst_stock.balance_sheet;
            let cash_state_l = &lst_stock.cash_flow_statement;

            let _ = stock.income_statement_yoy.insert(IncomeStatement {
                revenue: in_state.revenue / in_state_l.revenue - 1.0,
                total_cogs: in_state.total_cogs / in_state_l.total_cogs - 1.0,
                gross_profit: Some(
                    in_state.gross_profit.unwrap() / in_state_l.gross_profit.unwrap() - 1.0,
                ),
                gross_profit_margin: Some(
                    in_state.gross_profit_margin.unwrap() / in_state_l.gross_profit_margin.unwrap()
                        - 1.0,
                ),
                operating_expense: in_state.operating_expense / in_state_l.operating_expense - 1.0,
                operating_income: Some(
                    in_state.operating_income.unwrap() / in_state_l.operating_income.unwrap() - 1.0,
                ),
                operating_profit_margin: Some(
                    in_state.operating_profit_margin.unwrap()
                        / in_state_l.operating_profit_margin.unwrap()
                        - 1.0,
                ),
                interest_expense: in_state.interest_expense / in_state_l.interest_expense - 1.0,
                net_income: in_state.net_income / in_state_l.net_income - 1.0,
                net_profit_margin: Some(
                    in_state.net_profit_margin.unwrap() / in_state_l.net_profit_margin.unwrap()
                        - 1.0,
                ),
                eps_basic: in_state.eps_basic / in_state_l.eps_basic - 1.0,
                shares_outstanding_basic: in_state.shares_outstanding_basic
                    / in_state_l.shares_outstanding_basic
                    - 1.0,
            });

            let _ = stock.balance_sheet_yoy.insert(BalanceSheet {
                cash_and_equivalents: bl_sh.cash_and_equivalents / bl_sh_l.cash_and_equivalents - 1.0,
                total_assets: bl_sh.total_assets / bl_sh_l.total_assets - 1.0,
                short_term_debt: bl_sh.short_term_debt / bl_sh_l.short_term_debt - 1.0,
                long_term_debt: bl_sh.long_term_debt / bl_sh_l.long_term_debt - 1.0,
                total_liabilities: bl_sh.total_liabilities / bl_sh_l.total_liabilities - 1.0,
                total_debt: Some(bl_sh.total_debt.unwrap() / bl_sh_l.total_debt.unwrap() - 1.0),
                total_equity: Some(bl_sh.total_equity.unwrap() / bl_sh_l.total_equity.unwrap() - 1.0),
                debt_to_capital: Some(bl_sh.debt_to_capital.unwrap() / bl_sh_l.debt_to_capital.unwrap() - 1.0),
            });

            let _ = stock.cash_flow_statement_yoy.insert(CashFlowStatement { 
                operating_cash_flow: (cash_state.operating_cash_flow / cash_state_l.operating_cash_flow), 
                investing_cash_flow: (cash_state.investing_cash_flow / cash_state_l.investing_cash_flow), 
                capital_expenditure: (cash_state.capital_expenditure / cash_state_l.capital_expenditure), 
                financing_cash_flow: (cash_state.financing_cash_flow / cash_state_l.financing_cash_flow), 
                dividends_paid: (cash_state.dividends_paid / cash_state_l.dividends_paid), 
                dividends_per_share: (cash_state.dividends_per_share / cash_state_l.dividends_per_share), 
                free_cash_flow: (cash_state.free_cash_flow.unwrap() / cash_state_l.free_cash_flow.unwrap()).into(), 
                fcf_per_share: (cash_state.fcf_per_share.unwrap() / cash_state_l.fcf_per_share.unwrap()).into() 
            });
            // inStateYoy.eps_basic
        }

        println!("{}", serde_json::to_string_pretty(&stock).unwrap());
        let _ = last_stock.insert(stock);
    }
    actix_web::HttpResponse::Created() // temporary while computing all ratios from reports

    // let result = collection.insert_one(stock_report, None).await;
    // match result {
    //     Ok(insert_result) => {
    //         println!("Created id: {}", insert_result.inserted_id);
    //         actix_web::HttpResponse::Created()
    //     },
    //     Err(err) => {
    //         println!("Error: {}", err);
    //         actix_web::HttpResponse::InternalServerError()
    //     }
    // }
}

#[get("/items")]
async fn get_items(db: web::Data<Arc<Database>>) -> impl actix_web::Responder {
    let collection = db
        .client
        .database(&db.db_name)
        .collection::<StockReport>("stock_reports");

    // Retrieve all stock reports from the collection
    let cursor = collection.find(doc! {}, None).await;

    match cursor {
        Ok(mut cursor) => {
            let mut reports = Vec::new();

            loop {
                let result = cursor.advance().await;
                if result.is_err() {
                    return actix_web::HttpResponse::InternalServerError()
                        .body("Failed to extract all items from DB");
                }

                if !result.unwrap() {
                    break;
                }

                let report = cursor.deserialize_current();
                match report {
                    Ok(report) => reports.push(report),
                    Err(_) => print!("Nothing temporary"),
                }
            }

            if let Ok(serialised) = serde_json::to_string(&reports) {
                println!("{:?}", serialised);
                return actix_web::HttpResponse::NotFound().body(serialised);
            } else {
                return actix_web::HttpResponse::InternalServerError()
                    .body("Failed to serialize all items from DB");
            }
        }
        Err(_) => {
            return actix_web::HttpResponse::InternalServerError()
                .body("Failed to interact with DB");
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .unwrap();
    let db_name = String::from("anual-reports-db");
    // Create a MongoDB client
    let client = Client::with_options(client_options).unwrap();

    // Check if the "mydb" database exists; if not, create it
    let databases = client.list_database_names(None, None).await.unwrap();
    if !databases.contains(&db_name.to_string()) {
        client
            .database(db_name.as_str())
            .create_collection("stock_reports", None)
            .await
            .unwrap();
    }

    // Create a data structure to share the MongoDB client across Actix threads
    let database = Database {
        client: client,
        db_name: db_name,
    };

    HttpServer::new(move || {
        App::new()
            .data(Arc::new(database.clone()))
            .service(create_item)
            .service(get_items)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
