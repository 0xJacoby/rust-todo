use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_files::Files;

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("Pong!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(ping)
            .service(Files::new("/", "www/dist")
                .prefer_utf8(true)
                .index_file("index.html")
            )
    })
    .bind(("127.0.0.1", 8000))? 
    .run()
    .await
}
