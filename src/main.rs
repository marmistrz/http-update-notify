#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
use toml;
#[macro_use]
extern crate log;
use env_logger;
use structopt::StructOpt;

mod args;
mod check;
mod mails;

use crate::{args::Opt, check::FileWatcher, mails::Config};
use actix::prelude::*;
use failure::{Fallible, ResultExt};
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
    let opt = Opt::from_args();

    let config = get_config()?;
    let config = Arc::new(config);
    info!(
        "Watching URLs: {:?}, poll interval: {}s",
        opt.urls, opt.interval
    );
    warn!("WIP only first url");
    let url = opt.urls[0].to_string();

    let config = get_config()?;
    //let config = Arc::new(config);
    /*info!(
        "Watching URLs: {:?}, poll interval: {}s",
        urls, poll_interval
    );*/
    //check_urls(&config, urls, poll_interval);

    System::run(move || {
        // start new actor
        // TODO handle all urls
        let _addr = FileWatcher::new(url, config, opt.interval).start();
    });

    Ok(())
}
