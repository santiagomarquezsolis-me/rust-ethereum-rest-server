use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use web3::transports::Http;
use web3::Web3;

async fn get_gas_price(node_url: web::Data<String>) -> impl Responder {
    let transport = match Http::new(&node_url) {
        Ok(transport) => transport,
        Err(_) => return HttpResponse::InternalServerError().body("Error creando el transporte"),
    };
    
    let web3 = Web3::new(transport);

    let gas_price = match web3.eth().gas_price().await {
        Ok(gas_price) => gas_price,
        Err(_) => return HttpResponse::InternalServerError().body("Error obteniendo el precio del gas"),
    };

    HttpResponse::Ok().body(format!("Precio del gas: {}", gas_price))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let node_url = "https://sepolia.infura.io/v3/e8e126fe1041436a97258323079a0708".to_string();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(node_url.clone()))
            .route("/gas_price", web::get().to(get_gas_price))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
