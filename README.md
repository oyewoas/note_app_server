# Note App Server

This is a backend server for a note-taking application built with Rust.

## Features
- Create, read, update, and delete notes
- RESTful API endpoints
- Database migrations

## Getting Started


### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://www.docker.com/get-started) (for running the database)
- [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) (for migrations)


### Setup
git clone https://github.com/oyewoas/note_app_server
docker-compose up -d
make migrate-run
cargo run

1. Clone the repository:

	```sh
	git clone https://github.com/oyewoas/note_app_server
	cd note_app_server
	```

2. Start the database:

	```sh
	make docker-up
	```

3. Run database migrations:

	```sh
	make migrate-run
	```

4. (Optional) Add a new migration:

	```sh
	make migrate-add name=your_migration_name
	```

5. (Optional) Revert the last migration:

	```sh
	make migrate-revert
	```

6. Start the server (with live reload):

	```sh
	make dev
	```

	Or run without live reload:

	```sh
	cargo run
	```

7. Stop the database:

	```sh
	make docker-stop
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
