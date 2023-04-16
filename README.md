# Backend for StudentLive Chrome Extension
This API allows for communication between the student and teacher clients when using the StudentLive Chrome extension. It is written in Rust, using a PostgreSGL database through Diesel, and is hosted with Heroku.

## Local Hosting 
To local host this API you will need to change a few lines of code in `main.rs`. In particular, comment out the line
```rust
    let port = std::env::var("PORT").expect("$PORT is not set.");
```
and the line
```rust
    .bind(("0.0.0.0", port.parse().unwrap()))?
```

Additionally, you will need to uncomment 
```rust
    .bind(("127.0.0.1", 8080))?
```

Then with Cargo installed you can run 
```bash
cargo run
```
and the API will be served locally.