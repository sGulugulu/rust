use std::path::PathBuf;
use clap::Parser;

#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// 隐式的 Action Set 
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Action Count 
    #[arg(short, long, action = clap::ArgAction::Count)]//
    debug: u8,

    /// Action Set 
    #[arg(short, long)]//, action = clap::ArgAction::Set
    set: String,

    /// Action Append
    #[arg(short, long)] //,action = clap::ArgAction::Append
    files: Vec<String>,

}
fn main() {
    let cli = Cli::parse();
    println!("{:?}",cli);
}