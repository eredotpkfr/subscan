SHELL=/bin/bash

all: install-pre-commit-mac \
	install-pre-commit-linux \
	install-nightly-toolchain \
	install-pre-commit-hooks \
	pre-commit-update-hooks \
	rustfmt-check \
	rustfmt \
	fix \
	clean \
	build \
	doc \
	doc-rs \
	clippy \
	deny \
	test

.PHONY: all

install-pre-commit-mac:
	@brew install pre-commit
install-pre-commit-linux:
	@sudo apt install pre-commit
install-pre-commit-hooks:
	@pre-commit install --install-hooks
	@pre-commit install --hook-type commit-msg --install-hooks
install-nightly-toolchain:
	@rustup toolchain install nightly
install-cargo-clippy:
	@rustup component add clippy
install-cargo-deny:
	@cargo install cargo-deny
pre-commit-update-hooks:
	@pre-commit autoupdate
rustfmt-check:
	@cargo +nightly fmt --all -- --check
rustfmt: fix
	@cargo +nightly fmt --all
test:
	@cargo test
doc:
	@cargo doc
doc-rs:
	@cargo +nightly docs-rs
clean:
	@cargo clean
fix:
	@cargo fix --allow-dirty --allow-staged
build:
	@cargo build
clippy:
	@cargo clippy --all-targets --all-features
deny:
	@cargo deny --all-features --log-level error check
