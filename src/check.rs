use failure::Fallible;
use mails::{Config, MailNotificationBuilder};
use reqwest::Client;
use std::thread;
use std::time::Duration;

fn get_last_modified(client: &Client, url: &str) -> Fallible<String> {
    info!("Polling URL: {}", url);
    let head = client.head(url).send()?;
    let date = head
        .headers()
        .get("Last-Modified")
        .expect("File doesn't support Last-Modified")
        .to_str()?
        .to_string();
    Ok(date)
}

fn check_url_internal(config: &Config, url: &str, poll_interval: u64) -> Fallible<()> {
    let client = Client::new();
    let mut init_date = get_last_modified(&client, &url)?;
    loop {
        thread::sleep(Duration::from_secs(poll_interval));
        let date = get_last_modified(&client, &url)?;
        if date != init_date {
            info!("The file was updated, sending a notification.");
            init_date = date;
            let s = MailNotificationBuilder {
                url: &url,
                last_modified: &init_date,
            };
            s.send(&config)?;
            info!("E-mail sent!");
        }
    }
}

pub fn check_url(config: Config, url: String, poll_interval: u64) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        check_url_internal(&config, &url, poll_interval).expect("an error happened")
    })
}
