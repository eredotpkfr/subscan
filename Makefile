SHELL=/bin/bash

.PHONY: all

all: book-build \
	book-test \
	build \
	check \
	clean \
	clippy \
	deny \
	doc \
	doc-rs \
	doc-test \
	fix \
	install-cargo-clippy \
	install-cargo-deny \
	install-cargo-dist \
	install-cargo-doc-rs \
	install-cargo-llvm-cov \
	install-cargo-machete \
	install-cargo-mdbook \
	install-cargo-nextest \
	install-cargo-tools \
	install-cargo-udeps \
	install-git-cliff \
	install-nightly-toolchain \
	install-pre-commit-hooks \
	install-pre-commit-linux \
	install-pre-commit-mac \
	live-book \
	machete
	nextest \
	rustfmt \
	rustfmt-check \
	rustup \
	test \
	udeps \
	update-pre-commit-hooks \

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
	install-cargo-mdbook \
	install-cargo-deny \
	install-cargo-udeps \
	install-cargo-llvm-cov \
	install-cargo-dist \
	install-git-cliff \
	install-cargo-nextest \
	install-cargo-machete

install-nightly-toolchain:
	@rustup toolchain install nightly
install-cargo-clippy:
	@rustup component add clippy
install-cargo-doc-rs:
	@cargo install cargo-docs-rs
install-cargo-mdbook:
	@cargo install mdbook
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
install-cargo-machete:
	@cargo install cargo-machete --locked

clean:
	@cargo clean
check:
	@cargo check
fix:
	@cargo fix --allow-dirty --allow-staged
book-build:
	@mdbook build book
book-test:
	@mdbook test book
live-book: book-test
	@mdbook serve book
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
rustup:
	@rustup self update
	@rustup update
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
machete:
	@cargo machete
