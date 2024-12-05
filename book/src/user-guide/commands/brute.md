# `brute`

With this command you can use the brute force technique to discover subdomains on a domain

## Argument List

All arguments below can be used with the `brute` command, [see here](#common-use-cases) for common use cases

| Name                     | Short |                   Description                    |
| :----------------------- | :---: | :----------------------------------------------: |
| `--domain`               | `-d`  |           Target domain to be scanned            |
| `--wordlist`             | `-w`  |      Wordlist file to be used during attack      |
| `--print`                | `-p`  |     If sets, output will be logged on stdout     |
| `--output`               | `-o`  | Set output format (`txt`, `csv`, `json`, `html`) |
| `--resolver-timeout`     |       |               IP resolver timeout                |
| `--resolver-concurrency` |       |         IP resolver concurrency level            |
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
