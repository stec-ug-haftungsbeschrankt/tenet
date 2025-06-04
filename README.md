# tenet

## Overview

Tenet is a library for managing tenants, applications, and users in SaaS environments, written in Rust. The library provides a robust foundation for developing multi-tenant applications.

## Key Features

- **Multi-tenant Architecture**: Full support for SaaS applications with isolated tenants
- **User Management**: Secure password storage with Argon2 encryption
- **Role Management**: Flexible permission levels (Administrator, User)
- **Application Configuration**: Various storage options for application data
- **Data Storage**: PostgreSQL database as the primary data store

## URL Schema

Applications are accessed using the following URL schema:

```
https://apps.stecug.de/<tenant-id>/<application-id>
```

## Storage Options

Tenet supports various storage methods for application configurations:

- JSON files
- SQLite databases
- PostgreSQL databases
- PostgreSQL schemas
- PostgreSQL table prefixes

## Role Types

Users can have the following roles:

- **Administrator**: Comprehensive permissions
- **User**: Limited standard permissions

## Application Types

Currently, the following application type is supported:

- **Shop**: E-commerce application

## Encryption Methods

The following encryption method is used for passwords:

- **Argon2**: Modern, secure password hashing function

## Example

```rust
use tenet::{Tenet, User, encryption_modes::EncryptionModes};

// Initialize Tenet with a database connection
let tenet = Tenet::new("postgres://user:password@localhost/mydb".to_string());

// Create a new tenant
let tenant = tenet.create_tenant("My Company".to_string()).unwrap();

// Create a user and add it to the tenant
let user = User::new(
    "admin@example.com".to_string(),
    "Admin User".to_string(),
    "secure_password".to_string(),
    EncryptionModes::Argon2,
    "admin@example.com".to_string(),
    true,
    tenant.id
);
let created_user = tenant.add_user(&user).unwrap();
```

## Testing

We use unit/integration tests. In order to run them you need `docker` running and have `cargo-nextest` installed. You can do this with:

```bash
> cargo install cargo-nextest --locked
```

To run the tests, use the following command:

```bash
> cargo nextest run
```

## License

GPL-3.0-or-later