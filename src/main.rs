mod fetch;
mod parse;
mod compare;

use actix_files::NamedFile;
use actix_web::{get, App, HttpServer, Responder, HttpResponse};
use std::path::PathBuf;
use tokio::time::{sleep, Duration};
use std::io::Result;

static ICS_FILE: &str = "latest_calendar.ics";

#[get("/calendar.ics")]
async fn get_ics() -> actix_web::Result<NamedFile> {
    let path: PathBuf = ICS_FILE.parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("ICS Webserver is running")
}

async fn fetch_and_process_ics() -> Result<()> {
    match fetch::fetch_ics().await {
        Ok(_) => {
            println!("ICS file fetched successfully");
            if let Err(e) = parse::filter_and_modify_ics() {
                eprintln!("Failed to filter and modify ICS file: {}", e);
            } else {
                println!("ICS file filtered and modified successfully");
                if let Err(e) = compare::compare_and_update_files() {
                    eprintln!("Failed to compare and update ICS files: {}", e);
                } else {
                    println!("ICS files compared and updated if necessary");
                }
            }
        }
        Err(e) => eprintln!("Failed to fetch ICS file: {}", e),
    }
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tokio::spawn(async {
        loop {
            if let Err(e) = fetch_and_process_ics().await {
                eprintln!("Error during fetch and process: {}", e);
            }
            sleep(Duration::from_secs(3600)).await; // Sleep for 1 hours
        }
    });

    HttpServer::new(|| {
        App::new()
            .service(get_ics)
            .service(index)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
