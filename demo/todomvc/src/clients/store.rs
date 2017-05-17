use uuid::Uuid;

use cargonauts::resources::Error;
use cargonauts::futures::Future;
use cargonauts::clients::{Client, Conn};
use cargonauts::redis::{Cmd, Redis, FromRedisValue};
use cargonauts::server::Service;

use serde::{Serialize, Deserialize};
use json;

pub struct RedisStore {
    conn: Conn<Redis>
}

impl Client for RedisStore {
    const CONNECTION_NAME: &'static str = "redis";
    type Connection = Redis;

    fn connect(conn: Conn<Redis>) -> RedisStore {
        RedisStore { conn }
    }
}

impl RedisStore {
    pub fn get<T: for<'de> Deserialize<'de> + 'static>(&self, id: Uuid) -> impl Future<Item = T, Error = Error> + 'static {
        let mut cmd = Cmd::new();
        cmd.arg("HGET").arg("notes").arg(id.to_string());
        self.conn.call(cmd).then(|result| {
            let val = String::from_redis_value(&result?).map_err(|_| Error)?;
            json::from_str(&val).map_err(|_| Error)
        })
    }

    pub fn get_all<T: for<'de> Deserialize<'de> + 'static>(&self) -> impl Future<Item = Vec<T>, Error = Error> + 'static {
        let mut cmd = Cmd::new();
        cmd.arg("HGETALL").arg("notes");
        self.conn.call(cmd).then(|result| {
            let val: Vec<(String, String)> = Vec::from_redis_value(&result?).map_err(|_| Error)?;
            val.into_iter().map(|(_, val)| json::from_str(&val).map_err(|_| Error))
                           .collect::<Result<Vec<T>, Error>>()
        })
    }

    pub fn save<T: Serialize>(&self, id: Uuid, item: T) -> impl Future<Item = (), Error = Error> + 'static {
        let mut cmd = Cmd::new();
        cmd.arg("HSET").arg("notes").arg(id.to_string()).arg(json::to_string(&item).unwrap());
        self.conn.call(cmd).then(|result| { result?; Ok(()) })
    }

    pub fn delete(&self, id: Uuid) -> impl Future<Item = (), Error = Error> + 'static {
        let mut cmd = Cmd::new();
        cmd.arg("HDEL").arg("notes").arg(id.to_string());
        self.conn.call(cmd).then(|result| {
            let outcome = i32::from_redis_value(&result?).map_err(|_| Error)?;
            if outcome == 1 { Ok(()) }
            else { Err(Error) }
            
        })
    }
}
