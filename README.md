# Rust REST API POC

A proof-of-concept REST API built with Rust, implementing basic CRUD operations for User management.

## Features

- RESTful API endpoints for User management
- CRUD operations (Create, Read, Update, Delete)
- InMemory persistance integration
- Error handling

## Tech Stack

- [Actix-web](https://actix.rs/) - Web framework
- [Serde](https://serde.rs/) - Serialization framework
- [env_logger](https://docs.rs/env_logger/latest/env_logger/) - Logging implementation

## Getting Started

### Prerequisites

- Rust (latest stable version)
- SQLite
- Git

### Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/rust-api-poc.git
cd rust-api-poc
```

2. Set up the environment variables:
```bash
cp .env.example .env
# Edit .env with your configurations
```

3. Initialize the database:
```bash
sqlx database create
sqlx migrate run
```

4. Build and run the project:
```bash
cargo build
cargo run
```

The server will start at `http://localhost:8080`

## API Endpoints

### Users

- `POST /api/users` - Create a new user
- `GET /api/users` - List all users
- `GET /api/users/{id}` - Get a specific user
- `PUT /api/users/{id}` - Update a user
- `DELETE /api/users/{id}` - Delete a user


## Request/Response Examples

### Create User

```bash
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{
    "username": "johndoe",
    "email": "john@example.com",
  }'
```

Response:
```json
{
  "id": "951ca0f0-1cbb-43cd-b62e-74cb036af07a",
  "name": "John Doe",
  "email": "doe@example.cl",
  "created_at": "2025-02-01T14:51:11.734290069Z",
  "updated_at": "2025-02-01T14:51:11.734290069Z"
}
```

## Development

### Running Tests

```bash
cargo test
```

## Error Handling (pending)

The API uses standard HTTP status codes:

- 200: Success
- 201: Created
- 400: Bad Request
- 401: Unauthorized
- 404: Not Found
- 500: Internal Server Error


## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [Rust Book](https://doc.rust-lang.org/book/)
- [Actix Documentation](https://actix.rs/docs/)
- [SQLx Documentation](https://docs.rs/sqlx/latest/sqlx/)