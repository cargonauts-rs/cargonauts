use std::net::SocketAddr;

use core::reactor::Handle;
use tokio::NewService;
use c3po::Conn;
use serde::de::DeserializeOwned;

pub trait Configure<Args>: Sized {
    type Config: Default + DeserializeOwned;
    fn config(cfg: Self::Config, args: Args) -> Self;
}

pub trait Client: 'static {
    const CONNECTION_NAME: &'static str;
    type Connection: Configure<Handle> + NewService + 'static;
    fn connect(conn: Conn<Self::Connection>) -> Self;
}

use redis::Redis;

#[derive(Deserialize)]
pub struct RedisConfig {
    #[serde(default = "redis_socket_addr")]
    pub address: SocketAddr,
}

impl Default for RedisConfig {
    fn default() -> RedisConfig {
        RedisConfig { address: redis_socket_addr() }
    }
}

fn redis_socket_addr() -> SocketAddr {
    "127.0.0.1:6379".parse().unwrap()
}

impl Configure<Handle> for Redis {
    type Config = RedisConfig;
    fn config(cfg: RedisConfig, handle: Handle) -> Redis {
        Redis::new(cfg.address, handle)
    }
}
