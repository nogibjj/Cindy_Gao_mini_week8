# Makefile for managing both Rust and Python tasks

# Rust tasks
.PHONY: rust-version format install lint test run release all_rust

rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version               # rust compiler
	cargo --version               # rust package manager
	rustfmt --version             # rust code formatter
	rustup --version              # rust toolchain manager
	clippy-driver --version       # rust linter

format:
	cargo fmt --quiet

install:
	@echo "Installing/updating Rust toolchain"
	rustup update stable
	rustup default stable 

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run

release:
	cargo build --release

# 'all_rust' target to format, lint, test, and run
all_rust: 
	@echo "Running all Rust targets (format, lint, test, run)"
	make format
	make lint
	make test
	make run


# Python tasks
.PHONY: install_py test_py format_py lint_py container-lint_py refactor_py deploy_py all_py

install_py:
	pip install --upgrade pip && \
		pip install -r requirements.txt

test_py:
	python -m pytest -vv --cov=lib --cov-report=term-missing lib/test_main.py

format_py:
	black lib/*.py

lint_py:
	ruff check --line-length 100 lib/*.py

container-lint_py:
	docker run --rm -i hadolint/hadolint < Dockerfile

refactor_py: format_py lint_py

deploy_py:
	@echo "Python deployment steps would go here"

# 'all_py' target to install, format, lint, test, and deploy Python
all_py: install_py format_py lint_py test_py deploy_py


# Combined tasks for both Python and Rust
.PHONY: all

all: all_rust all_py
	@echo "All Rust and Python tasks have been completed"


