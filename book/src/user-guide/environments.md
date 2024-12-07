# Environments

`Subscan` has the ability to read all your environment variables from the `.env` file in your working directory. To learn how to define your environment variables in the `.env` file, you can refer to the `.env.template` file. All the `Subscan` environment variables uses `SUBSCAN` namespace as a prefix

There are two types of environment variables:

- **Dynamic:** These environment variables follow a specific format (e.g., `SUBSCAN_<MODULE_NAME>_FOO`) and `Subscan` can read them automatically
- **Static:** These are predefined environment variables that we know already

## Statics

<!-- markdownlint-disable MD033 MD041 -->

| Name                           | Required | Description |
| :----------------------------- | :------: | :---------: |
| `SUBSCAN_CHROME_PATH`          | `false`  | Specify your Chrome executable. If not specified, the Chrome binary will be fetched automatically by <a href="https://github.com/rust-headless-chrome/rust-headless-chrome/">headless_chrome<a/> based on your system architecture |

<!-- markdownlint-enable MD033 MD041 -->

## Dynamics

| Name                           | Required | Description |
| :----------------------------- | :------: | :---------: |
| `SUBSCAN_<MODULE_NAME>_HOST` | `false`  | Some API integration modules can provide user specific host, for these cases, set module specific host |
| `SUBSCAN_<MODULE_NAME>_APIKEY` | `false`  | Some modules may include API integration and require an API key for authentication. Set the API key in these cases |
| `SUBSCAN_<MODULE_NAME>_USERNAME` | `false`  | Set the username for a module if it uses HTTP basic authentication |
| `SUBSCAN_<MODULE_NAME>_PASSWORD` | `false`  | Set the password for a module if it uses HTTP basic authentication |

## Creating `.env` File

Please see the [.env.template](https://github.com/eredotpkfr/subscan/blob/main/.env.template) file in project repository. Your `.env` file should follow a similar format as shown below

```bash
SUBSCAN_BEVIGIL_APIKEY=foo
SUBSCAN_BINARYEDGE_APIKEY=bar
SUBSCAN_BUFFEROVER_APIKEY=baz
```
