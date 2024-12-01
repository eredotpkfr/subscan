<!-- markdownlint-disable MD033 MD041 -->
<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://github.com/eredotpkfr/subscan/blob/main/assets/logo-light.png">
    <img alt="Subscan Logo" height="105px" src="https://github.com/eredotpkfr/subscan/blob/main/assets/logo-dark.png">
  </picture>
</div>
<br>
<div align="center">
  <a href="https://github.com/eredotpkfr/subscan/actions/workflows/tests.yml">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/actions/workflow/status/eredotpkfr/subscan/tests.yml?label=test&logo=Github&labelColor=ff3030&color=e6e6e6">
      <img alt="GitHub Actions Test Workflow Status" src="https://img.shields.io/github/actions/workflow/status/eredotpkfr/subscan/tests.yml?label=test&logo=Github&labelColor=42445a&color=e6e6e6">
    </picture>
  </a>

  <a href="https://app.codecov.io/gh/eredotpkfr/subscan">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/codecov/c/gh/eredotpkfr/subscan?labelColor=ff3030&color=e6e6e6&logo=codecov&logoColor=white">
      <img alt="Codecov Status" src="https://img.shields.io/codecov/c/gh/eredotpkfr/subscan?labelColor=42445a&color=e6e6e6&logo=codecov&logoColor=white">
    </picture>
  </a>

  <a href="https://github.com/eredotpkfr/subscan/actions/workflows/docker.yml">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/actions/workflow/status/eredotpkfr/subscan/docker.yml?label=docker&logo=Docker&labelColor=ff3030&color=e6e6e6&logoColor=white">
      <img alt="Codecov Status" src="https://img.shields.io/github/actions/workflow/status/eredotpkfr/subscan/docker.yml?label=docker&logo=Docker&labelColor=42445a&color=e6e6e6&logoColor=white">
    </picture>
  </a>

  <a href="https://github.com/eredotpkfr/subscan/blob/main/LICENSE">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/license/eredotpkfr/subscan?labelColor=ff3030&color=e6e6e6">
      <img alt="Codecov Status" src="https://img.shields.io/github/license/eredotpkfr/subscan?labelColor=42445a&color=e6e6e6">
    </picture>
  </a>
</div>
<div align="center">
  <a href="https://pre-commit.com/">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit&logoColor=white&color=e6e6e6&labelColor=ff3030">
      <img alt="Codecov Status" src="https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit&logoColor=white&color=e6e6e6&labelColor=42445a">
    </picture>
  </a>

  <a href="https://gitleaks.io/">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/badge/protected%20by-gitleaks-blue?color=e6e6e6&labelColor=ff3030">
      <img alt="Codecov Status" src="https://img.shields.io/badge/protected%20by-gitleaks-blue?color=e6e6e6&labelColor=42445a">
    </picture>
  </a>

  <a href="https://github.com/rust-secure-code/safety-dance/">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/badge/unsafe-forbidden-success.svg?color=e6e6e6&labelColor=ff3030">
      <img alt="Unsafe Forbidden" src="https://img.shields.io/badge/unsafe-forbidden-success.svg?color=e6e6e6&labelColor=42445a">
    </picture>
  </a>
</div>
<br>
<h5 align="center">
  <a href="#">Install</a> •
  <a href="#">Usage</a> •
  <a href="#">Doc</a> •
  <a href="#">Book</a> •
  <a href="#">Development</a>
</h5>
<br>
<!-- markdownlint-enable MD033 MD041 -->

🔍🕵️ **Subscan** is a powerful subdomain enumeration tool built with Rust, specifically designed for penetration testing purposes. It combines multiple discovery techniques into a single, lightweight binary, making subdomain hunting easier and faster for security researchers

## Features

## Install

```bash
cargo install subscan
```

## Usage

```bash
Usage: subscan [OPTIONS] <COMMAND>

Commands:
  scan    Start scan on any domain address
  brute   Start brute force attack with a given wordlist
  module  Subcommand to manage implemented modules
  help    Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
  -h, --help        Print help (see more with '--help')
  -V, --version     Print version
```
