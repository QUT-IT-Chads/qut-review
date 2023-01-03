# Back-end Development

## Required Tools

- [Rust](https://www.rust-lang.org/)
- [Cargo](https://doc.rust-lang.org/stable/cargo/)

## Setup

1. From the root directory (`/`), navigate to `api/` with: `cd api/`.
2. Create a `.env` file and copy the values found under **Environment**
3. Run the project using: `cargo run`.

## Environment

Here are all the required environment variables.

```
# .env

JWT_SECRET=localhost_secret
DATABASE_URL=postgres://username:password@localhost/qut_review
```

## Setting up your local database

1. Install PostgreSQL
2. Install the [Diesel CLI](https://github.com/diesel-rs/diesel/tree/master/diesel_cli) tool
3. Create the database: `CREATE DATABASE qut_review`
4. Navigate into `infrastructure/` and run the latest migration: `diesel migration run`

## Technology Used

The project is written in Rust using two main frameworks:

- [Rocket.rs](https://rocket.rs/) - A web framework
- [Diesel.rs](https://diesel.rs/) - An ORM and Query Builder

This project attempts to follow clean architecture practices to manage the project's architecture and file structure.

## Documentation
The API documenation can be found at `/api/rapi-doc`
