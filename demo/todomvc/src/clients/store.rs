use uuid::Uuid;

use cargonauts::resources::Error;
use cargonauts::futures::Future;
use cargonauts::clients::{Client, ConnectClient, Conn, NewServiceLike};
use cargonauts::redis::{Cmd, Redis, FromRedisValue};
use cargonauts::server::{Service, StatusCode};

use serde::{Serialize, Deserialize};
use json;

pub struct RedisStore<C: NewServiceLike<Redis> = Redis> {
    conn: Conn<C>,
}

impl<C: NewServiceLike<Redis>> Client for RedisStore<C> {
    const CONNECTION_NAME: &'static str = "redis";
    type Connection = Redis;
}

impl<C: NewServiceLike<Redis>> ConnectClient<C> for RedisStore<C> {
    fn connect(conn: Conn<C>) -> RedisStore<C> {
        RedisStore { conn }
    }
}

impl<C: NewServiceLike<Redis>> RedisStore<C> {
    pub fn get<T: for<'de> Deserialize<'de> + 'static>(&self, id: Uuid) -> impl Future<Item = T, Error = Error> + 'static {
        let mut cmd = Cmd::new();
        cmd.arg("HGET").arg("notes").arg(id.to_string());
        self.conn.call(cmd).then(|result| {
            let val = String::from_redis_value(&result?).map_err(|e| Error::from_err(e, StatusCode::InternalServerError))?;
            json::from_str(&val).map_err(|e| Error::from_err(e, StatusCode::InternalServerError))
        })
    }

    pub fn get_all<T: for<'de> Deserialize<'de> + 'static>(&self) -> impl Future<Item = Vec<T>, Error = Error> + 'static {
        let mut cmd = Cmd::new();
        cmd.arg("HGETALL").arg("notes");
        self.conn.call(cmd).then(|result| {
            let val: Vec<(String, String)> = Vec::from_redis_value(&result?).map_err(|e| Error::from_err(e, StatusCode::InternalServerError))?;
            val.into_iter().map(|(_, val)| json::from_str(&val).map_err(|e| Error::from_err(e, StatusCode::InternalServerError)))
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
        self.conn.call(cmd).then(move |result| {
            let outcome = i32::from_redis_value(&result?).map_err(|e| Error::from_err(e, StatusCode::InternalServerError))?;
            if outcome == 1 { Ok(()) }
            else { Err(Error::with_msg(StatusCode::BadRequest, format!("No such resource: {}", id))) }
            
        })
    }
}
