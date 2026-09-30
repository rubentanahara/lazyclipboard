.PHONY: help install dev build lint fmt types test check clean

help:
	@grep -E '^[a-z-]+:.*##' $(MAKEFILE_LIST) | awk -F':.*## ' '{printf "%-10s %s\n", $$1, $$2}'

install: ## Install JS dependencies from the lockfile
	pnpm install --frozen-lockfile

dev: ## Run the Tauri app with hot reload
	pnpm dev

build: ## Build the Tauri app without bundling
	pnpm build

lint: ## Clippy with warnings as errors, rustfmt check, JS lint scripts
	cargo clippy --workspace --all-targets -- -D warnings
	cargo fmt --all --check
	pnpm -r --if-present run lint

fmt: ## Format Rust code
	cargo fmt --all

types: ## Type-check Rust and TypeScript
	cargo check --workspace --all-targets
	pnpm -r --if-present run types

test: ## Run Rust and JS unit tests
	cargo test --workspace
	pnpm -r --if-present run test

check: lint types test ## Lint, types and tests together

clean: ## Remove build output
	cargo clean
	rm -rf apps/ui/dist .turbo
