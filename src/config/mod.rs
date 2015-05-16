use std::path::PathBuf;

#[cfg(test)]
mod tests;

static DEFAULTS: Options = Options {
    debug: false,
    roots: None,
    port: 3000usize

};

pub type ParseResult = Result<Options,String>;

pub struct Options {
    pub debug: bool,
    pub roots: Option<Vec<PathBuf>>,
    pub port: usize
}

pub fn parse_toml(config: &str) -> ParseResult {
    unimplemented!()
}
