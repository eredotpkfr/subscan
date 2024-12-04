# Usage

This chapter explains how to use the `Subscan` CLI to discover subdomains. Each subdomain discovery feature is implemented as a `SubscanModule`. These modules are executed automatically when a scan is initiated. For more details, refer to the [Development](/development/index.html) chapter

🛠️ The `Subscan` CLI is a versatile tool that provides the following functionalities:

- [Start a scan](scan.md) to discover subdomains associated with a specific domain
- [Perform a brute force](brute.md) attack on a domain using a specified wordlist
- Manage registered modules. See the [module](/user-guide/commands/module.md) command details

✨ Here's a quick overview of how to use it:

```bash
~$ subscan

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
