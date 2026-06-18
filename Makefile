# ecd — Makefile
#
# Run `make` or `make help` to list all targets.

CARGO ?= cargo

.DEFAULT_GOAL := help

.PHONY: help build test check man install publish-dry-run clean fixtures

help: ## Show all available targets
	@echo "Usage: make <target>"
	@echo ""
	@echo "Targets:"
	@awk 'BEGIN {FS = ":.*## "} /^[a-zA-Z0-9_.-]+:.*## / {printf "  %-18s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

build: ## Build optimized release binary (target/release/ecd)
	$(CARGO) build --release

test: ## Run unit and integration tests
	$(CARGO) test

check: ## Check formatting and run clippy with warnings denied
	$(CARGO) fmt --check
	$(CARGO) clippy --all-targets -- -D warnings

man: ## Generate man pages into man/ via cargo xtask
	$(CARGO) xtask man

install: ## Install ecd binary to ~/.cargo/bin
	$(CARGO) install --path . --force

publish-dry-run: ## Verify crates.io package without uploading
	$(CARGO) publish --dry-run --registry crates-io --allow-dirty

clean: ## Remove build artifacts (target/)
	$(CARGO) clean

fixtures: ## Regenerate per-encoding test fixtures and manifest
	$(CARGO) test --test encodings write_encoding_fixtures -- --ignored
