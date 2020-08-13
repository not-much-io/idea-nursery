.DEFAULT_GOAL:= help

format: ## Format rust code
	@cargo fmt

format-check: ## Check formatting of rust code
	@cargo fmt -- --check

lint: ## Lint rust code
	@cargo clippy

test: ## Rust rust tests
	@cargo test

dbuild-image: ## Build the defined docker image. Usage: make dbuild-image variant=Base|VSCode|CI
	@docker build --file Dockerfile.$(variant) --tag toolbox-$(variant)-image .

dcreate-container: ## Create the defined docker container. Usage: make dcreate-container variant=Base|VSCode|CI
	@docker create \
	--name toolbox-$(variant)-container \
	toolbox-$(variant)-image

dstart-container: ## Start the defined docker container. Usage: make dstart-container variant=Base|VSCode|CI
	@docker start toolbox-$(variant)-container -a

dclean: ## Remove everything associated with the defined dockerfile. Usage: make dclean variant=Base|VSCode|CI
	@docker stop toolbox-$(variant)-container
	@docker rm toolbox-$(variant)-container
	@docker rmi -f toolbox-$(variant)-image

dinit: ## Initialize project, VSCode setup done by VSCode
	@make dbuild-image variant=base

dcreate-context:  ## TODO
	@docker context create ws --docker "host=ssh://nmio@85.253.142.91"

# Source: https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
help: ## Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
	| awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
