use mainsail::Resource;
use mime::Mime;

use http;
use present::Present;
use receive::Receive;

pub trait Format<T: Resource> {
    type Receiver: Receive<T>;
    type Presenter: Present<T>;
    const MIME_TYPES: &'static [&'static str];

    fn accepted_by(req: http::Request) -> bool {
        Self::MIME_TYPES.iter().any(|mime| req.accepts(&mime.parse().unwrap()))
    }
}

trait Accepts {
    fn accepts(&self, offered: &Mime) -> bool;
}

impl Accepts for Mime {
    fn accepts(&self, offered: &Mime) -> bool {
        // TODO actually check 'accepts'
        true
    }
}

impl Accepts for http::Request {
    fn accepts(&self, offered: &Mime) -> bool {
        self.headers().get::<http::headers::Accept>().map_or(true, |accept| {
            accept.0.iter().any(|q| q.item.accepts(offered))
        })
    }
}
