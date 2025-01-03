use std::fmt::Debug;
use std::net::SocketAddr;

use anyhow::{anyhow, Context};
use serde::{Deserialize, Serialize};

use netlink_http::{default_tcp_stun, default_udp_stun, Config, ConfigBuilder};

use crate::{CMD_ADDRESS, DEFAULT_ALGORITHM, LISTEN_PORT};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct FileConfigView {
    pub api_addr: SocketAddr,
    pub api_disable: bool,
    pub threads: usize,

    pub group_code: String,
    pub node_ipv4: String,
    pub prefix: u8,
    pub node_ipv6: Option<String>,
    pub prefix_v6: u8,
    pub tun_name: Option<String>,
    pub encrypt: Option<String>,
    pub algorithm: String,
    pub port: u16,
    pub peer: Option<Vec<String>>,
    pub bind_dev_name: Option<String>,
    pub exit_node: Option<String>,

    pub udp_stun: Vec<String>,
    pub tcp_stun: Vec<String>,
}

impl FileConfigView {
    pub fn read_file(file_path: &str) -> anyhow::Result<Self> {
        let conf = std::fs::read_to_string(file_path)?;
        let file_conf = serde_yaml::from_str::<FileConfigView>(&conf)?;
        file_conf.check()?;
        Ok(file_conf)
    }
    pub fn check(&self) -> anyhow::Result<()> {
        if self.group_code.trim().is_empty() {
            Err(anyhow!("group_code cannot be empty"))?
        }
        if self.node_ipv4.trim().is_empty() {
            Err(anyhow!("node_ipv4 cannot be empty"))?
        }
        Ok(())
    }
}

impl TryFrom<FileConfigView> for Config {
    type Error = anyhow::Error;

    fn try_from(value: FileConfigView) -> Result<Self, Self::Error> {
        let node_ipv6 = if let Some(node_ipv6) = value.node_ipv6 {
            Some(node_ipv6.parse().context("node_ipv6 format error")?)
        } else {
            None
        };
        let mut builder = ConfigBuilder::new()
            .udp_stun(value.udp_stun)
            .tcp_stun(value.tcp_stun)
            .node_ipv4(value.node_ipv4.parse().context("node_ipv4 format error")?)
            .node_ipv6(node_ipv6)
            .prefix(value.prefix)
            .prefix_v6(value.prefix_v6)
            .group_code(value.group_code.try_into()?)
            .port(value.port)
            .algorithm(Some(value.algorithm))
            .encrypt(value.encrypt)
            .config_name(Some("file_config".to_string()))
            .tun_name(value.tun_name)
            .bind_dev_name(value.bind_dev_name)
            .peer_str(value.peer)?;

        if let Some(exit_node) = value.exit_node {
            builder = builder.exit_node(Some(exit_node.parse().context("node_ipv6 format error")?))
        }

        builder.build()
    }
}

impl Default for FileConfigView {
    fn default() -> Self {
        Self {
            api_addr: CMD_ADDRESS,
            api_disable: false,
            threads: 2,
            group_code: "".to_string(),
            node_ipv4: "".to_string(),
            prefix: 24,
            node_ipv6: None,
            prefix_v6: 96,
            tun_name: None,
            encrypt: None,
            algorithm: DEFAULT_ALGORITHM.to_string(),
            port: LISTEN_PORT,
            peer: None,
            bind_dev_name: None,
            exit_node: None,
            udp_stun: default_udp_stun(),
            tcp_stun: default_tcp_stun(),
        }
    }
}
