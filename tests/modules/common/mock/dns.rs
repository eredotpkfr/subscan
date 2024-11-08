use crate::common::{constants::LOCAL_HOST, utils::get_random_port};
use hickory_client::{
    op::{Header, MessageType, OpCode, ResponseCode},
    proto::rr::LowerName,
    rr::{
        rdata::{A, AAAA, NS},
        RData, Record, RecordType,
    },
};
use hickory_resolver::Name;
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
use tokio::net::TcpListener;

pub struct MockDNSServer {
    pub handler: MockDNSHandler,
    pub port: u16,
}

#[derive(Clone, Debug, Default)]
pub struct MockDNSHandler {
    pub zone: LowerName,
}

impl MockDNSServer {
    pub fn new(domain: &str) -> Self {
        Self {
            handler: MockDNSHandler::new(domain),
            port: get_random_port(),
        }
    }

    pub async fn socket_addr(&self) -> SocketAddr {
        SocketAddr::from_str(&format!("{LOCAL_HOST}:{}", self.port)).unwrap()
    }

    pub async fn start(&self) {
        let mut server = ServerFuture::new(self.handler.clone());
        let listener = TcpListener::bind(self.socket_addr().await).await.unwrap();

        server.register_listener(listener, Duration::from_secs(10));
        server.block_until_done().await.unwrap()
    }
}

impl MockDNSHandler {
    pub fn new(zone: &str) -> Self {
        Self {
            zone: LowerName::from(Name::from_str(zone).unwrap()),
        }
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
            RecordType::AXFR => self.handle_axfr_query(request, responder).await,
            RecordType::NS => self.handle_ns_query(request, responder).await,
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

    async fn handle_ns_query<R: ResponseHandler>(
        &self,
        request: &Request,
        mut responder: R,
    ) -> Option<ResponseInfo> {
        let builder = MessageResponseBuilder::from_message_request(request);
        let header = Header::response_from_request(request.header());

        let name = Name::from_utf8("ns.foo.com").unwrap();
        let rdata = RData::NS(NS(name));

        let records = vec![Record::from_rdata(request.query().name().into(), 60, rdata)];
        let response = builder.build(header, records.iter(), &[], &[], &[]);

        responder.send_response(response).await.ok()
    }

    async fn handle_axfr_query<R: ResponseHandler>(
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

        let name_one = Name::from_utf8("bar.foo.com").unwrap();
        let name_two = Name::from_utf8("*.foo.com").unwrap();

        let records = vec![
            Record::from_rdata(name_one, 60, rdata.clone()),
            Record::from_rdata(name_two, 60, rdata),
        ];
        let response = builder.build(header, records.iter(), &[], &[], &[]);

        responder.send_response(response).await.ok()
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
