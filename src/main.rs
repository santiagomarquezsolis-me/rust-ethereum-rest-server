use actix_web::dev::ServiceRequest;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_web::middleware::Logger;
use actix_web_httpauth::middleware::HttpAuthentication;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{encode, decode, EncodingKey, DecodingKey, Header, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use web3::transports::Http;
use web3::Web3;
use web3::types::{Block, BlockId, H256, TransactionId, Address, BlockNumber, SyncState};

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

async fn get_transaction_details(node_url: web::Data<String>, tx_hash: web::Path<String>) -> impl Responder {
    let transport = match Http::new(&node_url) {
        Ok(transport) => transport,
        Err(_) => return HttpResponse::InternalServerError().body("Error creando el transporte"),
    };

    let web3 = Web3::new(transport);

    let tx_hash: H256 = match tx_hash.parse() {
        Ok(hash) => hash,
        Err(_) => return HttpResponse::InternalServerError().body("Error parseando el hash de la transacción"),
    };

    let transaction = match web3.eth().transaction(TransactionId::Hash(tx_hash)).await {
        Ok(tx) => tx,
        Err(_) => return HttpResponse::InternalServerError().body("Error obteniendo los detalles de la transacción"),
    };

    match transaction {
        Some(tx) => HttpResponse::Ok().body(format!("Detalles de la transacción: {:?}", tx)),
        None => HttpResponse::InternalServerError().body("Transacción no encontrada."),
    }
}

async fn get_balance(node_url: web::Data<String>, address: web::Path<String>) -> impl Responder {
    let transport = match Http::new(&node_url) {
        Ok(transport) => transport,
        Err(_) => return HttpResponse::InternalServerError().body("Error creando el transporte"),
    };

    let web3 = Web3::new(transport);

    let address: Address = match address.parse() {
        Ok(addr) => addr,
        Err(_) => return HttpResponse::InternalServerError().body("Error parseando la dirección"),
    };

    let balance = match web3.eth().balance(address, None).await {
        Ok(balance) => balance,
        Err(_) => return HttpResponse::InternalServerError().body("Error obteniendo el balance"),
    };

    HttpResponse::Ok().body(format!("Balance de la dirección {}: {}", address, balance))
}

async fn get_network_info(node_url: web::Data<String>) -> impl Responder {
    let transport = match Http::new(&node_url) {
        Ok(transport) => transport,
        Err(_) => return HttpResponse::InternalServerError().body("Error creando el transporte"),
    };

    let web3 = Web3::new(transport);

    let net_version = match web3.net().version().await {
        Ok(version) => version,
        Err(_) => return HttpResponse::InternalServerError().body("Error obteniendo la versión de la red"),
    };

    let peer_count = match web3.net().peer_count().await {
        Ok(count) => count,
        Err(_) => return HttpResponse::InternalServerError().body("Error obteniendo el número de peers conectados"),
    };

    HttpResponse::Ok().body(format!("Versión de la red: {}\nNúmero de peers conectados: {}", net_version, peer_count))
}

async fn get_sync_status(node_url: web::Data<String>) -> impl Responder {
    let transport = match Http::new(&node_url) {
        Ok(transport) => transport,
        Err(_) => return HttpResponse::InternalServerError().body("Error creando el transporte"),
    };

    let web3 = Web3::new(transport);

    let sync_status = match web3.eth().syncing().await {
        Ok(status) => status,
        Err(_) => return HttpResponse::InternalServerError().body("Error obteniendo el estado de sincronización"),
    };

    match sync_status {
        SyncState::Syncing(sync_info) => HttpResponse::Ok().body(format!("El nodo está sincronizando.\nEstado de sincronización: {:?}", sync_info)),
        SyncState::NotSyncing => HttpResponse::Ok().body("El nodo está completamente sincronizado.".to_string()),
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
            .service(
                web::resource("/transaction_details/{tx_hash}")
                    .wrap(HttpAuthentication::bearer(jwt_middleware))
                    .route(web::get().to(get_transaction_details))
            )
            .service(
                web::resource("/balance/{address}")
                    .wrap(HttpAuthentication::bearer(jwt_middleware))
                    .route(web::get().to(get_balance))
            )
            .service(
                web::resource("/network_info")
                    .wrap(HttpAuthentication::bearer(jwt_middleware))
                    .route(web::get().to(get_network_info))
            )
            .service(
                web::resource("/sync_status")
                    .wrap(HttpAuthentication::bearer(jwt_middleware))
                    .route(web::get().to(get_sync_status))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
