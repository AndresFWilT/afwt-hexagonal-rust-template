# AFWT Hexagonal Rust Template

AFWT Hexagonal Rust Template is a foundational project template implementing the **Hexagonal Architecture** in Rust. It provides a clean structure for developing scalable, maintainable, and testable applications by separating concerns into distinct layers.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Project Structure](#project-structure)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Usage](#usage)
- [Configuration](#configuration)
- [Endpoints](#endpoints)
- [Extending the Template](#extending-the-template)
- [Contributing](#contributing)
- [License](#license)

---

## Overview

This project is a **Rust-based template** designed for building applications with Hexagonal Architecture principles. It supports:

- Clean separation of concerns.
- Dependency injection for flexibility.
- Easy integration of services and middlewares.
- Customizable environment configurations.

---

## Features

- **Hexagonal Architecture**: Divides the application into layers like domain, application, adapters, and infrastructure.
- **Environment Variable Support**: Uses `.env` for configuration management.
- **Integrated Logging**: Configurable logging with `env_logger`.
- **Error Handling**: Customizable error responses.
- **Middleware Support**: Prepares for extensible middleware integration.

---

## Project Structure

```plaintext
src/
├── adapters/               # Controller and route logic
├── application/            # Application-specific business logic
│   ├── use_cases/          # Use case implementations (e.g., sorting algorithms)
├── config/                 # Application configuration (e.g., environment variables)
├── domain/                 # Core domain entities and ports
│   ├── entities/           # Domain models
│   ├── ports/              # Interfaces for core logic
├── infrastructure/         # Infrastructure and server setup
│   ├── server/             # Web server configuration
└── main.rs                 # Application entry point
```

## Getting Started

### Prerequisites

Ensure you have the following installed:

- Rust
- Cargo
- RustUp
- RustC
- RustDoc
- Postman or cURL

### Installation

Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/andresfwilt/afwt-hexagonal-rust-template.git
cd afwt-hexagonal-rust-template
```

install dependencies and build the project

```bash
cargo build
```

### Usage

run the application:

```bash
cargo run
```

## Configuration

The template uses a `.env` file for configuration. Add your environment variables to the `.env` file at the root of the project:

```.env
SERVER_PORT=9081
SERVER_DEBUG=info
```

- Logging Levels

Modify the `SERVER_DEBUG` value in `.env` to set the logging level (**info**, **debug**, **error**, etc.).

## Endpoints

Health Check

- URL: /v1/health
- Method: GET
- Description: Returns the health status of the server.

Bubble Sort

- URL: /v1/sort/bubble
- Method: GET
- Description: Sorts an array using the Bubble Sort algorithm.

Request Body:

```json
{
  "array": [5, 2, 9, 1, 5, 6]
}
```

Response:

```json
{
  "status_code": 200,
  "description": "Array sorted successfully using bubble sort",
  "response": {
      "sorted_array": [1, 2, 5, 5, 6, 9]
  }
}
```

## Extending the Template

- Adding a New Service

1. Create a new struct in application/use_cases.
2. Implement the appropriate port interface.
3. Register the new service in the run_server function or the services module.

- Adding Middleware

Middlewares can be added to the App::wrap method in run_server. For example:

```rust
App::new()
    .wrap(your_custom_middleware)
    .configure(routes::configure_routes)
```

## Contributing

Contributions are welcome! Feel free to fork the repository and submit a pull request with your changes.

1. Fork the repository.
2. Create a new branch for your feature/bug fix.
3. Submit a pull request for review.

## License

This project is licensed under the MIT License. See the LICENSE file for details.
