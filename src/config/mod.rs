use std::path::PathBuf;
use std::error::Error;

use toml::{self,Value,ParserError};

#[cfg(test)]
mod tests;

static DEFAULTS: Options = Options {
    debug: false,
    roots: None,
    port: 3000

};

pub type ParseResult = Result<Options,String>;

pub struct Options {
    pub debug: bool,
    pub roots: Option<Vec<PathBuf>>,
    pub port: i64
}

pub fn parse_toml(config: &str) -> ParseResult {

    let result = config
        .parse()
        .map_err(|errs: Vec<ParserError>| errs.iter().fold(
            String::new(),
            |mut a: String, ref e: &ParserError| {
                a.push_str(e.description());
                a.push('\n');
                a
            })
        );

    let value: Value = try!(result);

    Ok(Options {
        debug: value
            .lookup("logging.debug")
            .and_then(|ref v: &Value| v.as_bool())
            .unwrap_or(DEFAULTS.debug.clone()),
        port: value
            .lookup("port")
            .and_then(|ref v: &Value| v.as_integer())
            .unwrap_or(DEFAULTS.port.clone()),
        roots: DEFAULTS.roots.clone()
    })
}
