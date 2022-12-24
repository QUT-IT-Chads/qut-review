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

# Whether the api returns dummy data or not
DEMO=false 
DATABASE_URL=postgres://username:password@localhost/qut_review
```

## Setting up your local database
**Todo**

## Technology Used

The project is written in Rust using two main frameworks:

- [Rocket.rs](https://rocket.rs/) - A web framework
- [Diesel.rs](https://diesel.rs/) - An ORM and Query Builder

This project attempts to follow clean architecture practices to manage the project's architecture and file structure.
