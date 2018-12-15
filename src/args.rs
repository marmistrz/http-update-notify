use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "hun", about = "Notify about updated files on a HTTP server")]
pub struct Opt {
    #[structopt(short = "i", long = "interval", help = "Poll interval in seconds")]
    pub interval: u64,
    #[structopt(short = "u", long = "url")]
    pub urls: Vec<String>,
}
