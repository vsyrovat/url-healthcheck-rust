use ::clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    pub delay: usize,
    pub url: String,
}
