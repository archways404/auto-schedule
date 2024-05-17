mod fetch;
mod parse;
mod compare;

use actix_files::NamedFile;
use actix_web::{get, App, HttpServer, Responder, HttpResponse};
use std::path::PathBuf;

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Fetch the ICS file before starting the server
    match fetch::fetch_ics().await {
        Ok(_) => {
            println!("ICS file fetched successfully");
            parse::filter_and_modify_ics()?;
            println!("ICS file filtered and modified successfully");
            compare::compare_and_update_files()?;
            println!("ICS files compared and updated if necessary");
        }
        Err(e) => eprintln!("Failed to fetch ICS file: {}", e),
    }

    HttpServer::new(|| {
        App::new()
            .service(get_ics)
            .service(index)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
