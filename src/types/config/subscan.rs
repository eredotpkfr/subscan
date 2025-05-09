use std::path::PathBuf;

use super::requester::RequesterConfig;
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
    constants::DEFAULT_MODULE_CONCURRENCY,
    enums::cache::CacheFilter,
    types::config::resolver::ResolverConfig,
};

/// `Subscan` configurations as a struct type
#[derive(Clone, Debug)]
pub struct SubscanConfig {
    pub concurrency: u64,
    pub filter: CacheFilter,
    pub print: bool,
    pub resolver: ResolverConfig,
    pub requester: RequesterConfig,
    pub stream: Option<PathBuf>,
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
    /// use reqwest::header::{USER_AGENT, HeaderValue};
    ///
    /// let config = SubscanConfig::default();
    ///
    /// assert_eq!(
    ///     config.requester.headers.get(USER_AGENT).unwrap(),
    ///     HeaderValue::from_static(DEFAULT_USER_AGENT),
    /// );
    /// assert_eq!(config.concurrency, DEFAULT_MODULE_CONCURRENCY);
    /// assert_eq!(config.requester.timeout, DEFAULT_HTTP_TIMEOUT);
    /// ```
    fn default() -> Self {
        Self {
            concurrency: DEFAULT_MODULE_CONCURRENCY,
            filter: CacheFilter::default(),
            print: false,
            resolver: ResolverConfig::default(),
            requester: RequesterConfig::default(),
            stream: None,
            wordlist: None,
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
    /// assert_eq!(config.requester.timeout.as_secs(), args.http_timeout);
    /// ```
    fn from(args: ModuleRunSubCommandArgs) -> Self {
        Self {
            concurrency: 1,
            print: args.print,
            requester: args.clone().into(),
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
    /// assert_eq!(config.requester.timeout.as_secs(), args.http_timeout);
    /// ```
    fn from(args: ScanCommandArgs) -> Self {
        Self {
            concurrency: args.module_concurrency,
            filter: args.filter(),
            print: args.print,
            resolver: args.clone().into(),
            requester: args.into(),
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
            print: args.print,
            resolver: args.clone().into(),
            stream: args.stream_to_txt,
            wordlist: Some(args.wordlist),
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
    /// assert_eq!(config.requester.timeout.as_secs(), 120);
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
    /// assert_eq!(config.requester.timeout, DEFAULT_HTTP_TIMEOUT);
    ///
    /// // Module get command
    /// let args = vec!["subscan", "module", "get", "foo"];
    /// let cli = Cli::try_parse_from(args).unwrap();
    ///
    /// let config = SubscanConfig::from(cli);
    /// assert_eq!(config.requester.timeout, DEFAULT_HTTP_TIMEOUT);
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
