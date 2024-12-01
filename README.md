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
<p align="center">
  <a href="#">Install</a> •
  <a href="#">Usage</a> •
  <a href="#">Doc</a> •
  <a href="#">Book</a> •
  <a href="#">Development</a>
</p>
<br>
<!-- markdownlint-enable MD033 MD041 -->

🔍🕵️ **Subscan** is a powerful subdomain enumeration tool built with [Rust](https://www.rust-lang.org/), specifically designed for penetration testing purposes. It combines various discovery techniques into a single, lightweight binary, making subdomain hunting easier and faster for security researchers
<!-- markdownlint-disable MD007 -->
## Features

🎯 **Let's Dive Into What Makes `Subscan` Super Cool**

- 🕵️ **Smart Discovery Tricks:**
   - Leverage multiple search engines including `Google`, `Yahoo`, `Bing`, and `Duckduckgo` for extensive subdomain discovery
   - Integrate seamlessly with leading reconnaissance APIs such as `Shodan`, `Censys`, `VirusTotal` and more
   - Perform a zone transfer check on the target domain
   - Execute subdomain brute-forcing with optimized wordlists
- 🔍 **IP Detective:** Resolve IP addresses for all discovered subdomains
- 🛠️ **Completely Configurable:**
   - Customize HTTP requests, such as user-agent, timeout, and more
   - Rotate HTTP requests through proxies using the `--proxy` argument
   - Fine-tune the IP resolver component with arguments that start with `--resolver`
   - Use the `--skips` and `--modules` arguments to filter and run only the specific modules you want
- 📎 **Flexible Reporting:**
   - Generate reports in `CSV`, `HTML`, `JSON`, or `TXT` formats
   - Generate detailed `JSON` reports for technical analysis and insights
- 🐳 **Docker Friendly:**
   - Native support for Linux architectures, including `amd64` and `arm64` platforms
   - A tiny container that won't eat up your storage — under 1GB and ready to roll
- 💻 **Cross Platform:** Install effortlessly as a single binary compatible across multiple platforms
- 🚀 **Super Lightweight:** A minimalist design with a small Docker image size and an even smaller binary

<!-- markdownlint-enable MD007 -->
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

## Environments

All environments are managed by the `.env` file. Subscan can read your environments from this `.env` file. You can refer to the `.env.template` file to see how to create them. Also you can specify your environments from shell:

```bash
SUBSCAN_VIRUSTOTAL_APIKEY=foo subscan scan -d foo.com --modules=virustotal
```

<!-- markdownlint-disable MD033 MD041 -->
<div align="center">

| Name                           | Required | Description |
| :----------------------------: | :------: | :---------: |
| `SUBSCAN_CHROME_PATH`          | `false`  | Specify your Chrome executable. If not specified, the Chrome binary will be fetched automatically by <a href="https://github.com/rust-headless-chrome/rust-headless-chrome/">headless_chrome<a/> based on your system architecture |
| `SUBSCAN_<MODULE_NAME>_HOST` | `false`  | Some API integration modules can provide user specific host, for these cases, set module specific host |
| `SUBSCAN_<MODULE_NAME>_APIKEY` | `false`  | Some modules may include API integration and require an API key for authentication. Set the API key in these cases |
| `SUBSCAN_<MODULE_NAME>_USERNAME` | `false`  | Set the username for a module if it uses HTTP basic authentication |
| `SUBSCAN_<MODULE_NAME>_PASSWORD` | `false`  | Set the password for a module if it uses HTTP basic authentication |

</div>
<!-- markdownlint-enable MD033 MD041 -->
