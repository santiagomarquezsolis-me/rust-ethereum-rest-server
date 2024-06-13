use actix_web::dev::ServiceRequest;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_web::middleware::Logger;
use actix_web_httpauth::middleware::HttpAuthentication;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{encode, decode, EncodingKey, DecodingKey, Header, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use web3::transports::Http;
use web3::Web3;
use web3::types::{Block, BlockId, BlockNumber};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

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

async fn get_latest_block(node_url: web::Data<String>) -> impl Responder {
    let transport = match Http::new(&node_url) {
        Ok(transport) => transport,
        Err(_) => return HttpResponse::InternalServerError().body("Error creando el transporte"),
    };

    let web3 = Web3::new(transport);

    let block_number = match web3.eth().block_number().await {
        Ok(block_number) => block_number,
        Err(_) => return HttpResponse::InternalServerError().body("Error obteniendo el número del bloque"),
    };

    let block: Option<Block<_>> = match web3.eth().block(BlockId::Number(BlockNumber::Number(block_number))).await {
        Ok(block) => block,
        Err(_) => return HttpResponse::InternalServerError().body("Error obteniendo el bloque"),
    };

    match block {
        Some(block) => HttpResponse::Ok().body(format!("Último bloque: {:?}", block)),
        None => HttpResponse::InternalServerError().body("No se pudo obtener el bloque."),
    }
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

async fn login() -> impl Responder {
    let claims = Claims {
        sub: "b@b.com".to_owned(),
        company: "ACME".to_owned(),
        exp: 10000000000,
    };

    let token = match encode(&Header::default(), &claims, &EncodingKey::from_secret("my_secret_key".as_ref())) {
        Ok(t) => t,
        Err(_) => return HttpResponse::InternalServerError().body("Error generando el token"),
    };

    HttpResponse::Ok().body(token)
}

async fn jwt_middleware(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, actix_web::Error> {
    let token = credentials.token();
    let secret = "my_secret_key";

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    );

    match token_data {
        Ok(_) => Ok(req),
        Err(_) => Err(actix_web::error::ErrorUnauthorized("Invalid token")),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let node_url =  "https://sepolia.infura.io/v3/e8e126fe1041436a97258323079a0708".to_string();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(node_url.clone()))
            .route("/", web::get().to(index))
            .route("/login", web::post().to(login))
            .service(
                web::resource("/gas_price")
                    .wrap(HttpAuthentication::bearer(jwt_middleware))
                    .route(web::get().to(get_gas_price))
            )
            .service(
                web::resource("/latest_block")
                    .wrap(HttpAuthentication::bearer(jwt_middleware))
                    .route(web::get().to(get_latest_block))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
