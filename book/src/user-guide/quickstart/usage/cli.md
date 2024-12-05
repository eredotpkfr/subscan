# Subscan CLI Usage

🛠️ The `Subscan CLI` is a versatile tool that provides the following functionalities

- [Start a scan](#starting-scan) to discover subdomains associated with a specific domain
- [Perform a brute force](#brute-force) attack on a domain using a specified wordlist
- Manage registered modules. See the [module](/user-guide/commands/module.md) command details

✨ Here's a quick overview of how to use it

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

## Starting Scan

To scan a domain using all available modules, use the following command:

```bash
~$ subscan scan -d example.com
```

You can also choose specific modules to run or skip using the `--skips` and `--modules` arguments. Module names should be provided as a comma-separated list[^note]

```bash
~$ # skip the commoncrawl and google modules during the scan
~$ subscan scan -d example.com --skips=commoncrawl,google
```

```bash
~$ # run only the virustotal module
~$ subscan scan -d example.com --modules=virustotal
```

[^note]: If a module is included in both the `--skips` and `--modules` arguments, it will be skipped and not executed

> If the module you’re using requires authentication, you can provide the necessary credentials, such as an API key, through module-specific environment variables. For more details about environment variables, refer to the [Environments](/user-guide/environments.md) chapter
>
> ```bash
> SUBSCAN_VIRUSTOTAL_APIKEY=foo subscan scan -d example.com --modules=virustotal
> ```

## Brute Force

Use the `brute` command to start a brute force attack with a specific wordlist

```bash
~$ subscan brute -d example.com --wordlist file.txt
```

> To specify wordlist into docker container, see the [Docker](docker.md) usage
