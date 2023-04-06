use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use futures::future;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}

async fn health() -> impl Responder {
    "All good\n"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let s1 = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/say/hello", web::get().to(|| async { "Hello Again!" }))
    })
    .bind("127.0.0.1:8080")?
    .run();

    let s2 = HttpServer::new(move || {
        App::new().service(web::resource("/health").route(web::get().to(health)))
    })
    .bind("127.0.0.1:8000")?
    .run();

    future::try_join(s1, s2).await?;
    Ok(())
}
