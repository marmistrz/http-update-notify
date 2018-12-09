/*extern crate actix;
extern crate futures;
extern crate tokio;
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

use actix::prelude::*;
use futures::Future;
use std::time::Duration;

/// Actor
struct WatchedFile {
    url: String,
}

mod args;
mod check;
mod mails;

/// Declare actor and its context
impl Actor for WatchedFile {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        // add stream
        let addr = ctx.address();
        ctx.run_interval(Duration::from_secs(1), |act, _ctx| {
            println!("Hello, {}!", act.url);
        });
    }
}

fn main() {
    // start system, this is required step
    System::run(|| {
        // start new actor
        let _addr = WatchedFile { url: "foo".into() }.start();
    });
}*/

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
extern crate actix;
extern crate env_logger;

mod args;
mod check;
mod mails;

use actix::prelude::*;
use check::FileWatcher;
use failure::{Fallible, ResultExt};
use mails::Config;
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
    warn!("WIP only first url");
    let url = urls[0].to_string();

    let config = get_config()?;
    info!(
        "Watching URLs: {:?}, poll interval: {}s",
        urls, poll_interval
    );

    System::run(move || {
        // start new actor
        // TODO handle all urls
        let _addr = FileWatcher::new(url, config, poll_interval).start();
    });

    Ok(())
}
