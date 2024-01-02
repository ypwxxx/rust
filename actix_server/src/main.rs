use std::sync::Mutex;

use actix_web::{get, guard, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

struct AppState {
    app_name: String,
    counter: Mutex<i32>,
}

#[get("/test1")]
async fn test1(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;

    format!("Hello {app_name}!")
}

#[get("/test2")]
async fn test2(data: web::Data<AppState>) -> String {
    let mut counter = data.counter.lock().unwrap();

    *counter += 1;
    format!("Request number: {counter}!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        app_name: String::from("Test level 2"),
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new().app_data(app_data.clone()).service(
            web::scope("/server")
                .service(hello)
                .service(echo)
                .service(test1)
                .service(test2)
                .route("/hey", web::get().to(manual_hello)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
