🚀 Newsletter Subscription Service – A Rust-based web application for managing newsletter subscriptions, built using Actix-Web, SQLx, and PostgreSQL.

Features
✅ User subscription with email & name
✅ PostgreSQL database integration with SQLx
✅ Actix-Web for high-performance API handling
✅ Secure form validation & error handling
✅ Docker support for easy deployment

Tech Stack
Rust 🦀 (Actix-Web, SQLx)
PostgreSQL 🗄️ (Database)
Docker 🐳 (Containerization)


## How to build

Launch a (migrated) Postgres database via Docker:

```bash
./scripts/init_db.sh
```

Launch a Redis instance via Docker:

```bash
./scripts/init_redis.sh
```

Launch `cargo`:

```bash
cargo build
```

You can now try with opening a browser on http://127.0.0.1:8000/login after
having launch the web server with `cargo run`.

There is a default `admin` account with password
`everythinghastostartsomewhere`. The available entrypoints are listed in
[src/startup.rs](https://github.com/LukeMathWalker/zero-to-production/blob/6bd30650cb8670a146819a342ccefd3d73ed5085/src/startup.rs#L92)

## How to test

Launch a (migrated) Postgres database via Docker:

```bash
./scripts/init_db.sh
```

Launch a Redis instance via Docker:

```bash
./scripts/init_redis.sh
```

Launch `cargo`:

```bash
cargo test 
```
