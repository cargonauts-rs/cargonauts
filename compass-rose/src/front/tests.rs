use front::Routes;

#[test]
fn valid_simple() {
    assert!(Routes::new("resource Foo {}").is_ok());
}

#[test]
fn valid_multiple() {
    assert!(Routes::new("resource Foo { } resource Bar { }").is_ok())
}

#[test]
fn valid_has_one() {
    assert!(Routes::new("resource Foo { has one Bar; }").is_ok());
}

#[test]
fn valid_has_many() {
    assert!(Routes::new("resource Foo { has many Bar; }").is_ok());
}

#[test]
fn valid_multiple_rels() {
    assert!(Routes::new("resource Foo { has many Bar; has one Baz; }").is_ok());
}

#[test]
fn valid_alias_get() {
    assert!(Routes::new("resource Foo { alias get as foobar; }").is_ok());
}

#[test]
fn valid_alias_index() {
    assert!(Routes::new("resource Foo { alias index as foobar; }").is_ok());
}

#[test]
fn valid_multiple_alias() {
    assert!(Routes::new("resource Foo { alias get as barfoo; alias index as foobar; }").is_ok());
}
