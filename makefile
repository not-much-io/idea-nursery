.DEFAULT_GOAL:= help

build: ## Build everything
	@cargo build
	@cargo build --release

format: ## Format rust code
	@cargo fmt

format-check: ## Check formatting of rust code
	@cargo fmt -- --check

lint: ## Lint rust code
	@cargo clippy

test: ## Run rust tests
	@cargo test

ci: ## Run CI rust quality check process
	make lint && make format-check && make test

dbuild-image: ## Build the defined docker image. Usage: make dbuild-image variant=Base|VSCode|CI
	@docker build --file Dockerfile.$(variant) --tag idea-nursery-$(variant)-image .

dcreate-container: ## Create the defined docker container. Usage: make dcreate-container variant=Base|VSCode|CI
	@docker create \
		--name idea-nursery-$(variant)-container \
		idea-nursery-$(variant)-image

dstart-container: ## Start the defined docker container. Usage: make dstart-container variant=Base|VSCode|CI
	@docker start idea-nursery-$(variant)-container -a

dclean: ## Remove everything associated with the defined dockerfile. Usage: make dclean variant=Base|VSCode|CI
	@docker stop idea-nursery-$(variant)-container
	@docker rm idea-nursery-$(variant)-container
	@docker rmi -f idea-nursery-$(variant)-image

dinit: ## Initialize project, VSCode setup done by VSCode
	@make dbuild-image variant=base

# Source: https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
help: ## Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
	| awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
