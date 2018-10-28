use failure::Fallible;
use mails::{Config, MailNotificationBuilder};
use reqwest::Client;
use std::thread;
use std::time::Duration;

fn get_last_modified(client: &Client, url: &str) -> Fallible<String> {
    let head = client.head(url).send()?;
    let date = head
        .headers()
        .get("Last-Modified")
        .expect("File doesn't support Last-Modified")
        .to_str()?
        .to_string();
    Ok(date)
}

fn check_url_internal(config: &Config, url: &str) -> Fallible<()> {
    let client = Client::new();
    let mut init_date = get_last_modified(&client, &url)?;
    loop {
        thread::sleep(Duration::from_secs(1800));
        let date = get_last_modified(&client, &url)?;
        if date != init_date {
            init_date = date;
            let s = MailNotificationBuilder { url: &url };
            s.send(&config)?;
            eprintln!("E-mail sent!");
        }
    }
}

pub fn check_url(config: Config, url: String) -> thread::JoinHandle<()> {
    thread::spawn(move || check_url_internal(&config, &url).expect("an error happened"))
}
