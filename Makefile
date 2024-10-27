# Python tasks
install-python:
	pip install --upgrade pip && pip install -r requirements.txt

test-python:
	python -m pytest -vv --cov=main --cov=mylib test_*.py

format-python:
	black main.py mylib/*.py

lint-python:
	ruff check main.py mylib/*.py

# Rust tasks
rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version
	cargo --version
	rustfmt --version
	rustup --version
	clippy-driver --version

format-rust:
	cd sqlite && cargo fmt --quiet

lint-rust:
	cd sqlite && cargo clippy --quiet

test-rust:
	cd sqlite && cargo test --quiet

run-rust:
	cd sqlite && cargo run

release-rust:
	cd sqlite && cargo build --release

# All-in-one commands
install: install-python

format: format-python format-rust
lint: lint-python lint-rust
test: test-python test-rust

deploy:
	# Add deployment steps here

all: install format lint test deploy
