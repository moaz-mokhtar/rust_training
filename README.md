# Rust Training

This is the code to training materials.

Technologies are as below:

- Rust programming language #rust_lang
- Actix web framework
- MySql for database
- Diesel ORM
- REST API

## Development steps

- Install Rust follow link: <https://www.rust-lang.org/tools/install>
- Install Docker follow link: <https://docs.docker.com/get-docker/>
<!-- TODO -->
- Install MySql-server docker image, link: <https://hub.docker.com/r/yugabytedb/yugabyte>

    ```shell
    docker pull yugabytedb/yugabyte
    ```

- Create and run Yugabyte container named `yugabyte`

    ```
    docker run -d --name yugabyte -p7000:7000 -p9000:9000 -p5433:5433 -p9042:9042 -v ~/yb_data:/home/yugabyte/yb_data yugabytedb/yugabyte:latest bin/yugabyted start --base_dir=/home/yugabyte/yb_data --daemon=false
    ```

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

### `health` endpoint

- Request:

    ```
    GET http://127.0.0.1:8080/health
    ```

- Response:

    ```
    Healthy
    ```

### `users` endpoint

- Request:

    ```
    GET http://127.0.0.1:8080/users
    ```

- Response:

    ```json
        [
            {
                "id": "3",
                "username": "said",
                "password": "pass"
            },
            {
                "id": "2",
                "username": "baher",
                "password": "pass"
            },
            {
                "id": "1",
                "username": "ahmed",
                "password": "pass"
            }
        ]
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

---
Thanks
> Developed by Moaz bin Mohamed Mokhtar
