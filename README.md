# axum-test

This app is just a test using the [https://docs.rs/axum/latest/axum/index.html](Axum) web application framework. Tracing is also added to the application.

Traces are being exported to jaeger and stdout.

## Usage

Startup the application

```shell
docker compose up -d
RUST_LOG=info cargo run
```

Then in another shell:

```shell
curl http://localhost:8080
```

To view the traces in jaeger, just navigate with your browser to `http://localhost:16686`

