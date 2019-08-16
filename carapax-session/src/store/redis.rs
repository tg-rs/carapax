use crate::{
    session::{SessionKey, SessionLifetime},
    store::{
        data::{Data, DataRef},
        SessionStore,
    },
};
use failure::Error;
use futures::{
    future::{self, Either},
    Future,
};
use redis::{r#async::SharedConnection, Client, Cmd, FromRedisValue};
use serde::{de::DeserializeOwned, Serialize};

/// Redis powered session store
///
/// Serialization and deserialization of input/output values implemented using serde_json
#[derive(Clone)]
pub struct RedisSessionStore {
    conn: SharedConnection,
    namespace: String,
    lifetime: SessionLifetime,
}

impl RedisSessionStore {
    /// Use this method to create a new store
    ///
    /// # Arguments
    ///
    /// - params - Redis URL (`redis://[:<passwd>@]<hostname>[:port][/<db>]`)
    /// - namespace - A prefix string for keys
    pub fn open<P: AsRef<str>, N: Into<String>>(
        params: P,
        namespace: N,
    ) -> impl Future<Item = RedisSessionStore, Error = Error> {
        future::result(Client::open(params.as_ref()))
            .from_err()
            .and_then(|client| {
                client
                    .get_shared_async_connection()
                    .from_err()
                    .map(|conn| RedisSessionStore {
                        conn,
                        namespace: namespace.into(),
                        lifetime: SessionLifetime::default(),
                    })
            })
    }

    /// Sets session lifetime
    ///
    /// This store does not implement GarbageCollector but,
    /// thanks to redis, you still can set session lifetime
    pub fn with_lifetime<L>(mut self, lifetime: L) -> Self
    where
        L: Into<SessionLifetime>,
    {
        self.lifetime = lifetime.into();
        self
    }

    fn format_namespace(&self, key: &SessionKey) -> String {
        format!("{}-{}", self.namespace, key.namespace())
    }

    fn query<V>(&self, cmd: Cmd) -> Box<dyn Future<Item = V, Error = Error> + Send>
    where
        V: FromRedisValue + Send + 'static,
    {
        Box::new(cmd.query_async(self.conn.clone()).from_err().map(|(_conn, v)| v))
    }
}

impl SessionStore for RedisSessionStore {
    fn get<O>(&self, key: SessionKey) -> Box<dyn Future<Item = Option<O>, Error = Error> + Send>
    where
        O: DeserializeOwned + Send + 'static,
    {
        let mut cmd = redis::cmd("HGET");
        cmd.arg(self.format_namespace(&key));
        cmd.arg(key.name());
        Box::new(self.query::<Option<String>>(cmd).and_then(|val| {
            Ok(match val {
                Some(val) => {
                    let data: Data = serde_json::from_str(&val)?;
                    if data.is_expired()? {
                        None
                    } else {
                        Some(data.parse_value()?)
                    }
                }
                None => None,
            })
        }))
    }

    fn set<I>(&self, key: SessionKey, val: &I) -> Box<dyn Future<Item = (), Error = Error> + Send>
    where
        I: Serialize,
    {
        match serde_json::to_string(&DataRef::new(val)) {
            Ok(val) => {
                let namespace = self.format_namespace(&key);
                let mut hlen_cmd = redis::cmd("HLEN");
                hlen_cmd.arg(&namespace);
                let lifetime = self.lifetime;
                Box::new(hlen_cmd.query_async(self.conn.clone()).from_err().and_then(
                    move |(conn, hlen_val): (SharedConnection, i64)| {
                        let mut hset_cmd = redis::cmd("HSET");
                        hset_cmd.arg(&namespace);
                        hset_cmd.arg(key.name());
                        hset_cmd.arg(val);
                        hset_cmd.query_async(conn).from_err().and_then(move |(conn, ())| {
                            let duration = if hlen_val == 0 {
                                match lifetime {
                                    SessionLifetime::Forever => None,
                                    SessionLifetime::Duration(duration) => Some(duration.as_secs()),
                                }
                            } else {
                                None
                            };
                            duration
                                .map(|duration| {
                                    let mut expire_cmd = redis::cmd("EXPIRE");
                                    expire_cmd.arg(&namespace);
                                    expire_cmd.arg(duration);
                                    Either::A(
                                        expire_cmd
                                            .query_async(conn)
                                            .from_err()
                                            .map(|(_conn, _n): (SharedConnection, i64)| ()),
                                    )
                                })
                                .unwrap_or_else(|| Either::B(future::ok(())))
                        })
                    },
                ))
            }
            Err(err) => Box::new(future::err(err.into())),
        }
    }

    fn expire(&self, key: SessionKey, seconds: usize) -> Box<dyn Future<Item = (), Error = Error> + Send> {
        let mut hget_cmd = redis::cmd("HGET");
        let namespace = self.format_namespace(&key);
        hget_cmd.arg(namespace.clone());
        hget_cmd.arg(key.name());
        Box::new(hget_cmd.query_async(self.conn.clone()).from_err().and_then(
            move |(conn, val): (SharedConnection, Option<String>)| {
                match val {
                    Some(val) => Either::A(
                        future::result(serde_json::from_str::<Data>(&val))
                            .from_err()
                            .and_then(move |mut data| data.set_lifetime(seconds as u64).map(|()| data))
                            .and_then(|data| serde_json::to_string(&data).map_err(Error::from))
                            .and_then(move |val| {
                                let mut hset_cmd = redis::cmd("HSET");
                                hset_cmd.arg(&namespace);
                                hset_cmd.arg(key.name());
                                hset_cmd.arg(val);
                                hset_cmd.query_async(conn).from_err()
                            })
                            .and_then(|(_conn, ())| Ok(())),
                    ),
                    None => Either::B(future::ok(())),
                }
            },
        ))
    }

    fn del(&self, key: SessionKey) -> Box<dyn Future<Item = (), Error = Error> + Send> {
        let mut cmd = redis::cmd("HDEL");
        cmd.arg(self.format_namespace(&key));
        cmd.arg(key.name());
        self.query(cmd)
    }
}
