SHELL=/bin/bash

all: install-pre-commit-mac \
	install-pre-commit-linux \
	pre-commit-update-hooks \
	install-pre-commit-hooks \
	rustfmt-check \
	rustfmt \
	fix \
	clean \
	build \
	doc \
	clippy \
	test

.PHONY: all

install-pre-commit-mac:
	@brew install pre-commit
install-pre-commit-linux:
	@sudo apt install pre-commit
install-pre-commit-hooks:
	@pre-commit install --install-hooks
	@pre-commit install --hook-type commit-msg --install-hooks
pre-commit-update-hooks:
	@pre-commit autoupdate
rustfmt-check:
	@cargo fmt --all -- --check
rustfmt:
	@cargo fmt --all
test:
	@cargo test
doc:
	@cargo doc
clean:
	@cargo clean
fix:
	@cargo fix --allow-dirty --allow-staged
build:
	@cargo build
clippy:
	@cargo clippy --all-targets --all-features