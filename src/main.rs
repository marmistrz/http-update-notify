#[macro_use]
extern crate failure;
extern crate lettre;
extern crate lettre_email;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate clap;
extern crate toml;

mod args;
mod check;
mod mails;

use check::check_url;
use failure::{Fallible, ResultExt};
use mails::Config;
use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    if let Err(e) = run() {
        eprint!("error");
        for cause in e.iter_chain() {
            eprint!(": {}", cause);
        }
        eprintln!("");
        std::process::exit(1);
    }
}

fn get_config() -> Fallible<Config> {
    let path = env::current_dir().context("Failed to get current directory")?;
    let config_file = path.join("config.toml");
    let mut file = File::open(config_file).context("Failed to open configuration file")?;
    let mut cfgstr = String::new();
    file.read_to_string(&mut cfgstr)
        .context("Failed to read the configuration file")?;
    let mut config: Config = toml::from_str(&cfgstr).context("Failed to load configuration")?;

    if config.smtp == "" {
        let mut s = config.email.split('@').skip(1);
        let domain = s
            .next()
            .ok_or_else(|| format_err!("Invalid e-mail format: no domain"))?;
        let smtp = "smtp.".to_string() + domain;
        eprintln!("Assuming smtp server: {}", smtp);
        config.smtp = smtp
    }

    Ok(config)
}

fn run() -> Fallible<()> {
    let matches = args::get_parser().get_matches();
    let poll_interval: u64 = matches.value_of("interval").unwrap().parse()?;
    let url = matches.value_of("url").unwrap();

    let config = get_config()?;
    println!("Watching URL: {}, poll interval: {}s", url, poll_interval);
    let handle = check_url(config, url.into(), poll_interval);
    handle.join().unwrap();
    Ok(())
}
