use crate::{
    enums::{
        dispatchers::{RequesterDispatcher, SubdomainExtractorDispatcher, SubscanModuleDispatcher},
        module::SubscanModuleStatus::Finished,
    },
    interfaces::module::SubscanModuleInterface,
    types::{core::Subdomain, result::module::SubscanModuleResult},
};
use async_trait::async_trait;
use hickory_client::{
    client::{AsyncClient, ClientHandle},
    proto::iocompat::AsyncIoTokioAsStd,
    rr::{domain::Name, DNSClass, RecordType},
    tcp::TcpClientStream,
};
use hickory_resolver::{
    config::{NameServerConfig, NameServerConfigGroup, Protocol::Tcp},
    system_conf,
};
use std::{net::SocketAddr, str::FromStr};
use tokio::{net::TcpStream as TokioTcpStream, sync::Mutex};

pub const ZONETRANSFER_MODULE_NAME: &str = "zonetransfer";

/// `ZoneTransfer` non-generic module
///
/// | Property           | Value          |
/// |:------------------:|:--------------:|
/// | Module Name        | `zonetransfer` |
/// | Requester          | [`None`]       |
/// | Extractor          | [`None`]       |
/// | Generic            | [`None`]       |
pub struct ZoneTransfer {
    /// Module name
    pub name: String,
}

impl ZoneTransfer {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let zonetransfer = Self {
            name: ZONETRANSFER_MODULE_NAME.into(),
        };

        zonetransfer.into()
    }

    async fn get_default_ns(&self) -> Option<NameServerConfig> {
        let tcp = |ns: &&NameServerConfig| ns.protocol == Tcp;

        if let Ok((config, _)) = system_conf::read_system_conf() {
            config.name_servers().iter().find(tcp).cloned()
        } else {
            NameServerConfigGroup::google().iter().find(tcp).cloned()
        }
    }

    async fn get_async_client(&self, server: SocketAddr) -> Option<AsyncClient> {
        type WrappedStream = AsyncIoTokioAsStd<TokioTcpStream>;

        let (stream, sender) = TcpClientStream::<WrappedStream>::new(server);
        let client = AsyncClient::new(stream, sender, None);

        if let Ok((client, bg)) = client.await {
            tokio::spawn(bg);

            Some(client)
        } else {
            None
        }
    }

    async fn get_ns_records_as_ip(&self, domain: &str) -> Option<Vec<SocketAddr>> {
        let default_ns = self.get_default_ns().await?;

        let mut ip_addresses = vec![];
        let mut client = self.get_async_client(default_ns.socket_addr).await?;

        let name = Name::from_str(domain).unwrap();
        let ns_response = client.query(name, DNSClass::IN, RecordType::NS);

        for answer in ns_response.await.ok()?.answers() {
            let name = Name::from_str(&answer.data()?.as_ns()?.to_utf8()).ok()?;
            let a_response = client.query(name, DNSClass::IN, RecordType::A).await;

            for answer in a_response.ok()?.answers() {
                let with_port = format!("{}:53", answer.data()?);
                let socketaddr = SocketAddr::from_str(&with_port);

                ip_addresses.push(socketaddr.ok()?)
            }
        }

        Some(ip_addresses)
    }

    async fn attempt_zone_transfer(&self, server: SocketAddr, domain: &str) -> Vec<Subdomain> {
        let mut client = self.get_async_client(server).await.unwrap();
        let mut subs = vec![];

        let name = Name::from_str(domain).unwrap();
        let axfr_response = client.query(name, DNSClass::IN, RecordType::AXFR).await;

        if let Ok(response) = axfr_response {
            for answer in response.answers() {
                if let Some(data) = answer.data() {
                    let rtype = data.record_type();

                    if rtype == RecordType::A || rtype == RecordType::AAAA {
                        let name = answer.name().to_string();
                        let sub = name.strip_suffix(".").unwrap_or(&name);

                        subs.push(sub.to_string());
                    }
                }
            }
        }

        subs
    }
}

#[async_trait]
impl SubscanModuleInterface for ZoneTransfer {
    async fn name(&self) -> &str {
        &self.name
    }

    async fn requester(&self) -> Option<&Mutex<RequesterDispatcher>> {
        None
    }

    async fn extractor(&self) -> Option<&SubdomainExtractorDispatcher> {
        None
    }

    async fn run(&mut self, domain: &str) -> SubscanModuleResult {
        let mut result: SubscanModuleResult = self.name().await.into();

        if let Some(ips) = self.get_ns_records_as_ip(domain).await {
            for ip in ips {
                result.extend(self.attempt_zone_transfer(ip, domain).await);
            }
        }

        result.with_status(Finished).await
    }
}
