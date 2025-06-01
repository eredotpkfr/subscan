# `scan`

This command starts a scan by running registered modules for subdomain discovery. See the [module](module.md) command to manage registered modules

## Argument List

All arguments below can be used with the `scan` command and you can customize a scan according to your needs, [see here](#common-use-cases) for common use cases

| Name                     | Short |                   Description                    |
| :----------------------- | :---: | :----------------------------------------------: |
| `--domain`               | `-d`  |           Target domain to be scanned            |
| `--user-agent`           | `-u`  |            Set a `User-Agent` header             |
| `--http-timeout`         | `-t`  |             HTTP timeout as seconds              |
| `--proxy`                | `-p`  |                  Set HTTP proxy                  |
| `--output`               | `-o`  | Set output format (`txt`, `csv`, `json`, `html`) |
| `--print`                |       |     If sets, output will be logged on stdout     |
| `--module-concurrency`   | `-c`  |         Module runner concurrency level          |
| `--resolver-timeout`     |       |               IP resolver timeout                |
| `--resolver-concurrency` |       |         IP resolver concurrency level            |
| `--resolver-list`        |       | A text file containing list of resolvers. See `resolverlist.template` |
| `--disable-ip-resolve`   |       |        Disable IP address resolve process        |
| `--modules`              | `-m`  |      Comma separated list of modules to run      |
| `--skips`                | `-s`  |     Comma separated list of modules to skip      |
| `--help`                 | `-h`  |                    Print help                    |

## Common Use Cases

- Adjust HTTP request timeouts for slow networks

  ```bash
  ~$ subscan scan -d example.com -t 120
  ```

- Use a proxy server to bypass anti-bot systems

  ```bash
  ~$ subscan scan -d example.com -t 120 --proxy 'http://my.prox:4444'
  ```

- Increase concurrency to speed up the scan

  ```bash
  ~$ subscan scan -d example.com -c 10
  ```

- Fine-tune IP address resolver component according to your network

  ```bash
  ~$ subscan scan -d example.com --resolver-timeout 1 --resolver-concurrency 100
  ```

- Disable the IP resolution process

  ```bash
  ~$ subscan scan -d example.com --disable-ip-resolve
  ```

- Customize the scan by filtering modules

  ```bash
  # skip the commoncrawl and google modules during the scan
  ~$ subscan scan -d example.com --skips=commoncrawl,google
  ```

  ```bash
  # run only the virustotal module
  ~$ subscan scan -d example.com --modules=virustotal
  ```

  > If a module is included in both the `--skips` and `--modules` arguments, it will be skipped and not executed
