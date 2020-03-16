include .env
export  $(shell sed 's/=.*//' .env)

build:
	docker build \
	-t "rust-crud:latest" \
	.

init:
	docker run \
	--rm \
	-e USER=${USER} \
	-v "${PWD}":${APP_PATH_INTERNAL} \
	rust-crud:latest \
	cargo init

cargo:
	docker run \
	--rm \
	-e USER=${USER} \
	-v ${PWD}:${APP_PATH_INTERNAL} \
	rust-crud:latest \
	cargo install --path ${APP_PATH_INTERNAL}

compile:
	docker run \
	--rm \
	-e USER=${USER} \
	-v ${PWD}:${APP_PATH_INTERNAL} \
	rust-crud:latest \
	cargo build --release

exec:
	ROCKET_PORT=${ROCKET_PORT} target/release/myapp

postgre:
	docker run \
	-d \
	--rm \
	--name=${POSTGRES_NAME} \
	-e POSTGRES_USER=${POSTGRES_USER} \
	-e POSTGRES_PASSWORD=${POSTGRES_PASSWORD} \
	-e POSTGRES_DB=${POSTGRES_DB} \
	-v ${PWD}/postgres:/var/lib/postgresql/data \
	-v ${PWD}/src/migration:/var/migration \
	-p ${POSTGRES_PORT}:5432 \
	--expose=5432 \
	postgres:latest

migrate:
	docker exec \
		${POSTGRES_NAME} \
		psql -f \
			${MIGRATION_FILE_CREATE} \
			postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost:5432/${POSTGRES_DB} 
