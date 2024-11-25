use clap::Parser;

#[derive(Parser, Debug)]
#[command(name="node-cache", version, about, long_about = None)]
pub struct AppArgs {
    #[arg(long, default_value = "127.0.0.1:7777")]
    pub host: String,

    #[arg(long)]
    pub node: Option<String>,

    /// Should all requests be recorded
    #[arg(long, default_value = "false")]
    pub record: bool,

    // Path to the database fil
    #[arg(long)]
    pub db_file_path: String,
}
