# Setup Development Environment

This section covers topics like setting up a development environment and running tests for those who want to contribute to `Subscan`

To set up your development environment, please follow the instructions below

1. Clone repository

   ```bash
   ~$ git clone https://github.com/eredotpkfr/subscan && cd subscan
   ```

2. Install `pre-commit` and its hooks

   ```bash
   ~$ # Install pre-commit Mac or Linux
   ~$ make install-pre-commit-mac
   ~$ # Install pre-commit hooks
   ~$ make install-pre-commit-hooks
   ~$ # Check everything is OK
   ~$ pre-commit run -a
   ```

3. Install required cargo tools for development

   ```bash
   ~$ # Install cargo tools
   ~$ make install-cargo-tools
   ```

4. Create `.env` file from `.env.template`

   ```bash
   ~$ cp .env.template .env
   ```

5. Finally build the project and run CLI

   ```bash
   ~$ cargo build && target/debug/subscan --help
   ```

## Running Tests

You have many options to run the tests, below are the command sets on how to run the tests differently

```bash
~$ # run all tests
~$ cargo test # or `make test`
~$ # capture outputs
~$ cargo test -- --nocapture
~$ # run only doc tests
~$ cargo test --doc
~$ # run a single test
~$ cargo test -- engines::bing_test::bing_run_test
~$ # run only integration tests
~$ cargo test --tests modules::integrations
```

To run tests via [nextest](https://nexte.st/), run following command

```bash
~$ make nextest
```

Create coverage report with [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov)

```bash
~$ make coverage
```

## Building Docs

To build documentations, run following command

```bash
~$ make doc # or `cargo doc`
```

To serve project book with hot reload, use following command

```bash
~$ # run book tests and serve
~$ make live-book
```
