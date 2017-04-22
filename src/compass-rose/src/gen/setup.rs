use cfg::CargonautsConfig;
use heck::KebabCase;
use quote::{Tokens, Ident};

use ast::*;

pub fn setup(setup: &Setup, config: Option<&CargonautsConfig>) -> Tokens {
    let clients: Vec<_> = setup.members.iter().filter_map(SetupMember::as_client)
                               .map(|c| client(c, config)).collect();

    if clients.is_empty() {
        quote! {
            ::cargonauts::futures::future::ok(::cargonauts::routing::EnvBuilder::new().build())
        }
    } else {
        quote!({
            let env_b = ::cargonauts::routing::EnvBuilder::new();
            let mut vec = vec![];
            #({ let env_b = env_b.clone(); vec.push(#clients) })*
            ::cargonauts::futures::stream::futures_unordered(vec).for_each(|_| Ok(()))
                .map(|_| env_b.build())
        })
    }
}

fn client(client: &Client, config: Option<&CargonautsConfig>) -> Tokens {
    let service = match client.wrapper {
        Some(ref wrapper)   => {
            let ident = Ident::new(&wrapper[..]);
            quote!(::cargonauts::routing::ClientConnector<#ident>)
        }
        None                => {
            let ident = Ident::new(&client.conn[..]);
            quote!(#ident)
        }
    };
    let client = client.wrapper.as_ref().unwrap_or(&client.conn).to_kebab_case();
    let cfg = pool_cfg(&client, config);
    let client_cfg = client_cfg(&client, &service, config);
    quote!({env_b.new_pool::<#service>(handle.clone(), #cfg, #client_cfg)})
}

fn pool_cfg(client: &str, config: Option<&CargonautsConfig>) -> Tokens {
    match config.and_then(|cfg| cfg.client_cfg(client)) {
        Some(cfg)   => panic!(),
        None        => quote!(::cargonauts::server::pool::Config::default()),
    }
}

fn client_cfg(client: &str, service: &Tokens, config: Option<&CargonautsConfig>) -> Tokens {
    match config.and_then(|cfg| cfg.client_cfg(client)) {
        Some(cfg)   => panic!(),
        None        => {
            quote!(<#service as ::cargonauts::server::pool::Configure>::Config::default())
        }
    }
}
