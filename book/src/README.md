<!-- markdownlint-disable MD033 MD041 -->
<center>
    <br><br><br>
    <img alt="Subscan Logo" height="105px" src="https://github.com/eredotpkfr/subscan/blob/main/assets/logo-light.png?raw=true">
    <br><br><br>
</center>
<!-- markdownlint-enable MD033 MD041 -->

Subscan is a powerful subdomain enumeration tool built with [Rust](https://www.rust-lang.org/), specifically designed for penetration testing purposes. It combines various discovery techniques into a single, lightweight binary, making subdomain hunting easier and faster for security researchers

### Features

- 🕵️ Smart Discovery Tricks
  - Use multiple search engines (`Google`, `Yahoo`, `Bing`, `DuckDuckGo`, etc.)
  - Integrate with APIs like `Shodan`, `Censys`, `VirusTotal` and more
  - Perform zone transfer checks
  - Subdomain brute-forcing with optimized wordlists
- 🔍 Resolve IP addresses for all subdomains
- 📎 Export reports in `CSV`, `HTML`, `JSON`, or `TXT` formats
- 🛠️ Configurable
  - Customize HTTP requests (user-agent, timeout, etc.)
  - Rotate requests via proxies (`--proxy` argument)
  - Fine-tune IP resolver with `--resolver` arguments
  - Filter and run specific modules with `--skips` and `--modules`
- 🐳 Docker Friendly
  - Native support for `amd64` and `arm64` Linux platforms
  - A tiny container that won't eat up your storage — under 1GB and ready to roll 🚀
- 💻 Compatible with multiple platforms and easy to install as a single binary
