# My First Rust API

This is a simple API built with Rust using the Axum framework, connected to a PostgreSQL database.

## Prerequisites

Before you start, ensure you have Docker and Docker Compose installed on your system.

## Getting Started

### 1. Clone the Repository

First, clone the repository to your local machine:

```sh
git clone git@github.com:tiago-peres/my-first-rust-api.git
cd my-first-rust-api
```

### 2. Start the Services with Docker Compose

Use Docker Compose to build and start the services:

```sh
docker-compose up --build
```

This will start both the PostgreSQL database and the Rust API server.

### 3. Test the API Endpoints

Once the services are up and running, you can test the API endpoints using `curl` or any API client like Postman.

#### Add a User

```sh
curl -X POST http://localhost:8080/users -H "Content-Type: application/json" -d '{"name": "John Doe"}'
```

#### Get All Users

```sh
curl http://localhost:8080/users
```

## Project Structure

```
my-first-rust-api/
├── Cargo.toml
├── Dockerfile
├── docker-compose.yml
├── init.sql
└── src/
    └── main.rs
```

