mv migrations/2024-03-18-130803_serial_index_cleanup .
docker compose down
docker volume rm database_db_data
docker compose up -d
sleep 5
DYLD_LIBRARY_PATH=/opt/homebrew/opt/libpq/lib:$DYLD_LIBRARY_PATH DATABASE_URL=postgres://postgres:password@localhost/imdb diesel setup
cargo run --release --quiet --bin insert
mv 2024-03-18-130803_serial_index_cleanup migrations
DYLD_LIBRARY_PATH=/opt/homebrew/opt/libpq/lib:$DYLD_LIBRARY_PATH DATABASE_URL=postgres://postgres:password@localhost/imdb diesel migration run
echo "VACUUM FULL aka_attributes;
VACUUM FULL aka_types;
VACUUM FULL akas;
VACUUM FULL crew;
VACUUM FULL genres;
VACUUM FULL names;
VACUUM FULL principals;
VACUUM FULL titles;" | psql postgres://postgres:password@localhost/imdb