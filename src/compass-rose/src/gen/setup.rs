use std::collections::BTreeMap;

use cfg::{CargonautsConfig, Value};
use heck::{KebabCase, SnekCase};
use quote::{Tokens, Ident};
use json;

use ast::*;

pub fn setup(setup: &Setup, config: Option<&CargonautsConfig>) -> Tokens {
    let conns: Vec<_> = setup.members.iter().map(|setup| match *setup {
        SetupMember::Connection(ref c) => conn(c, config),
    }).collect();

    if conns.is_empty() {
        quote! {
            ::cargonauts::futures::future::ok(::cargonauts::routing::EnvBuilder::new().build())
        }
    } else {
        quote!({
            let env_b = ::cargonauts::routing::EnvBuilder::new();
            let mut vec = vec![];
            #({ let env_b = env_b.clone(); vec.push(#conns) })*
            ::cargonauts::futures::stream::futures_unordered(vec).for_each(|_| Ok(()))
                .map(|_| env_b.build())
        })
    }
}

fn conn(conn: &Connection, config: Option<&CargonautsConfig>) -> Tokens {
    let ident = Ident::new(&conn.conn[..]);
    let conn = conn.conn.to_kebab_case();
    let cfg = pool_cfg(&conn, config);
    let member_cfg = member_cfg(&conn, &quote!(#ident), config);
    quote!({env_b.new_pool::<#ident>(handle.clone(), #cfg, #member_cfg)})
}

fn pool_cfg(conn: &str, config: Option<&CargonautsConfig>) -> Tokens {
    match config.and_then(|cfg| cfg.client_cfg(conn)) {
        Some(cfg)   => {
            let (cfg, _) = split(cfg);
            let config = json::to_string(&cfg).unwrap();
            quote!(::cargonauts::json::from_str(#config).unwrap())
        }
        None        => quote!(::cargonauts::server::pool::Config::default()),
    }
}

fn member_cfg(conn: &str, service: &Tokens, config: Option<&CargonautsConfig>) -> Tokens {
    match config.and_then(|cfg| cfg.client_cfg(conn)) {
        Some(cfg)   => {
            let (_, cfg) = split(cfg);
            let config = json::to_string(&cfg).unwrap();
            quote!(::cargonauts::json::from_str(#config).unwrap())
        }
        None        => {
            quote!(<#service as ::cargonauts::server::pool::Configure<::cargonauts::server::Handle>>::Config::default())
        }
    }
}

fn split(cfg: &BTreeMap<String, Value>) -> (BTreeMap<String, Value>, BTreeMap<String, Value>) {
    let mut pool = BTreeMap::new();
    let mut member = BTreeMap::new();
    for (key, val) in cfg {
        match &key[..] {
            "min-connections"       => pool.insert(String::from("min_connections"), val.clone()),
            "max-connections"       => pool.insert(String::from("max_connections"), val.clone()),
            "min-idle-connections"  => pool.insert(String::from("min_idle_connections"), val.clone()),
            "max-idle-connections"  => pool.insert(String::from("min_idle_connections"), val.clone()),
            "connect-timeout"       => pool.insert(String::from("connect_timeout"), val.clone()),
            "max-live-time"         => pool.insert(String::from("max_live_time"), val.clone()),
            "max-idle-time"         => pool.insert(String::from("max_idle_time"), val.clone()),
            "reap-frequency"        => pool.insert(String::from("reap_frequency"), val.clone()),
            _                       => member.insert(key.to_snek_case(), val.clone()),
        };
    }
    (pool, member)
}
