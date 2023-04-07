use std::{net::SocketAddr, sync::Arc};

use lib::{
    entity::{Msg, ServerInfo, ServerStatus, ServerType, Type},
    net::client::{ClientConfigBuilder, ClientMultiConnection, SubConnectionConfig},
    Result,
};
use tracing::error;

use crate::{config::CONFIG, util::my_id};

use super::{get_cluster_connection_map, MsgSender};

pub(super) struct Client {
    multi_client: ClientMultiConnection,
}

impl Client {
    pub(super) fn new() -> Self {
        let mut client_config = ClientConfigBuilder::default();
        client_config
            .with_remote_address("[::1]:0".parse().unwrap())
            .with_ipv4_type(CONFIG.server.cluster_address.is_ipv4())
            .with_domain(CONFIG.server.domain.clone())
            .with_cert(CONFIG.server.cert.clone())
            .with_keep_alive_interval(CONFIG.transport.keep_alive_interval)
            .with_max_bi_streams(CONFIG.transport.max_bi_streams);
        let client_config = client_config.build().unwrap();
        let multi_client = ClientMultiConnection::new(client_config).unwrap();
        Self { multi_client }
    }

    pub(super) async fn new_connection(&self, remote_address: SocketAddr) -> Result<()> {
        let cluster_map = get_cluster_connection_map().0;
        let sub_config = SubConnectionConfig {
            remote_address,
            domain: CONFIG.server.domain.clone(),
            opened_bi_streams_number: CONFIG.transport.max_bi_streams,
            timeout: std::time::Duration::from_millis(3000),
        };
        let mut service_address = CONFIG.server.service_address;
        service_address.set_ip(CONFIG.server.service_ip.parse().unwrap());
        let mut cluster_address = CONFIG.server.cluster_address;
        cluster_address.set_ip(CONFIG.server.cluster_ip.parse().unwrap());
        let server_info = ServerInfo {
            id: my_id(),
            service_address,
            cluster_address: Some(cluster_address),
            connection_id: 0,
            status: ServerStatus::Online,
            typ: ServerType::MessageCluster,
            load: None,
        };
        let mut auth = Msg::raw_payload(&server_info.to_bytes());
        auth.set_type(Type::Auth);
        auth.set_sender(server_info.id as u64);
        let (mut conn, auth_resp) = self
            .multi_client
            .new_timeout_connection(sub_config, Arc::new(auth))
            .await?;
        let (sender, receiver, timeout) = conn.operation_channel();
        let res_server_info = ServerInfo::from(auth_resp.payload());
        cluster_map.insert(res_server_info.id, MsgSender::Client(sender.clone()));
        tokio::spawn(async move {
            // extend lifetime of connection
            let _conn = conn;
            if let Err(e) =
                super::handler::handler_func(MsgSender::Client(sender), receiver, timeout).await
            {
                error!("handler_func error: {}", e);
            }
        });
        Ok(())
    }
}
