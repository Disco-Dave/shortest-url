#!/usr/bin/env bash

set -e

DB_USER=${SHORTEST_URL_DATABASE_MIGRATION_USERNAME:=postgres}
DB_PASSWORD="${SHORTEST_URL_DATABASE_MIGRATION_PASSWORD:=password}"
DB_NAME="${SHORTEST_URL_DATABASE_DATABASE_NAME:=shortest_url}"
DB_PORT="${SHORTEST_URL_DATABASE_PORT:=5432}"

DB_APP_PASSWORD="${SHORTEST_URL_DATABASE_APP_PASSWORD:=password}"

container_name="shortest_url_test_pg_database"
container_volume="shortest_url_test_pg_data"

if [ ! "$(docker ps -aq -f name=$container_name)" ]; then
    if ! docker volume ls | grep -q $container_name ; then
        docker volume create $container_volume
    fi

    docker run -d \
        --name $container_name \
        -e POSTGRES_USER=${DB_USER} \
        -e POSTGRES_PASSWORD=${DB_PASSWORD} \
        -e POSTGRES_DB=${DB_NAME} \
        -p "${DB_PORT}":5432 \
        -v "${container_volume}":/var/lib/postgresql/data \
        -d postgres \
        postgres -N 1000
        # ^ Increased maximum number of connections for testing purposes
elif [ "$(docker ps -aq -f status=exited -f name=$container_name)" ]; then
    docker start $container_name
fi

# Keep pinging Postgres until it's ready to accept commands
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q' 2> /dev/null; do
    >&2 echo "Postgres is still unavailable - sleeping"
    sleep 1
done

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create

if ! psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "${DB_NAME}" -tAc "SELECT 1 from pg_roles WHERE rolname='app';" | grep -q 1 ; then
    >&2 echo "Adding ${DB_APP_USER} user"
    psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "${DB_NAME}" -c "CREATE USER app WITH PASSWORD '${DB_APP_PASSWORD}';"
fi

>&2 echo "Postgres is up and running on port ${DB_PORT}!"

sqlx migrate run
>&2 echo "Postgres is migrated and ready to go!"
