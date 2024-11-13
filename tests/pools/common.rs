use hickory_client::{
    op::{Header, MessageType, OpCode, ResponseCode},
    proto::rr::LowerName,
    rr::{
        rdata::{A, AAAA},
        RData, Record, RecordType,
    },
};
use hickory_resolver::config::{Protocol::Tcp, ResolverConfig as HickoryResolverConfig};
use hickory_resolver::{
    config::{NameServerConfig, ResolverOpts},
    Name,
};
use hickory_server::{
    authority::MessageResponseBuilder,
    server::{Request, RequestHandler, ResponseHandler, ResponseInfo},
    ServerFuture,
};
use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
    time::Duration,
};
use subscan::types::config::resolver::ResolverConfig;
use tokio::net::TcpListener;

pub const LOCAL_HOST: &str = "127.0.0.1";

pub fn get_random_port() -> u16 {
    std::net::TcpListener::bind(format!("{LOCAL_HOST}:0"))
        .unwrap()
        .local_addr()
        .unwrap()
        .port()
}

pub struct MockDNSServer {
    pub handler: MockDNSHandler,
    pub socket: SocketAddr,
}

#[derive(Clone, Debug, Default)]
pub struct MockDNSHandler {
    pub zone: LowerName,
}

impl MockDNSServer {
    pub fn new(domain: &str) -> Self {
        let port = get_random_port();
        let socket = SocketAddr::from_str(&format!("{LOCAL_HOST}:{}", port));
        let zone = Name::from_str(domain).unwrap();

        Self {
            handler: MockDNSHandler::new(zone.into()),
            socket: socket.unwrap(),
        }
    }

    pub async fn get_resolver_config(&self) -> ResolverConfig {
        let mut config = HickoryResolverConfig::new();
        let mut opts = ResolverOpts::default();

        config.add_name_server(NameServerConfig::new(self.socket, Tcp));
        opts.timeout = Duration::from_secs(2);

        ResolverConfig {
            config,
            opts,
            concurrency: 1,
            disabled: false,
        }
    }

    pub async fn start(&self) {
        let mut server = ServerFuture::new(self.handler.clone());
        let listener = TcpListener::bind(self.socket).await.unwrap();

        server.register_listener(listener, Duration::from_secs(10));
        server.block_until_done().await.unwrap()
    }
}

impl MockDNSHandler {
    pub fn new(zone: LowerName) -> Self {
        Self { zone }
    }

    async fn handle_zone<R: ResponseHandler>(
        &self,
        request: &Request,
        responder: R,
    ) -> Option<ResponseInfo> {
        match request.query().query_type() {
            RecordType::A | RecordType::AAAA => {
                self.handle_a_and_aaaa_query(request, responder).await
            }
            _ => None,
        }
    }

    async fn handle_request<R: ResponseHandler>(
        &self,
        request: &Request,
        response: R,
    ) -> Option<ResponseInfo> {
        if request.op_code() != OpCode::Query || request.message_type() != MessageType::Query {
            return None;
        }

        match request.query().name() {
            name if self.zone.zone_of(name) => self.handle_zone(request, response).await,
            _ => None,
        }
    }

    async fn handle_a_and_aaaa_query<R: ResponseHandler>(
        &self,
        request: &Request,
        mut responder: R,
    ) -> Option<ResponseInfo> {
        let builder = MessageResponseBuilder::from_message_request(request);
        let header = Header::response_from_request(request.header());

        let rdata = match request.src().ip() {
            IpAddr::V4(ipv4) => RData::A(A(ipv4)),
            IpAddr::V6(ipv6) => RData::AAAA(AAAA(ipv6)),
        };

        let records = vec![Record::from_rdata(request.query().name().into(), 60, rdata)];
        let response = builder.build(header, records.iter(), &[], &[], &[]);

        responder.send_response(response).await.ok()
    }
}

#[async_trait::async_trait]
impl RequestHandler for MockDNSHandler {
    async fn handle_request<R: ResponseHandler>(
        &self,
        request: &Request,
        response: R,
    ) -> ResponseInfo {
        if let Some(info) = self.handle_request(request, response).await {
            info
        } else {
            let mut header = Header::new();

            header.set_response_code(ResponseCode::ServFail);
            header.into()
        }
    }
}
