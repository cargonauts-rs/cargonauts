macro_rules! server {
    (for $test:ident use $example:expr, $port:expr ;
        $( $method:ident $endpoint:expr => {
            responds $body:expr;
            status $status:ident;
        } )*
    ) => {
        #[test]
        fn $test() {
            let mut child = serve($example, $port);
            let client = reqwest::Client::new().unwrap();
            $( request(&client,
                $port,
                reqwest::Method::$method,
                $endpoint,
                $body,
                reqwest::StatusCode::$status,
            ); )*
            child.kill().unwrap();
        }
    };
}

server! {
    for test_echo use "echo", 7881;

    Get "echo/test" => {
        responds "Echo { echo: \"test\" }";
        status Ok;
    }

    Get "echo/something" => {
        responds "Echo { echo: \"something\" }";
        status Ok;
    }

    Get "echo/test/all-caps" => {
        responds "Echo { echo: \"TEST\" }";
        status Ok;
    }

    Get "not-echo/whatever" => {
        responds "";
        status NotFound;
    }
}

server! {
    for test_setup use "setup", 7882;

    Get "my-resource/foo" => {
        responds "MyResource { slug: \"foo\" }";
        status Ok;
    }

    Get "my-resource/bar" => {
        responds "MyResource { slug: \"bar\" }";
        status Ok;
    }

    Get "my-resource/baz" => {
        responds "MyResource { slug: \"foo\" }";
        status Ok;
    }

    Get "my-resource/anything_else" => {
        responds ();
        status BadRequest;
    }
}

// Helper functions

extern crate reqwest;

use std::process;
use std::io::{Read, BufRead, BufReader};

fn serve(example: &'static str, port: u16) -> process::Child {
    // Spawn a server running `example`
    let mut cmd = process::Command::new("cargo");
    let mut child = cmd.arg("run").arg("--example").arg(example)
                       .env("PORT", port.to_string())
                       .stderr(process::Stdio::piped())
                       .spawn().unwrap();

    // Wait to receiving the `Running` message and then return
    let mut stderr = BufReader::new(child.stderr.take().unwrap());
    let mut buf = String::new();
    loop {
        stderr.read_line(&mut buf).unwrap();
        if buf.trim().starts_with("Running") {
            break
        } {
            buf.clear();
        }
    }

    child
}

fn request<T: CheckBody>(
    client: &reqwest::Client,
    port: u16,
    method: reqwest::Method,
    endpoint: &'static str,
    body: T,
    status: reqwest::StatusCode,
) {
    let mut ctr = 10;
    let response = loop {
        match client.request(method.clone(), &format!("http://127.0.0.1:{}/{}", port, endpoint)).send() {
            Ok(response)    => break response,
            _               => { ctr -= 1; if ctr == 0 { panic!() } }
        };
    };

    assert_eq!(response.status(), &status);
    body.check_body(response);
}

trait CheckBody {
    fn check_body(self, response: reqwest::Response);
}

impl CheckBody for &'static str {
    fn check_body(self, mut response: reqwest::Response) {
        let mut buf = String::new();
        response.read_to_string(&mut buf).unwrap();
        assert_eq!(self, &buf[..]);
    }
}

// () ignores the body
impl CheckBody for () {
    fn check_body(self, _: reqwest::Response) { }
}
