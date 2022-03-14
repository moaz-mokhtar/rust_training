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

Total APIs to be developed as below:

- register
- login
- user
- refresh
- logout
- forgot
- reset
- two-factor
- google-auth

### `register` endpoint

- Request:

    ```
    GET http://127.0.0.1:8000/register
    ```

    ```json
        {
            "first_name": "...",
            "last_name": "...",
            "username": "...",
            "email": "...",
            "password": "...",
            "confirm_password": "..."
        }
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

    ```json
        {
            "username": "...",
            "password": "..."
        }
    ```

- Response:

    ```json
        {
            "message": "User authenticated",
            "data": true
        }
    ```

<br>

<br>

## References

Below are little resources: <br>
<https://serde.rs/derive.html> <br>
<https://stackoverflow.com/questions/62269278/how-can-i-make-protected-routes-in-actix-web> <br>
<https://actix.rs/docs/middleware/> <br>
<https://actix.rs/> <br>

<br>

---

Thanks <br>
> Developed by Moaz bin Mohamed Mokhtar
