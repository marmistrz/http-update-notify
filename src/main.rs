#[macro_use]
extern crate failure;
extern crate lettre;
extern crate lettre_email;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod mails;

use failure::{Fallible, ResultExt};
use mails::{Config, MailNotificationBuilder};
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
            .ok_or(format_err!("Invalid e-mail format: no domain"))?;
        let smtp = "smtp.".to_string() + domain;
        eprintln!("Assuming smtp server: {}", smtp);
        config.smtp = smtp
    }

    Ok(config)
}

fn run() -> Fallible<()> {
    let config = get_config()?;
    let s = MailNotificationBuilder {
        url: "abcd".to_string(),
    };
    s.send(&config)?;
    eprintln!("E-mail sent!");
    Ok(())
}
