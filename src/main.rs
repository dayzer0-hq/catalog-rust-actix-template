use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Health {
    status: &'static str,
}

// GET /healthz — the one route the starter ships with. Every catalog brief builds its
// own /v1/... endpoints on top of this; the skeleton only proves the service runs.
#[get("/healthz")]
async fn healthz() -> impl Responder {
    HttpResponse::Ok().json(Health { status: "ok" })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("listening on http://127.0.0.1:8080");
    HttpServer::new(|| App::new().service(healthz))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
