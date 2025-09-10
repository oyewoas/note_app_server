# Makefile

# Run the Rust app with live reload
dev:
	cargo watch -q -c -w src/ -x run

# Start docker services
docker-up:
	docker compose up -d

# Stop docker services
docker-stop:
	docker compose stop

# Add a new migration (usage: make migrate-add name=your_migration_name)
migrate-add:
	sqlx migrate add -r $(name)

# Run all migrations
migrate-run:
	sqlx migrate run

# Revert last migration
migrate-revert:
	sqlx migrate revert
