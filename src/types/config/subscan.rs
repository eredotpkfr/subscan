use crate::{
    cli::{
        commands::{
            brute::BruteCommandArgs,
            module::{run::ModuleRunSubCommandArgs, ModuleSubCommands},
            scan::ScanCommandArgs,
            Commands,
        },
        Cli,
    },
    constants::{DEFAULT_HTTP_TIMEOUT, DEFAULT_MODULE_CONCURRENCY, DEFAULT_USER_AGENT},
    enums::cache::CacheFilter,
    types::config::resolver::ResolverConfig,
};
use std::path::PathBuf;

/// `Subscan` configurations as a struct type
#[derive(Clone, Debug)]
pub struct SubscanConfig {
    pub concurrency: u64,
    pub user_agent: String,
    pub timeout: u64,
    pub proxy: Option<String>,
    pub filter: CacheFilter,
    pub resolver: ResolverConfig,
    pub wordlist: Option<PathBuf>,
}

impl Default for SubscanConfig {
    /// Create [`SubscanConfig`] with defaults
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::config::subscan::SubscanConfig;
    /// use subscan::constants::{DEFAULT_MODULE_CONCURRENCY, DEFAULT_HTTP_TIMEOUT, DEFAULT_USER_AGENT};
    ///
    /// let config = SubscanConfig::default();
    ///
    /// assert_eq!(config.concurrency, DEFAULT_MODULE_CONCURRENCY);
    /// assert_eq!(config.timeout, DEFAULT_HTTP_TIMEOUT.as_secs());
    /// assert_eq!(config.user_agent, DEFAULT_USER_AGENT);
    /// ```
    fn default() -> Self {
        Self {
            concurrency: DEFAULT_MODULE_CONCURRENCY,
            timeout: DEFAULT_HTTP_TIMEOUT.as_secs(),
            user_agent: DEFAULT_USER_AGENT.into(),
            filter: CacheFilter::default(),
            resolver: ResolverConfig::default(),
            wordlist: None,
            proxy: None,
        }
    }
}

impl From<ModuleRunSubCommandArgs> for SubscanConfig {
    /// Create [`SubscanConfig`] from [`ModuleRunSubCommandArgs`]
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::config::subscan::SubscanConfig;
    /// use subscan::cli::commands::module::run::ModuleRunSubCommandArgs;
    /// use subscan::enums::output::OutputFormat;
    ///
    /// let args = ModuleRunSubCommandArgs {
    ///     http_timeout: 120,
    ///     ..Default::default()
    /// };
    /// let config = SubscanConfig::from(args.clone());
    ///
    /// assert_eq!(config.timeout, args.http_timeout);
    /// ```
    fn from(args: ModuleRunSubCommandArgs) -> Self {
        Self {
            user_agent: args.clone().user_agent,
            timeout: args.http_timeout,
            proxy: args.clone().proxy,
            resolver: args.into(),
            ..Default::default()
        }
    }
}

impl From<ScanCommandArgs> for SubscanConfig {
    /// Create [`SubscanConfig`] from [`ScanCommandArgs`]
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::config::subscan::SubscanConfig;
    /// use subscan::cli::commands::scan::ScanCommandArgs;
    ///
    /// let args = ScanCommandArgs {
    ///     http_timeout: 120,
    ///     ..Default::default()
    /// };
    /// let config = SubscanConfig::from(args.clone());
    ///
    /// assert_eq!(config.timeout, args.http_timeout);
    /// ```
    fn from(args: ScanCommandArgs) -> Self {
        Self {
            user_agent: args.user_agent.clone(),
            timeout: args.http_timeout,
            proxy: args.proxy.clone(),
            concurrency: args.module_concurrency,
            filter: args.filter(),
            resolver: args.into(),
            ..Default::default()
        }
    }
}

impl From<BruteCommandArgs> for SubscanConfig {
    /// Create [`SubscanConfig`] from [`BruteCommandArgs`]
    ///
    /// # Examples
    ///
    /// ```
    /// use std::{path::PathBuf, str::FromStr};
    /// use subscan::types::config::subscan::SubscanConfig;
    /// use subscan::cli::commands::brute::BruteCommandArgs;
    ///
    /// let wordlist = PathBuf::from_str("wordlist.txt");
    /// let args = BruteCommandArgs {
    ///     resolver_concurrency: 100,
    ///     wordlist: wordlist.unwrap(),
    ///     ..Default::default()
    /// };
    /// let config = SubscanConfig::from(args.clone());
    ///
    /// assert_eq!(config.resolver.concurrency, args.resolver_concurrency);
    /// ```
    fn from(args: BruteCommandArgs) -> Self {
        Self {
            wordlist: Some(args.clone().wordlist),
            resolver: args.into(),
            ..Default::default()
        }
    }
}

impl From<Cli> for SubscanConfig {
    /// Create [`SubscanConfig`] from [`Cli`]
    ///
    /// # Examples
    ///
    /// ```
    /// use clap::Parser;
    /// use subscan::types::config::subscan::SubscanConfig;
    /// use subscan::cli::Cli;
    /// use subscan::constants::DEFAULT_HTTP_TIMEOUT;
    ///
    /// // Scan command
    /// let args = vec!["subscan", "scan", "-d", "foo.com", "-t", "120"];
    /// let cli = Cli::try_parse_from(args).unwrap();
    ///
    /// let config = SubscanConfig::from(cli);
    /// assert_eq!(config.timeout, 120);
    ///
    /// // Brute command
    /// let args = vec![
    ///     "subscan",
    ///     "brute",
    ///     "-d", "foo.com",
    ///     "--resolver-concurrency", "100",
    ///     "--wordlist", "wordlist.txt"
    /// ];
    /// let cli = Cli::try_parse_from(args).unwrap();
    ///
    /// let config = SubscanConfig::from(cli);
    /// assert_eq!(config.resolver.concurrency, 100);
    ///
    /// // Module list command
    /// let args = vec!["subscan", "module", "list"];
    /// let cli = Cli::try_parse_from(args).unwrap();
    ///
    /// let config = SubscanConfig::from(cli);
    /// assert_eq!(config.timeout, DEFAULT_HTTP_TIMEOUT.as_secs());
    ///
    /// // Module get command
    /// let args = vec!["subscan", "module", "get", "foo"];
    /// let cli = Cli::try_parse_from(args).unwrap();
    ///
    /// let config = SubscanConfig::from(cli);
    /// assert_eq!(config.timeout, DEFAULT_HTTP_TIMEOUT.as_secs());
    /// ```
    fn from(cli: Cli) -> Self {
        match cli.command {
            Commands::Module(module) => match module.command {
                ModuleSubCommands::List(_) | ModuleSubCommands::Get(_) => Self::default(),
                ModuleSubCommands::Run(args) => args.into(),
            },
            Commands::Scan(args) => args.into(),
            Commands::Brute(args) => args.into(),
        }
    }
}
