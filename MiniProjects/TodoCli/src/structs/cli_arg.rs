use clap::Parser;

#[derive(Parser)]
pub struct CliArg {
    pub command: String,
    pub arg: Option<String>
}