use toml;
use std::{io, fmt, string};

use crate::show;

#[derive(Debug)]
pub enum Kind {
    InvalidOption,
    OptionsTooSimilar
}

struct Error {
    kind: Kind,
    message: String,
    exit_code: i32
}

impl fmt::Display for Error {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!( f, "{}", self.message )
    }
}

impl fmt::Debug for Error {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        let debug_message = format!(
            "SchedulerError {{ kind: {:?}, message: {}, exit_code: {} }}",
            self.kind,
            self.message,
            self.exit_code
        );

        write!( f, "{}", debug_message )
    }
}

pub fn raise<T>( kind: Kind, subject: T ) -> !
where T: fmt::Display {
    let message = match kind {
        Kind::InvalidOption => format!( "Option `'{}'` was not recognized, try `'--help'` to see all options", subject ),
        Kind::OptionsTooSimilar => format!( "There are too many options begining with `'{}'`, can't assign a short name", subject )
    };

    let error = Error {
        kind,
        message,
        exit_code: 1
    };


    Err(error).handle()
}

pub trait Handler<T> {
    fn handle( self ) -> T;
}

impl<T> Handler<T> for Result<T, io::Error> {
    fn handle( self ) -> T {
        let result: T = match self {
            Ok(ok) => ok,
            Err(err) => {
                show::error( &err.to_string() )
            }
        };

        result
    }
}

impl<T> Handler<T> for Result<T, Error> {
    fn handle( self ) -> T {
        let result: T = match self {
            Ok(ok) => ok,
            Err(err) => {
                show::error( &err.to_string() )
            }
        };

        result
    }
}

impl<T> Handler<T> for Result<T, toml::de::Error> {
    fn handle( self ) -> T {
        let result: T = match self {
            Ok(ok) => ok,
            Err(err) => {
                show::error( &err.to_string() )
            }
        };

        result
    }
}

impl<T> Handler<T> for Result<T, serde_json::Error> {
    fn handle( self ) -> T {
        let result: T = match self {
            Ok(ok) => ok,
            Err(err) => {
                let mut error_string = err.to_string().replace("field", "required option");
                error_string = error_string.split("at line").nth(0).unwrap().to_string();

                show::error( &error_string )
            }
        };

        result
    }
}

impl<T> Handler<T> for Result<T, string::FromUtf8Error> {
    fn handle( self ) -> T {
        let result: T = match self {
            Ok(ok) => ok,
            Err(err) => {
                show::error( &err.to_string() )
            }
        };

        result
    }
}