DOCKER_IP := $(shell ipconfig getifaddr en0)

run:
	DATABASE_URL="posgres://postgres@$(DOCKER_IP)/rust_api" cargo run