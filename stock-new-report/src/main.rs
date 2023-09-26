use actix_web::{middleware::Logger, get, post, delete, web, App, HttpServer, HttpResponse};
use bson::doc;

use mongodb::{options::{ClientOptions}, Client};

use std::sync::Arc;
use log::{info, error};

mod report_model;
use report_model::{
    AnnualStockReport, Report
};

#[derive(Clone)]
struct Database {
    client: Client,
    db_name: String,
}

#[post("/add_report/{ticker}")]
async fn srv_add_report(
    ticker: web::Path<String>,
    report: web::Json<Report>,
    db: web::Data<Arc<Database>>
) -> impl actix_web::Responder {

    info!("/add_report/{}", ticker.as_str());

    let filter = doc! { "ticker": ticker.as_str() };

    let collection = db
        .client
        .database(db.db_name.as_str())
        .collection::<AnnualStockReport>("stock_reports");

    match collection.find_one(filter.clone(), Option::None).await {
        Err(err) => {
            error!("Eror whilge getting tickr: {:?} with err={:?}", ticker, err);
            HttpResponse::InternalServerError().body(format!("Failed to delete the ticker for: {:?}", ticker.as_str()))
        },
        Ok(stock) => {
            if let Option::Some(mut complete_report) = stock{
                complete_report.add_new_report(report.into_inner());

                // delete the old one
                match collection.delete_one(filter, None).await {
                    Ok(result) => {
                        if result.deleted_count != 1 {
                            return HttpResponse::InternalServerError().body(format!("failed to update the report for: {:?}", ticker.as_str()));
                        }
                    },
                    Err(err) => {
                        return HttpResponse::InternalServerError().body(format!("Internal error: {:?}", err));
                    }
                }

                // insert the new one
                insert_complete_report(&collection, complete_report).await

                // TODO: handle the case when delete is with sucess but insert failes by inverting the 2 one or by doing it by transaction
            }
            else{
                HttpResponse::Ok().body(format!("Ticker {} does not exists in db", ticker))
            }
        }
    }
}

async fn insert_complete_report(db_collection: &mongodb::Collection<AnnualStockReport>, complete_report: AnnualStockReport) -> actix_web::HttpResponse {
    let result = db_collection.insert_one(&complete_report, None).await;
    match result {
        Ok(insert_result) => {
            info!("Created id: {}", insert_result.inserted_id);
            actix_web::HttpResponse::Created().json(complete_report)
        },
        Err(err) => {
            error!("Error: {}", err);
            actix_web::HttpResponse::InternalServerError().body("")
        }
    }
}

#[post("/create_initial_report")]
async fn srv_create_initial_report(
    item: web::Json<AnnualStockReport>,
    db: web::Data<Arc<Database>>,
) -> impl actix_web::Responder {
    info!("{:?}", item);

    let db_collection = db
        .client
        .database(db.db_name.as_str())
        .collection::<AnnualStockReport>("stock_reports");

    insert_complete_report(&db_collection, AnnualStockReport::from(item)).await
}

#[delete("/item/{ticker}")]
async fn srv_delete_item(
    ticker: web::Path<String>,
    db: web::Data<Arc<Database>>
) -> impl actix_web::Responder 
{
    info!("{:?}", ticker.as_str());
    let filter = doc! { "ticker": ticker.as_str() };

    let collection = db
        .client
        .database(db.db_name.as_str())
        .collection::<AnnualStockReport>("stock_reports");

    match collection.find_one(filter.clone(), Option::None).await{
        Err(err) => {
            error!("Eror whilge getting tickr: {:?} with err={:?}", ticker, err);
            HttpResponse::InternalServerError().body(format!("Failed to delete the ticker for: {:?}", ticker.as_str()))
        },
        Ok(stock) => {
            if let Option::Some(report) = stock{
                let result = collection.delete_one(filter, None).await.unwrap();
                if result.deleted_count == 1 {
                    HttpResponse::Ok().body(format!("I've just deleted the following report {:?}", report))
                } else {
                    HttpResponse::InternalServerError().body(format!("failed to delete the dicker for: {:?}", ticker.as_str()))
                }
            }
            else{
                HttpResponse::Ok().body(format!("Ticker {} does not exists in db", ticker))
            }
        }
    }
}

#[get("/item/{ticker}")]
async fn srv_get_item(
    ticker: web::Path<String>,
    db: web::Data<Arc<Database>>
) -> impl actix_web::Responder 
{
    info!("{:?}", ticker.as_str());
    let filter = doc! { "ticker": ticker.as_str() };

    let report = db
        .client
        .database(db.db_name.as_str())
        .collection::<AnnualStockReport>("stock_reports")
        .find_one(filter, Option::None).await;

    match report {
        Err(err) => {
            error!("Failed to interact with db for getting ticker {:?}", err);
            actix_web::HttpResponse::InternalServerError().body("Failed to extract all items from DB")
        },
        Ok(opt_stock_report) => {
            if let Option::Some(report) = opt_stock_report{
                actix_web::HttpResponse::Ok().json(report)
            }
            else{
                actix_web::HttpResponse::NotFound().body("")
            }
        },
    }

}

#[get("/items")]
async fn srv_get_items(db: web::Data<Arc<Database>>) -> impl actix_web::Responder {
    let collection = db
        .client
        .database(&db.db_name)
        .collection::<AnnualStockReport>("stock_reports");

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
                    Err(_) => error!("Nothing temporary"),
                }
            }

            actix_web::HttpResponse::Ok().json(reports)
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
            .service(srv_create_initial_report)
            .service(srv_get_items)
            .service(srv_get_item)
            .service(srv_delete_item)
            .service(srv_add_report)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
