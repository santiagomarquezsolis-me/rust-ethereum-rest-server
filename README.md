
# Rust Actix Web Service

This is a Rust web service built using the Actix Web framework. The service includes JWT-based authentication and interacts with the Ethereum blockchain to fetch gas prices.

## Table of Contents

- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Running the Service](#running-the-service)
- [Endpoints](#endpoints)
- [Middleware](#middleware)
- [License](#license)

## Features

- Basic web server using Actix Web.
- JWT-based authentication.
- Fetching Ethereum gas price using `web3` crate.
- Logging middleware for request logging.

## Prerequisites

- Rust and Cargo installed. You can install Rust from [rust-lang.org](https://www.rust-lang.org/).
- An Ethereum node URL. You can use services like [Infura](https://infura.io/) to get a free node URL.

## Installation

1. Clone the repository:

    \`\`\`sh
    git clone <repository_url>
    cd <repository_directory>
    \`\`\`

2. Build the project:

    \`\`\`sh
    cargo build
    \`\`\`

## Running the Service

To run the web service, use the following command:

\`\`\`sh
cargo run
\`\`\`

The service will start on \`127.0.0.1:8080\`.

## Endpoints

### \`GET /\`

Returns a simple greeting.

- **Response**: \`Hello, world!\`

### \`POST /login\`

Generates a JWT token.

- **Response**: JWT token string.

### \`GET /gas_price\`

Fetches the current Ethereum gas price. Requires a valid JWT token.

- **Headers**:
    - \`Authorization\`: Bearer \`<JWT token>\`

- **Response**: Current gas price.

## Middleware

### Logger

Logs incoming requests.

### JWT Middleware

Validates JWT tokens for protected routes.

#### JWT Middleware Implementation

\`\`\`rust
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
\`\`\`

## License

This project is licensed under the MIT License.

---

## Code Explanation

### Structs and Enums

- **Claims**: Represents the structure of the JWT token claims.

    \`\`\`rust
    #[derive(Debug, Serialize, Deserialize)]
    struct Claims {
        sub: String,
        company: String,
        exp: usize,
    }
    \`\`\`

### Handlers

- **index**: Handles the \`/\` endpoint, returning a simple greeting.

    \`\`\`rust
    async fn index() -> impl Responder {
        HttpResponse::Ok().body("Hello, world!")
    }
    \`\`\`

- **login**: Handles the \`/login\` endpoint, generating a JWT token.

    \`\`\`rust
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
    \`\`\`

- **get_gas_price**: Handles the \`/gas_price\` endpoint, fetching the current Ethereum gas price.

    \`\`\`rust
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
    \`\`\`

### Main Function

Sets up the Actix web server with routes and middleware.

\`\`\`rust
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
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
\`\`\`

This README provides a comprehensive guide to the Rust Actix Web service, covering setup, usage, and key code components.
