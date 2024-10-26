rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format:
	cargo fmt --quiet

install:
	# Install if needed
	#@echo "Updating rust toolchain"
	#rustup update stable
	#rustup default stable 

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run

release:
	cargo build --release
	

# 'all' target to format, lint, test, and run
all: 
	@echo "Running the 'all' target"
	# Call the make targets for format, lint, test, and run
	make format
	make lint
	make test
	make run


# Python setup and scripts
install_py:
	pip install --upgrade pip && \
		pip install -r requirements.txt

test_py:
	python -m pytest -vv --cov=main --cov=lib test_*.py

format_py:	
	black *.py lib/*.py 

lint_py:
	# Run ruff for linting with line length configuration
	ruff check --line-length 100 *.py lib/*.py

container-lint_py:
	# Lint Dockerfile using hadolint container
	docker run --rm -i hadolint/hadolint < Dockerfile

refactor_py: format_py lint_py

deploy_py:
	# Placeholder for deployment steps
	@echo "Deployment steps go here"

all_py: install_py format_py lint_py test_py deploy_py

