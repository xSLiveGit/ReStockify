use actix_web::{middleware::Logger, get, post, delete, web, App, HttpServer, Result, HttpResponse};
use bson::doc;
use chrono;
use chrono::Utc;
use mongodb::{options::{ClientOptions, FindOptions}, Client};
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
    let stock_report = StockReport::from(item);
    
    return web::Json(stock_report);

    // actix_web::HttpResponse::Created() // temporary while computing all ratios from reports

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

#[delete("/item/{ticker}")]
async fn delete_item(
    ticker: web::Path<String>,
    db: web::Data<Arc<Database>>
) -> Result<HttpResponse>
{
    println!("{:?}", ticker.as_str());
    let filter = doc! { "ticker": ticker.as_str() };

    let collection = db
        .client
        .database(db.db_name.as_str())
        .collection::<StockReport>("stock_reports");

    match collection.find_one(filter.clone(), Option::None).await{
        Err(err) => {
            println!("Eror whilge getting tickr: {:?} with err={:?}", ticker, err);
            Ok(HttpResponse::InternalServerError().body(format!("Failed to delete the ticker for: {:?}", ticker.as_str())))
        },
        Ok(stock) => {
            if let Option::Some(report) = stock{
                let result = collection.delete_one(filter, None).await.unwrap();
                if result.deleted_count == 1 {
                    print!("{:?}", report);
                    return Ok(HttpResponse::Ok().body(format!("I've just deleted the following report {:?}", report)));
                } else {
                    return Ok(HttpResponse::InternalServerError().body(format!("failed to delete the dicker for: {:?}", ticker.as_str())));
                }
            }
            else{
                return Ok(HttpResponse::Ok().body(format!("Ticker {} does not exists in db", ticker)));
            }
        }
    }
        
    
    
  

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
            .wrap(Logger::default()) // Use the Logger middleware to log requests
            .data(Arc::new(database.clone()))
            .service(create_item)
            .service(get_items)
            .service(delete_item)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
