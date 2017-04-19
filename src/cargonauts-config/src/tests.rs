use CargonautsConfig;

#[test]
fn toml_ctor() {
    assert!(CargonautsConfig::from_toml("[package.metadata.cargonauts]").is_ok())
}

#[test]
fn with_clients() {
    const TOML: &'static str = "[package.metadata.cargonauts.clients.foo]\nbar = 0";

    let cfg = CargonautsConfig::from_toml(TOML).unwrap();
    assert!(cfg.client_cfg("foo").is_some());
}
