# Generic Modules

Some module implementations are very similar to each other, and sometimes we can use the same logic and algorithms while performing subdomain discovery. For example, during an API integration, the following steps will almost always be the same for most modules

- Make an API call to the endpoint
- Parse the subdomains from the incoming `JSON` content
- Check if there is a pagination
  - If pagination exists, go back to step 1 for the next page
  - If there is no pagination, break the loop
- Return the discovered subdomains for `Subscan`

To reduce the implementation time and avoid code duplication in `Subscan`, there are generic modules. Some of the registered modules in `Subscan` use these generic implementations, which can be viewed with the [`subscan module list`](../../user-guide/commands/module.md#list) command. Below are details of two modules, one using a generic module and one not

```bash
~$ subscan module get alienvault
+------------+------------+---------------+-------------+
| Name       | Requester  | Extractor     | Is Generic? |
+------------+------------+---------------+-------------+
| alienvault | HTTPClient | JSONExtractor | true        |
+------------+------------+---------------+-------------+
```

```bash
~$ subscan module get zonetransfer
+--------------+-----------+-----------+-------------+
| Name         | Requester | Extractor | Is Generic? |
+--------------+-----------+-----------+-------------+
| zonetransfer | None      | None      | false       |
+--------------+-----------+-----------+-------------+
```

The `zonetransfer` module is a very custom subdomain discovery method that performs DNS queries, so we cannot define it generically. Also as you can see, it has not any `Requester` or `Extractor` component. However, a module that makes API calls and parses the resulting output, such as the `alienvault` module, can use a generic module like [`GenericIntegrationModule`](https://docs.rs/subscan/latest/subscan/modules/generics/integration/struct.GenericIntegrationModule.html) within its implementation and return an instance of [`GenericIntegrationModule`](https://docs.rs/subscan/latest/subscan/modules/generics/integration/struct.GenericIntegrationModule.html) during its implementation

The following generic modules are defined within `Subscan`. For more details, follow the links provided

- [GenericIntegrationModule](integration.md)
- [GenericSearchEngineModule](engine.md)
