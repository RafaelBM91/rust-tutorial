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

mongodb:
	docker run \
	-d \
	--rm \
	--name=${MONGO_NAME} \
	-e MONGO_INITDB_ROOT_USERNAME=${MONGO_USER} \
	-e MONGO_INITDB_ROOT_PASSWORD=${MONGO_PASS} \
	-v ${PWD}/mongo:/data/db \
	-p ${MONGO_PORT}:27017 \
	--expose=27017 \
	mongo:latest
