use std::io::Write;

pub struct Context {
    pub logger: Box<dyn Write>,
}

pub fn create_cli_context() -> Context {
    Context {
        logger: Box::new(std::io::stdout()),
    }
}
