SHELL := bash
.ONESHELL:
MAKEFLAGS += --no-builtin-rules

.PHONY: help build-dev run-dev run-dev-infra stop-dev

export APP_NAME := web-api-rs
NOCACHE := $(if $(NOCACHE),"--no-cache")


help: ## List all available targets with help
	@grep -E '^[0-9a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
		| awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

build-dev: ## Build develop containers
	@docker build ${NOCACHE} --pull -f ./Dockerfile -t web-api-rs:latest  .
	@docker build ${NOCACHE} --pull -f ./helper.Dockerfile -t helper:latest .

run-dev-infra:
	@docker-compose up -d postgres

dev-migration-up: ## Up all migrations to local database
	@docker-compose run --rm helper sqlx migrate run

run-dev: run-dev-infra dev-migration-up ## Run service in develop environment
	@docker-compose up -d app

stop-dev: ## Stop develop environment
	@docker-compose down && docker volume rm app
