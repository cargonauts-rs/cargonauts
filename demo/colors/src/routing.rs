// This is your routing file, which contains the `routes!` macro that defines
// the surface API for your application.
//
// Every time you add a new endpoint, you will need to modify this file to
// declare how it will be used.
use cargonauts::methods::*;
use cargonauts::formats::*;

use methods::*;
use resources::*;

routes! {
    resource Square {
        method Get in Handlebars;
        method Random in Handlebars;
    }
}
