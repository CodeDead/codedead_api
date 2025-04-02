# CodeDead API

![GitHub](https://img.shields.io/badge/language-Rust-green)
![GitHub](https://img.shields.io/github/license/CodeDead/codedead_api)

## Overview

Work-in-progress: This is a simple REST API that retrieves the latest application version. It is built using Rust and
Actix-web.

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

## About

This library is maintained by CodeDead. You can find more about us using the following links:

* [Website](https://codedead.com)
* [Bluesky](https://bsky.app/profile/codedead.com)
* [Facebook](https://facebook.com/deadlinecodedead)

Copyright © 2025 CodeDead
