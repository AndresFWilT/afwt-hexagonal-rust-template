mod adapters;
mod application;
mod config;
mod domain;
mod infrastructure;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Initializing Server!");
    infrastructure::server::run::run_server().await
}
