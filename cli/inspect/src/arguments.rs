use clap::Parser;

#[derive(Parser)]
#[command(
    name = "cognexus-inspect",
    about = "Inspect Cognexus WASM plugins",
    long_about = None)]
pub(crate) struct Arguments {
    #[arg(value_name = "FILE")]
    pub(crate) wasm_file: String,

    #[arg(
        long,
        value_name = "KIND",
        default_value = "types",
        help = "Kind of plugin: types or nodes"
    )]
    pub(crate) kind: String,
}
