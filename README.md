

# Entry-Server


[中文](README_CN.md)

Entry-Server is a robust, fully-equipped backend project in Rust that's ready for use. Whether you're an independent developer or part of a small startup team looking to streamline server-side development, this project could be just what you need.

## **Key Features**

- High performance and strong security measures
- Lower machine resource usage
- A well-thought-out project setup, complete with unified response handling, and efficient database and cache usage, among other things
- A variety of business systems, including a user system, OAuth login/registration, and a license system
- Designed for easy extension, allowing you to concentrate on developing your unique business logic
- Simple deployment, with Docker Compose offering a one-click startup solution

## **API Documents**

[see here](tests/README.md)

| Method | Endpoint                     |
---------|-------------------------------
| GET    | /api/v1/license/:license     |
| POST   | /api/v1/license/:license     |
| GET    | /api/v1/user                 |
| POST   | /api/v1/user                 |
| POST   | /api/v1/user/login           |
| PATCH  | /api/v1/user/password        |
| GET    | /api/v1/user/oauth/:provider |
| POST   | /api/v1/user/oauth/:provider |
| GET    | /api/v1/sync                 |
| POST   | /api/v1/sync                 |
| DELETE | /api/v1/sync                 |

## **Technology Stack**

- Development Language: [Rust](https://www.rust-lang.org/)

- Web Framework: [Axum](https://github.com/tokio-rs/axum)

- Database: PostgreSQL with [SeaORM](https://github.com/SeaQL/sea-orm)

- Cache: Redis with [Fred](https://github.com/aembke/fred.rs)


## How to *Run*

After cloning the project

1. Prepare environment variables:

    The project uses `dotenv` to initialize environment variables, which means you can create a `.env` file in the project root directory to define variables.

    Here are some necessary environment variables, Do not use the following information in a production environment:

    ```bash

    DATABASE_URL=postgres://entry_server:123456@127.0.0.1:5432/entry
    POSTGRES_DB=entry
    POSTGRES_USER=entry_server
    POSTGRES_PASSWORD=123456

    REDIS_URL=redis://127.0.0.1:6379

    ENTRY_SERVER_ADDR=127.0.0.1:3000

    ```

    There are also some unnecessary environment variables that you may need when you need some other features.

2. Prepare the database:
    
    There is an initialization script in the `sql` directory, and you need to run it first.
    
3. Start service dependencies:
    
    ```bash
    docker compose -f docker-compose.yaml up -d
    
    ```
    
4. Run
    
    ```bash
    cargo run
    
    ```

## How to *Deploy*

todo!()