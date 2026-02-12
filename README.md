# CodeDead API

![GitHub](https://img.shields.io/badge/language-Rust-green)
![GitHub](https://img.shields.io/github/license/CodeDead/codedead_api)

## Overview

Work-in-progress: This is a simple REST API that retrieves the latest application version. It is built using Rust and
Actix-web.

## Configuration

| Variable                          | Type     | Required | Default   | Example                     | Comment                                                              |
|-----------------------------------|----------|----------|-----------|-----------------------------|----------------------------------------------------------------------|
| `SERVER_HOST`                     | `String` | `false`  | `0.0.0.0` | `0.0.0.0`                   | The hostname for the server                                          |
| `SERVER_PORT`                     | `u16`    | `false`  | `80`      | `80`                        | The port for the server                                              |
| `SERVER_WORKERS`                  | `i64`    | `false`  | `-1`      | `24`                        | The amount of threads the server can use                             |
| `SERVER_CONTEXT`                  | `String` | `true`   | N/A       | `https://localhost:132`     | The base URL context where the server runs                           |
| `MONGODB_CONNECTION_STRING`       | `String` | `true`   | N/A       | `mongodb://localhost:27017` | The MongoDB connection string                                        |
| `MONGODB_DATABASE_NAME`           | `String` | `true`   | N/A       | `codedead_staging`          | The MongoDB database name                                            |
| `MONGODB_APPLICATIONS_COLLECTION` | `String` | `true`   | N/A       | `applications`              | The MongoDB application collection                                   |
| `MAX_FETCH_LIMIT`                 | `i64`    | `false`  | `100`     | `100`                       | The maximum amount of entries that can be retrieved on a single page |
| `RUST_LOG`                        | `String` | `false`  | `info`    | `info`                      | The RUST log level                                                   |
| `RUST_BACKTRACE`                  | `bool`   | `false`  | `1`       | `1`                         | Allow an acquisition of a backtrace at runtime programmatically      |

## Usage

To run the API, you need to have Rust installed. You can install Rust using [rustup](https://rustup.rs/).

### Run the API

```bash
cargo run
```

### Build the API

```bash
cargo build --release
```

## Features

- Fetches the latest version of the application from MongoDB.
- Returns the version in JSON format.
- Supports CORS for cross-origin requests.
- Includes a health check endpoint to verify the API's status.

## Dependencies

- `actix-cors`
- `actix-web`
- `chrono`
- `dotenvy`
- `env_logger`
- `futures`
- `log`
- `mongodb`
- `serde`
- `serde_json`
- `utoipa`
- `utoipa-swagger-ui`

## About

This library is maintained by CodeDead. You can find more about us using the following links:

* [Website](https://codedead.com)
* [Bluesky](https://bsky.app/profile/codedead.com)
* [Facebook](https://facebook.com/deadlinecodedead)

Copyright © 2026 CodeDead
