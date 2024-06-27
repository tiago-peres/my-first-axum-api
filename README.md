# My First Axum API

This is a simple API built with Rust using the Axum framework, connected to a PostgreSQL database.

## Prerequisites

Before you start, ensure you have Docker, Docker Compose and Rust installed on your system.

## Getting Started

### 1. Clone the Repository

First, clone the repository to your local machine:

```sh
git clone git@github.com:tiago-peres/my-first-axum-api.git
cd my-first-axum-api
```

### 2. Build and Start All Services

Now, build and start all services using Docker Compose:

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

#### Get a Specific User

```sh
curl http://localhost:8080/users/1
````

#### Update a User

```sh
curl -X PUT http://localhost:8080/users/1 -H "Content-Type: application/json" -d '{"name": "Jane Doe"}'
```

#### Delete a User

```sh
curl -X DELETE http://localhost:8080/users/1
```

## 4. Making Changes to Database Queries

If you need to make changes to your database queries or add new ones, follow these steps:

1. **Start the Database Service**

   Use Docker Compose to start the PostgreSQL database service:

   ```sh
   docker-compose up -d db
   ```

2. **Generate Cornucopia File**

   Open Git Bash and run the Cornucopia CLI against the database to generate the new query files:

   ```sh
   cornucopia live postgres://postgres:password@localhost:5432/mydb
   ```

3. **Shut Down the Database Service**

   Shut down the database service:

   ```sh
   docker-compose down
   ```

## Project Structure

```
my-first-rust-api/
├── .gitignore
├── Cargo.toml
├── docker-compose.yml
├── Dockerfile
├── init.sql
├── README.md
└── queries/
    └── users.sql
└── src/
    └── main.rs
    └── cornucopia.rs
```