use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[clap(subcommand)]
    sub: SubCommand,
}
#[derive(Parser, Debug)]
enum SubCommand {
    /// Encode vax status into vax number
    Encode {
        #[clap(name = "VAX_STATUS")]
        s: String,
    },
    /// Decode vax number into vax status
    Decode {
        #[clap(name = "VAX_NUMBER")]
        v: u64,
    },
}
fn main() {
    let args = Args::parse();
    match args.sub {
        SubCommand::Encode { s } => {
            let s: Vec<char> = s.chars().collect();
            let v = vax_number::encode(&s);
            println!("{v}");
        }
        SubCommand::Decode { v } => {
            let s = vax_number::decode(v);
            let s: String = s.iter().collect();
            println!("{s}");
        }
    }
}
