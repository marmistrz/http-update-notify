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
#[macro_use]
extern crate log;
extern crate env_logger;

mod args;
mod check;
mod mails;

use crate::check::check_urls;
use failure::{Fallible, ResultExt};
use crate::mails::Config;
use std::{env, fs::File, io::Read, sync::Arc};

fn main() {
    init_logger();
    if let Err(e) = run() {
        eprint!("error");
        for cause in e.iter_chain() {
            eprint!(": {}", cause);
        }
        eprintln!("");
        std::process::exit(1);
    }
}

fn init_logger() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "hun=info")
    }
    env_logger::init();
}

fn get_config() -> Fallible<Config> {
    let path = env::current_dir().context("Failed to get current directory")?;
    let config_name = "hun.toml";
    let config_file = path.join(config_name);
    let mut file = File::open(config_file).context(format!(
        "Failed to open configuration file: {}",
        config_name
    ))?;
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
        info!("Assuming smtp server: {}", smtp);
        config.smtp = smtp
    }

    Ok(config)
}

fn run() -> Fallible<()> {
    let matches = args::get_parser().get_matches();
    let poll_interval: u64 = matches.value_of("interval").unwrap().parse()?;
    let urls: Vec<_> = matches.values_of("url").unwrap().collect();

    let config = get_config()?;
    let config = Arc::new(config);
    info!(
        "Watching URLs: {:?}, poll interval: {}s",
        urls, poll_interval
    );
    check_urls(&config, urls, poll_interval);
    Ok(())
}
