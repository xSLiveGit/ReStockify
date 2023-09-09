use actix_web::{post, get, App, HttpServer, web, Result};
use bson::doc;
use mongodb::{Client, options::ClientOptions};
use std::sync::Arc;
use serde::{Serialize};
use serde_json;

mod report_model;
use report_model::StockReport;

#[derive(Clone)]
struct Database {
    client: Client,
}

#[post("/items")]
async fn create_item(item: web::Json<StockReport>, db: web::Data<Arc<Database>>) -> impl actix_web::Responder {
    println!("{:?}", item);

    let collection = db.client.database("mydb").collection::<StockReport>("stock_reports");

    // Insert the received JSON data into the MongoDB collection
    let stock_report = item.into_inner();
    let result = collection.insert_one(stock_report, None).await;

    match result {
        Ok(_) => actix_web::HttpResponse::Created(),
        Err(_) => actix_web::HttpResponse::InternalServerError(),
    }

}

#[get("/items")]
async fn get_items(db: web::Data<Arc<Database>>) -> impl actix_web::Responder {
    let collection = db.client.database("mydb").collection::<StockReport>("stock_reports");

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

// #[get("/items/{ticker}")]
// async fn get_items(
//     path: web::Path<(String)>,
//     db: web::Data<Arc<Database>>,
// ) -> impl actix_web::Responder {
//     let collection = db.client.database("mydb").collection::<StockReport>("stock_reports");
//     let (tiker) = path.into_inner();
//
//     // Define the query to retrieve all reports for the stock ticker
//     let filter = doc! { "ticker": ticker };
//
//     // Retrieve all reports for the stock ticker based on the query
//     let cursor = collection.find(filter, None).await;
//
//     match cursor {
//         Ok(mut cursor) => {
//             let mut reports = Vec::new();
//             while let Some(result) = cursor.next().await {
//                 match result {
//                     Ok(report) => reports.push(report),
//                     Err(_) => return actix_web::HttpResponse::InternalServerError(),
//                 }
//             }
//
//             if !reports.is_empty() {
//                 return actix_web::HttpResponse::Ok().json(reports).into();
//             } else {
//                 return actix_web::HttpResponse::NotFound().body("No reports found for the stock ticker").into();
//             }
//         }
//         Err(_) => { return actix_web::HttpResponse::InternalServerError().into();},
//     }
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    
    // Create a MongoDB client
    let client = Client::with_options(client_options).unwrap();

    // Check if the "mydb" database exists; if not, create it
    let databases = client.list_database_names(None, None).await.unwrap();
    if !databases.contains(&"mydb".to_string()) {
        client.database("mydb").create_collection("stock_reports", None).await.unwrap();
    }

    // Create a data structure to share the MongoDB client across Actix threads
    let database = Database { client };

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