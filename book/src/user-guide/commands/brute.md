# `brute`

With this command you can use the brute force technique to discover subdomains on a domain

## Argument List

All arguments below can be used with the `brute` command, [see here](#common-use-cases) for common use cases

| Name                     | Short |                   Description                    |
| :----------------------- | :---: | :----------------------------------------------: |
| `--domain`               | `-d`  |           Target domain to be scanned            |
| `--wordlist`             | `-w`  |      Wordlist file to be used during attack      |
| `--print`                |       |     If sets, output will be logged on stdout     |
| `--stream-to-txt`        | `-s`  | Optional `txt` file to create file stream for the subdomains that found. If sets the `--output` parameter will be disabled |
| `--output`               | `-o`  | Set output format (`txt`, `csv`, `json`, `html`) |
| `--resolver-timeout`     |       |               IP resolver timeout                |
| `--resolver-concurrency` |       |         IP resolver concurrency level            |
| `--resolver-list`        |       | A text file containing list of resolvers. See `resolverlist.template` |
| `--help`                 | `-h`  |                    Print help                    |

## Common Use Cases

- Run a basic brute force attack with default settings

  ```bash
  ~$ subscan brute -d example.com -w wordlist.txt
  ```

- Increase resolver concurrency to improve attack speed

  ```bash
  ~$ subscan brute -d example.com -w wordlist.txt --resolver-concurrency 200
  ```

- Fine-tune IP address resolver component according to your network

  ```bash
  ~$ subscan brute -d example.com -w wordlist.txt --resolver-timeout 1 --resolver-concurrency 100
  ```

- Skip creating a report and print results directly to `stdout`

  ```bash
  ~$ subscan brute -d example.com -w wordlist.txt --print
  ```
