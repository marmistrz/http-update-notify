//#[macro_use]
extern crate failure;
extern crate lettre;
extern crate lettre_email;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod mails;

use failure::Fallible;

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

fn run() -> Fallible<()> {
    Ok(())
}
