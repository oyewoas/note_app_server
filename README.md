# Note App Server

This is a backend server for a note-taking application built with Rust.

## Features
- Create, read, update, and delete notes
- RESTful API endpoints
- Database migrations

## Getting Started

### Prerequisites
- Rust (https://www.rust-lang.org/tools/install)
- Docker (for running the database)

### Setup
1. Clone the repository:
   ```sh
git clone <repo-url>
cd note_app_server
```
2. Start the database using Docker Compose:
   ```sh
docker-compose up -d
```
3. Run database migrations:
   ```sh
make migrate
```
4. Build and run the server:
   ```sh
cargo run
```

## API Endpoints
- `GET /notes` - List all notes
- `POST /notes` - Create a new note
- `GET /notes/{id}` - Get a note by ID
- `PUT /notes/{id}` - Update a note
- `DELETE /notes/{id}` - Delete a note

## Project Structure
- `src/` - Source code
- `migrations/` - SQL migration files
- `Cargo.toml` - Rust dependencies and metadata
- `docker-compose.yml` - Database configuration
- `Makefile` - Common tasks

## License
MIT
