SHELL=/bin/bash

.PHONY: all

all: install-pre-commit-mac \
	install-pre-commit-linux \
	install-pre-commit-hooks \
	install-cargo-tools \
	update-pre-commit-hooks \
	install-nightly-toolchain \
	install-cargo-clippy \
	install-cargo-deny \
	install-cargo-udeps \
	install-cargo-llvm-cov \
	install-cargo-dist \
	install-git-cliff \
	install-cargo-nextest \
	clean \
	check \
	fix \
	doc \
	doc-rs \
	doc-test \
	nextest \
	test \
	build \
	rustfmt-check \
	rustfmt \
	clippy \
	deny

install-pre-commit-mac:
	@brew install pre-commit
install-pre-commit-linux:
	@sudo apt install pre-commit
install-pre-commit-hooks:
	@pre-commit install --install-hooks
	@pre-commit install --hook-type commit-msg --install-hooks

update-pre-commit-hooks:
	@pre-commit autoupdate

install-cargo-tools: install-nightly-toolchain \
	install-cargo-clippy \
	install-cargo-doc-rs \
	install-cargo-deny \
	install-cargo-udeps \
	install-cargo-llvm-cov \
	install-cargo-dist \
	install-git-cliff \
	install-cargo-nextest

install-nightly-toolchain:
	@rustup toolchain install nightly
install-cargo-clippy:
	@rustup component add clippy
install-cargo-doc-rs:
	@cargo install cargo-docs-rs
install-cargo-deny:
	@cargo install cargo-deny --locked
install-cargo-udeps:
	@cargo install cargo-udeps --locked
install-cargo-llvm-cov:
	@cargo install cargo-llvm-cov --locked
install-cargo-dist:
	@cargo install cargo-dist --locked
install-git-cliff:
	@cargo install git-cliff --locked
install-cargo-nextest:
	@cargo install cargo-nextest --locked

clean:
	@cargo clean
check:
	@cargo check
fix:
	@cargo fix --allow-dirty --allow-staged
doc:
	@cargo doc
doc-rs:
	@cargo +nightly docs-rs
doc-test:
	@cargo test --doc
test:
	@cargo test
nextest:
	@cargo nextest run
coverage:
	@cargo +nightly llvm-cov \
		--all-features \
		--workspace \
		--doctests \
		--html \
		--open
build:
	@cargo build
rustfmt-check:
	@cargo +nightly fmt --all -- --check
rustfmt: fix
	@cargo +nightly fmt --all
clippy:
	@cargo clippy --all-targets --all-features
deny:
	@cargo deny --all-features --log-level error check
udeps:
	@cargo +nightly udeps
