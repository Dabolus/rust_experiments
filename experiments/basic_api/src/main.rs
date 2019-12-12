use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{id}/{name}")]
async fn parametrized(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello, {}! id: {}", info.1, info.0)
}

#[get("/*")]
async fn index() -> impl Responder {
    "Hello everything else!"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    // Start http server
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(parametrized)
            .service(index)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .start();

    println!("Server listening on port {}", port);

    server.await
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
