use clap::Parser;
///非常好clap写的命令行参数读取程序
#[derive(Parser, Debug)]
#[command(author="Mineral", version, about, long_about = None)]
struct Args {
    /// 这是名字
    #[arg(short, long)]
    name: String,

    /// 这是一串file的名字
    #[arg(short, long, num_args = 1..)]
    files: Vec<String>,

    /// 这是一个debug选项 
    #[arg(short, long)] 
    debug:bool,

    /// 这是一个带默认参数的count
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}
fn main() {
    let args = Args::parse();
    println!("得到的arguments 结构体是:\n {:?}",args);
}