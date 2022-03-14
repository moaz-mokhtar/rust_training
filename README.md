# Rust Training

This is the code to training materials.

Technologies are as below:

- Rust programming language #rust_lang
- Actix web framework
- Postgres for database
- Diesel ORM
- REST API

## Development steps

- Install Rust follow link: <https://www.rust-lang.org/tools/install>
- Install Docker follow link: <https://docs.docker.com/get-docker/>
<!-- TODO -->
- Install postgres in your device or inside docker image:<br>
 <https://www.postgresql.org/download/> <br>
 <https://hub.docker.com/_/postgres>

- install Diesel CLI, link: <https://crates.io/crates/diesel_cli>

    ```
    cargo install diesel_cli --no-default-features --features "postgres"
    ```

## Run application

After downloading this repo:

- install database

    ```
    diesel setup
    ```

- run application

    ```
    cargo run
    ```

## REST API queries using Postman

### `register` endpoint

- Request:

    ```
    GET http://127.0.0.1:8000/register
    ```

- Response:

    ```
    TODO
    ```

### `login` endpoint

- Request:

    ```
    GET http://127.0.0.1:8000/login
    ```

- Response:

    ```json
        TODO
    ```

## GraphQL API queries using GraphiQL

Visit <http://127.0.0.1:8080/graphiql> after run the application.

### `api_version` endpoint

- Request

    ```json
    query version{
        apiVersion
    }
    ```

- Response:

    ```json
    {
        "data": {
            "apiVersion": "1.0"
        }
    }

    ```

### `users` endpoint

- Request

    ```json
    query users{
        users{
            id
            username
        }
    }
    ```

- Response:

    ```json
    {
        "data": {
            "users": [
            {
                "id": "3",
                "username": "said"
            },
            {
                "id": "2",
                "username": "baher"
            },
            {
                "id": "1",
                "username": "ahmed"
            }
            ]
        }
    }
    ```

<br>

<br>

## References

<https://serde.rs/derive.html> <br>
<https://stackoverflow.com/questions/62269278/how-can-i-make-protected-routes-in-actix-web> <br>
<https://actix.rs/docs/middleware/>
<br>

<br>
---
Thanks
> Developed by Moaz bin Mohamed Mokhtar
