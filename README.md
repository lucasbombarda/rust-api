# REST API in Rust
## Overview
This project is an implementation of a REST API in Rust, utilizing Axum for the web framework and Diesel for ORM and database interactions. It's designed as an educational endeavor to explore Rust's capabilities in web development.

## Getting Started

### Prerequisites
Ensure you have Rust installed on your system. Visit [Rust's official site](https://www.rust-lang.org/) for installation instructions.

### Setup

#### Clone the Repository
```bash
$ git clone https://github.com/lucasbombarda/rust-api
$ cd rust-api
```

#### Configure Database Connection
Copy the provided example environment file and set up the database connection details:
```bash
$ cp .env-example .env
$ nano .env
```

#### Install Diesel CLI for Migrations
Use the following command to install the [Diesel CLI](https://diesel.rs/guides/getting-started) for managing migrations with PostgreSQL support:
```bash
$ cargo install diesel_cli --no-default-features --features postgres
```
#### Run migrations
Execute the following command to run database migrations:
```bash
$ diesel migration run
```

#### Run the project
Start the server by running:

```bash
$ cargo run
```

The server will be accessible at http://127.0.0.1:8000.


### Roadmap
- [ ] Implement CRUD operations for Users.
- [ ] Implement JWT-based Login and Logout functionality.
- [ ] Introduce role-based permissions.
- [ ] Optimize database queries for efficiency.
- [ ] Implement logging and error handling mechanisms.


### Contributing
Contributions are welcome, especially from those looking to strengthen their Rust skills. Please feel free to fork, make changes, and submit pull requests.