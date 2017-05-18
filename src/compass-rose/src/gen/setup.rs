use std::collections::BTreeMap;

use cfg::{CargonautsConfig, Value};
use heck::SnekCase;
use itertools::Itertools;
use quote::{Tokens, Ident};
use json;

use ast::*;

pub fn setup(setup: &Setup, config: &CargonautsConfig) -> Tokens {
    let conn_groups = setup.members.iter().filter_map(|m| m.as_conn()).group_by(|c| &c.conn);
    let conns = conn_groups.into_iter().map(|(proto, conns)| {
       let conns = conns.collect::<Vec<_>>(); 
       if conns.len() > 1 {
            conn_group(proto, &conns, config)
       } else {
            conn(conns[0], config)
       }
    }).collect::<Vec<_>>();

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

fn conn(conn: &Connection, config: &CargonautsConfig) -> Tokens {
    let ident = Ident::new(&conn.conn[..]);
    let cfg = pool_cfg(&conn.name, config);
    let member_cfg = member_cfg(&conn.name, &quote!(#ident), config);
    quote!({env_b.new_pool::<#ident>(handle.clone(), #cfg, #member_cfg)})
}

fn conn_group(proto: &str, conns: &[&Connection], config: &CargonautsConfig) -> Tokens {
    let ident = Ident::new(proto);
    let cfgs = cfg_group(conns, config);
    quote!({env_b.new_pool_vec::<#ident>(handle.clone(), vec!#cfgs)})
}

fn cfg_group(conns: &[&Connection], config: &CargonautsConfig) -> Vec<Tokens> {
    conns.iter().map(|conn| {
        let name = &conn.name;
        let cfg = pool_cfg(name, config);
        let ident = Ident::new(&conn.conn[..]);
        let member_cfg = member_cfg(name, &quote!(#ident), config);
        quote!{(#name, #cfg, #member_cfg)}
    }).collect()
}

fn pool_cfg(conn: &str, config: &CargonautsConfig) -> Tokens {
    match config.conn_cfg(conn) {
        Some(cfg)   => {
            let (cfg, _) = split(cfg);
            let config = json::to_string(&cfg).unwrap();
            quote!(::cargonauts::json::from_str(#config).unwrap())
        }
        None        => quote!(::cargonauts::server::pool::Config::default()),
    }
}

fn member_cfg(conn: &str, service: &Tokens, config: &CargonautsConfig) -> Tokens {
    match config.conn_cfg(conn) {
        Some(cfg)   => {
            let (_, cfg) = split(cfg);
            let config = json::to_string(&cfg).unwrap();
            quote!(::cargonauts::json::from_str(#config).unwrap())
        }
        None        => {
            quote!(<#service as ::cargonauts::server::pool::Configure>::Config::default())
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
