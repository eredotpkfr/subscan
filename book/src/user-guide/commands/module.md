# `module`

`Subscan` is designed with an extensible architecture, where each subdomain discovery component is referred to as a `SubscanModule`. In `Subscan` terminology, any component involved in subdomain discovery is considered a module. You can create your own custom modules and integrate them into `Subscan`. Modules can also include additional components. Details on how to develop and integrate your own modules are available in the [Development](../../development/index.html) chapter

The module command allows you to list the modules registered in `Subscan`, view their details, and run any module. Below are the subcommands that serve these purposes;

## `list`

Lists the modules registered on `Subscan` as a table with their details. The output looks like the following

```bash
~$ subscan module list

+--------------------+---------------+----------------+-------------+
| Name               | Requester     | Extractor      | Is Generic? |
+--------------------+---------------+----------------+-------------+
| bing               | HTTPClient    | HTMLExtractor  | true        |
| duckduckgo         | ChromeBrowser | HTMLExtractor  | true        |
| google             | HTTPClient    | HTMLExtractor  | true        |
| yahoo              | HTTPClient    | HTMLExtractor  | true        |
| alienvault         | HTTPClient    | JSONExtractor  | true        |
| anubis             | HTTPClient    | JSONExtractor  | true        |
+--------------------+---------------+----------------+-------------+
```

## `get`

Gets a single module with details

```bash
~$ subscan module get zonetransfer

+--------------+-----------+-----------+-------------+
| Name         | Requester | Extractor | Is Generic? |
+--------------+-----------+-----------+-------------+
| zonetransfer | None      | None      | false       |
+--------------+-----------+-----------+-------------+
```

## `run`

This command runs the specified module and is primarily used to quickly test a new module during its implementation. It has a similar set of arguments as the `scan` command

### Argument List

| Name                     | Short |            Description                           |
| :----------------------- | :---: | :----------------------------------------------: |
| `--domain`               | `-d`  |    Target domain to be scanned                   |
| `--user-agent`           | `-u`  |     Set a `User-Agent` header                    |
| `--http-timeout`         | `-t`  |      HTTP timeout as seconds                     |
| `--proxy`                | `-p`  |           Set HTTP proxy                         |
| `--output`               | `-o`  | Set output format (`txt`, `csv`, `json`, `html`) |
| `--resolver-timeout`     |       |        IP resolver timeout                       |
| `--resolver-concurrency` |       |  IP resolver concurrency level                   |
| `--disable-ip-resolve`   |       | Disable IP address resolve process               |
| `--help`                 | `-h`  |             Print help                           |

### Common Use Cases

- Run module by name

  ```bash
  ~$ # runs google module on example.com
  ~$ subscan module run google -d example.com
  ```

- Run module by name without IP resolve

  ```bash
  ~$ # runs shodan module on example.com without IP resolve
  ~$ subscan module run shodan -d example.com --disable-ip-resolve
  ```

- If the module has authentication, set it as environment variable

  ```bash
  ~$ # runs censys module on example.com
  ~$ SUBSCAN_CENSYS_APIKEY=foo subscan module run censys -d example.com --user-agent 'subscan' -t 120
  ```

  > For more details about environment variables, refer to the [Environments](../environments.md) chapter
