# cortex 🧠🗄️

Easily create database schemas with a unified API for any supported database. Introducing `cortex`, a Rust library designed to simplify and unify database schema creation across multiple database platforms.

![Cortex Logo](./logo.png) 

## Why cortex?

- **Unified API**: No need to learn different schema creation languages or tools for different databases.
- **Flexibility**: Easily switch between databases or support multiple databases in a single product.
- **Extensibility**: If a database isn't already supported, extend the library by implementing the required traits.
- **Product Evolution**: As your product grows and scales, your database needs might change. Whether you're migrating, supporting multiple deployments, or optimizing for specific use-cases, `cortex` makes it a breeze.

## Features 🚀

- **Supported Databases**: PostgreSQL, SQLite(TODO), MySQL(TODO), and more. Easily expand to others.
- **Schema Creation**: Define once, use anywhere. No more database-specific scripts.
- **Validation**: Validate schemas against the selected database before application.

## Getting Started 🛠️

### Installation

Add `cortex` to your `Cargo.toml`:

```toml
[dependencies]
cortex = "0.1.0"
```

### Basic Usage

Define and create a schema in SQLite:

```rust
```

## Extend `cortex` to Other Databases 🌍

Simply implement the `Database` and `SchemaEngine` traits for your database:

```rust
```

## Use Cases 💼

- **Product Scalability**: Whether starting with SQLite for MVP or scaling up with PostgreSQL for production, `cortex` adapts with your needs.
- **Multi-tenant Systems**: Support multiple databases for different clients or deployments with a unified schema interface.
- **Rapid Development**: Prototype and switch databases without rewriting schema creation logic. Perfect for startups and agile development!

## Contribution 🤝

Got ideas or improvements?

- Open an issue for bugs, enhancements, or feature requests.
- Fork, improve, and submit a pull request.

## License 📜

`cortex` is licensed under the MIT license. Dive into the [LICENSE](./LICENSE) file for details.

## TODO

- cargo install cargo-llvm-cov
- cargo install cargo-nextest
- export RUSTFLAGS="-Cinstrument-coverage"
- grcov . --binary-path ./target/llvm-cov-target/debug/deps -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/html
