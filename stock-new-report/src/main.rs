use actix_web::{post, get, App, HttpServer, web, Result};
use bson::doc;
use mongodb::{Client, options::ClientOptions};
use std::sync::Arc;
use serde::{Serialize};
use serde_json;
use chrono;
use chrono::Utc;


mod report_model;
use report_model::StockReport;


#[derive(Clone)]
struct Database {
    client: Client,
    db_name: String
}

#[post("/push_initial_report")]
async fn create_item(item: web::Json<StockReport>, db: web::Data<Arc<Database>>) -> impl actix_web::Responder {
    println!("{:?}", item);

    let collection = db.client.database(db.db_name.as_str()).collection::<StockReport>("stock_reports");
    
    // Insert the received JSON data into the MongoDB collection
    let mut stock_report = item.into_inner();
    let _ = stock_report.latest_update.get_or_insert(Utc::now().timestamp());

    let result = collection.insert_one(stock_report, None).await;

    match result {
        Ok(insert_result) => {
            println!("Created id: {}", insert_result.inserted_id);
            actix_web::HttpResponse::Created()
        },
        Err(err) => {
            println!("Error: {}", err);
            actix_web::HttpResponse::InternalServerError()
        }
    }

}

#[get("/items")]
async fn get_items(db: web::Data<Arc<Database>>) -> impl actix_web::Responder {
    let collection = db.client.database(&db.db_name).collection::<StockReport>("stock_reports");

    // Retrieve all stock reports from the collection
    let cursor = collection.find(doc! {}, None).await;


    match cursor {
        Ok(mut cursor) => {
            let mut reports = Vec::new();
            
            loop{
                let result = cursor.advance().await;
                if result.is_err() {
                    return actix_web::HttpResponse::InternalServerError().body("Failed to extract all items from DB");
                }

                if !result.unwrap() {
                    break;
                }

                let report = cursor.deserialize_current();
                match report {
                    Ok(report) => reports.push(report),
                    Err(_) => print!("Nothing temporary")
                }
            }


            if let Ok(serialised) = serde_json::to_string(&reports) {
                println!("{:?}", serialised);
                return actix_web::HttpResponse::NotFound().body(serialised);
            }
            else {
                return actix_web::HttpResponse::InternalServerError().body("Failed to serialize all items from DB");
            }
        }
        Err(_) => {
            return actix_web::HttpResponse::InternalServerError().body("Failed to interact with DB");
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let db_name = String::from("anual-reports-db");
    // Create a MongoDB client
    let client = Client::with_options(client_options).unwrap();

    // Check if the "mydb" database exists; if not, create it
    let databases = client.list_database_names(None, None).await.unwrap();
    if !databases.contains(&db_name.to_string()) {
        client.database(db_name.as_str()).create_collection("stock_reports", None).await.unwrap();
    } 

    // Create a data structure to share the MongoDB client across Actix threads
    let database = Database { client: client, db_name: db_name };

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