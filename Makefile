DOCKER_IP := $(shell ipconfig getifaddr en0)

run:
	DATABASE_URL="posgres://postgres@$(DOCKER_IP)/rust_api" cargo run

build:
	DATABASE_URL="posgres://postgres@$(DOCKER_IP)/rust_api" cargo build --release

build-local:
	alias rust-musl-builder='docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder'
	rust-musl-builder cargo build --release

docker:
	docker build -t alextanhongpin/rust-api .