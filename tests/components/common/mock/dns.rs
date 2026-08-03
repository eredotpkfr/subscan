use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
    time::Duration,
};

use hickory_server::{
    net::runtime::Time,
    proto::{
        op::{Header, HeaderCounts, MessageType, Metadata, OpCode, ResponseCode},
        rr::{
            domain::Name,
            rdata::{A, AAAA, NS},
            LowerName, RData, Record, RecordType,
        },
    },
    server::{Request, RequestHandler, ResponseHandler, ResponseInfo},
    zone_handler::MessageResponseBuilder,
    Server,
};
use tokio::net::TcpListener;

use crate::common::{constants::LOCAL_HOST, utils::get_random_port};

#[derive(Clone)]
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
        let socket = SocketAddr::from_str(&format!("{LOCAL_HOST}:{port}"));
        let zone = Name::from_str(domain).unwrap();

        Self {
            handler: MockDNSHandler::new(zone.into()),
            socket: socket.unwrap(),
        }
    }

    pub async fn start(&self) {
        let mut server = Server::new(self.handler.clone());
        let listener = TcpListener::bind(self.socket).await.unwrap();

        server.register_listener(listener, Duration::from_secs(10), 32);
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
        match request.queries.queries().first().unwrap().query_type() {
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
        if request.metadata.op_code != OpCode::Query
            || request.metadata.message_type != MessageType::Query
        {
            return None;
        }

        match request.queries.queries().first().unwrap().name() {
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
        let metadata = Metadata::response_from_request(&request.metadata);

        let name = Name::from_utf8("ns.foo.com").unwrap();
        let rdata = RData::NS(NS(name));

        let records = [Record::from_rdata(
            request.queries.queries().first().unwrap().name().into(),
            60,
            rdata,
        )];
        let response = builder.build(metadata, records.iter(), &[], &[], &[]);

        responder.send_response(response).await.ok()
    }

    async fn handle_axfr_query<R: ResponseHandler>(
        &self,
        request: &Request,
        mut responder: R,
    ) -> Option<ResponseInfo> {
        let builder = MessageResponseBuilder::from_message_request(request);
        let metadata = Metadata::response_from_request(&request.metadata);

        let rdata = match request.src().ip() {
            IpAddr::V4(ipv4) => RData::A(A(ipv4)),
            IpAddr::V6(ipv6) => RData::AAAA(AAAA(ipv6)),
        };

        let name_one = Name::from_utf8("bar.foo.com").unwrap();
        let name_two = Name::from_utf8("*.foo.com").unwrap();

        let records = [
            Record::from_rdata(name_one, 60, rdata.clone()),
            Record::from_rdata(name_two, 60, rdata),
        ];
        let response = builder.build(metadata, records.iter(), &[], &[], &[]);

        responder.send_response(response).await.ok()
    }

    async fn handle_a_and_aaaa_query<R: ResponseHandler>(
        &self,
        request: &Request,
        mut responder: R,
    ) -> Option<ResponseInfo> {
        let builder = MessageResponseBuilder::from_message_request(request);
        let metadata = Metadata::response_from_request(&request.metadata);

        let rdata = match request.src().ip() {
            IpAddr::V4(ipv4) => RData::A(A(ipv4)),
            IpAddr::V6(ipv6) => RData::AAAA(AAAA(ipv6)),
        };

        let records = [Record::from_rdata(
            request.queries.queries().first().unwrap().name().into(),
            60,
            rdata,
        )];
        let response = builder.build(metadata, records.iter(), &[], &[], &[]);

        responder.send_response(response).await.ok()
    }
}

#[async_trait::async_trait]
impl RequestHandler for MockDNSHandler {
    async fn handle_request<R: ResponseHandler, T: Time>(
        &self,
        request: &Request,
        response: R,
    ) -> ResponseInfo {
        if let Some(info) = self.handle_request(request, response).await {
            info
        } else {
            let mut metadata = Metadata::new(0, MessageType::Query, OpCode::Query);

            metadata.response_code = ResponseCode::ServFail;

            Header {
                metadata,
                counts: HeaderCounts::default(),
            }
            .into()
        }
    }
}
