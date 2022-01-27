use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name="Test App", about="This application tests out the Rust structopt crate.")]
struct Cli {
    #[structopt(short="c", long="count", parse(from_occurrences))]
    count: usize,
    #[structopt(subcommand)]
    cmd: Option<CliCmd>,
}

#[derive(StructOpt, Debug)]
enum CliCmd {
    File {
        filename: String
    },
    Url {
        url: String
    },
}

fn main() {
    let args = Cli::from_args();
    println!("Hello, world");
    println!("args: {:?}", args);
    println!("count: {:?}", args.count);
    match args.cmd {
        Some(s) => match s {
            CliCmd::File { filename } => println!("filename: {:?}", filename),
            CliCmd::Url { url }=> println!("url: {:?}", url),
        },
        None => ()
    }
}
