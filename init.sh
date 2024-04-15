mv migrations/2024-03-18-130803_create_serial_and_index .
docker compose down
docker volume rm database_db_data
docker compose up -d
sleep 5 && DYLD_LIBRARY_PATH=/opt/homebrew/opt/libpq/lib:$DYLD_LIBRARY_PATH DATABASE_URL=postgres://postgres:password@localhost/imdb diesel setup
cargo run --release --quiet --bin insert
mv 2024-03-18-130803_create_serial_and_index migrations
DYLD_LIBRARY_PATH=/opt/homebrew/opt/libpq/lib:$DYLD_LIBRARY_PATH DATABASE_URL=postgres://postgres:password@localhost/imdb diesel migration run