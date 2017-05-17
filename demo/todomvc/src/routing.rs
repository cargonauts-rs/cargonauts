use cargonauts::methods::{Index, Post, Patch, Delete};
use cargonauts::formats::JsonApi;
use cargonauts::redis::Redis;

use resources::Note;
use middleware::Logging;

routes! {
    setup {
        connection to Redis;
    }

    mod api {
        resource Note as "notes" {
            #[middleware(Logging)]
            method Index, Post, Patch, Delete in JsonApi;
        }
    }
}
