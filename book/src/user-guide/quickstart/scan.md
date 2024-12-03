# Starting Scan

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
