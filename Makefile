DOCKER_IP := $(shell ipconfig getifaddr en0)

run:
	echo "posgres://postgres@$(DOCKER_IP)/rust_api"