use std::fmt;
use std::str::FromStr;

use cargonauts::futures::{future, Future};
use cargonauts::*;
use cargonauts::methods::*;
use cargonauts::server::StatusCode;

use rand::random;
use serde::Serializer;

use methods::Random;

#[derive(Serialize)]
pub struct Square {
    #[serde(serialize_with = "color_string")]
    color: Color,
}

impl Resource for Square {
    type Identifier = Color;
}

impl Get for Square {
    fn get(color: Color, _: Environment) -> Box<Future<Item = Self, Error = Error>> {
        Box::new(future::ok(Square { color }))
    }
}

impl Random for Square {
    fn random(env: Environment) -> Box<Future<Item = Self, Error = Error>> {
        let color = Color(random(), random(), random());
        Square::get(color, env)
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Color(u8, u8, u8);

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02x}{:02x}{:02x}", self.0, self.1, self.2)
    }
}

impl FromStr for Color {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn hex(s: &str) -> Result<u8, Error> {
            u8::from_str_radix(s, 16).map_err(|e| {
                Error::with_msg(StatusCode::BadRequest, format!("invalid color: {}", e))
            })
        }

        if s.len() != 6 {
            return Err(Error::with_msg(StatusCode::BadRequest, "invalid color"))
        }

        let r = hex(&s[0..2])?;
        let g = hex(&s[2..4])?;
        let b = hex(&s[4..6])?;
        Ok(Color(r, g, b))
    }
}

fn color_string<S: Serializer>(color: &Color, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&color.to_string())
}
