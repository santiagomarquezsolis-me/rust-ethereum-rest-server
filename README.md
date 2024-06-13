
# Rust Ethereum API with Actix-web

This is a REST API built using Rust and the Actix-web framework to interact with the Ethereum blockchain. It includes several endpoints to fetch information such as gas prices, latest blocks, transaction details, and more. The API is secured using JWT (JSON Web Tokens).

## Prerequisites

- [Rust](https://www.rust-lang.org/)
- [Cargo](https://doc.rust-lang.org/cargo/) (Rust package manager)
- An Ethereum node URL (e.g., [Infura](https://infura.io/))

## Installation

1. Clone the repository:

   ```bash
   git clone <repository-url>
   cd <repository-directory>
   ```

2. Install the dependencies:

   ```bash
   cargo build
   ```

## Configuration

Before running the server, set your Ethereum node URL in the `main` function of `main.rs`:

```rust
let node_url = "https://sepolia.infura.io/v3/<your-infura-project-id>".to_string();
```

Replace `<your-infura-project-id>` with your actual Infura project ID.

## Running the Server

Start the server by running:

```bash
cargo run
```

The server will start on `127.0.0.1:8080`. You can access the API endpoints via a browser or an HTTP client such as curl or Postman.

## API Endpoints

### Public Endpoints

- `GET /` - Welcome message.

Example:

```bash
curl http://127.0.0.1:8080/
```

- `POST /login` - Generates a JWT token for authentication.

Example:

```bash
curl -X POST http://127.0.0.1:8080/login
```

### Secured Endpoints

These endpoints require a Bearer token obtained from the `/login` endpoint.

- `GET /gas_price` - Returns the current gas price.

Example:

```bash
curl -H "Authorization: Bearer <your-token>" http://127.0.0.1:8080/gas_price
```

- `GET /latest_block` - Returns information about the latest block.

Example:

```bash
curl -H "Authorization: Bearer <your-token>" http://127.0.0.1:8080/latest_block
```

- `GET /transaction_details/{tx_hash}` - Returns details of a transaction by hash.

Example:

```bash
curl -H "Authorization: Bearer <your-token>" http://127.0.0.1:8080/transaction_details/<tx_hash>
```

Replace `<tx_hash>` with the actual transaction hash.

- `GET /balance/{address}` - Returns the balance of an Ethereum address.

Example:

```bash
curl -H "Authorization: Bearer <your-token>" http://127.0.0.1:8080/balance/<address>
```

Replace `<address>` with the actual Ethereum address.

- `GET /network_info` - Returns network version and peer count.

Example:

```bash
curl -H "Authorization: Bearer <your-token>" http://127.0.0.1:8080/network_info
```

- `GET /sync_status` - Returns the synchronization status of the node.

Example:

```bash
curl -H "Authorization: Bearer <your-token>" http://127.0.0.1:8080/sync_status
```

- `GET /transaction_count_in_block/{block_number}` - Returns the number of transactions in a specified block.

Example:

```bash
curl -H "Authorization: Bearer <your-token>" http://127.0.0.1:8080/transaction_count_in_block/<block_number>
```

Replace `<block_number>` with the actual block number.

## JWT Authentication

The API uses JWT for authentication. To access secured endpoints:

1. Get a token from the `/login` endpoint.

Example:

```bash
token=$(curl -s -X POST http://127.0.0.1:8080/login)
```

2. Include the token in the `Authorization` header of your request:

Example:

```bash
curl -H "Authorization: Bearer $token" http://127.0.0.1:8080/gas_price
```

## Project Structure

- `main.rs`: The main entry point of the application.
- `Cargo.toml`: The Cargo configuration file that lists dependencies and metadata for the project.

## About Actix-web

[Actix-web](https://actix.rs/) is a powerful, pragmatic, and extremely fast web framework for Rust. It is built on the Actix actor framework, which is a mature framework for building concurrent applications. Some of the key features of Actix-web include:

- **Concurrency**: Actix-web uses the Actix actor framework to provide a highly concurrent web server.
- **Speed**: Actix-web is designed for speed and efficiency, making it one of the fastest web frameworks available.
- **Type Safety**: The Rust language provides strong type safety guarantees, which are inherited by Actix-web.
- **Extensibility**: Actix-web is highly extensible, allowing you to add custom middleware, extractors, and more.

### Key Components

- **Actors**: Actix-web leverages the Actix actor model for managing state and concurrency.
- **Middleware**: Actix-web supports middleware for request pre-processing and post-processing.
- **Extractors**: Actix-web provides extractors to retrieve data from requests.
- **Routing**: Actix-web supports flexible routing with URL parameters, guards, and more.

## License

This project is licensed under the MIT License.
