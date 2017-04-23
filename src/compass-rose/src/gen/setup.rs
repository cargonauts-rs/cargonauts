use cfg::CargonautsConfig;
use heck::KebabCase;
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
            // TODO must separate out member cfg from pool cfg
            let config = json::to_string(cfg).unwrap();
            quote!(::cargonauts::json::from_str(#config).unwrap())
        }
        None        => quote!(::cargonauts::server::pool::Config::default()),
    }
}

fn member_cfg(conn: &str, service: &Tokens, config: Option<&CargonautsConfig>) -> Tokens {
    match config.and_then(|cfg| cfg.client_cfg(conn)) {
        Some(cfg)   => {
            // TODO must separate out member cfg from pool cfg
            let config = json::to_string(cfg).unwrap();
            quote!(::cargonauts::json::from_str(#config).unwrap())
        }
        None        => {
            quote!(<#service as ::cargonauts::server::pool::Configure>::Config::default())
        }
    }
}
