use crate::mails::{Config, MailNotificationBuilder};
use failure::{err_msg, Fallible};
use reqwest::Client;
use std::{sync::Arc, thread, time::Duration};

fn get_last_modified(client: &Client, url: &str) -> Fallible<String> {
    info!("Polling URL: {}", url);
    let head = client.head(url).send()?;
    let date = head
        .headers()
        .get("Last-Modified")
        .ok_or_else(|| err_msg("The URL doesn't support Last-Modified"))?
        .to_str()?
        .to_string();
    Ok(date)
}

fn check_url_internal(config: &Arc<Config>, url: &str, poll_interval: u64) -> Fallible<()> {
    let client = Client::new();
    let mut init_date = get_last_modified(&client, &url)?;
    loop {
        thread::sleep(Duration::from_secs(poll_interval));
        let date = get_last_modified(&client, &url)?;
        if date != init_date {
            info!(
                "The file was updated, sending a notification. \
                 Previous modification date: {}. \
                 Current modification date: {}",
                init_date, date
            );
            init_date = date;
            let s = MailNotificationBuilder {
                url: &url,
                last_modified: &init_date,
            };
            s.send(&config)?;
            info!("E-mail sent!");
        } else {
            info!("File unchanged, last modifictaion date: {}", init_date);
        }
    }
}

pub fn check_urls(config: &Arc<Config>, urls: Vec<String>, poll_interval: u64) {
    let mut handles = vec![];
    for url in urls {
        let config = Arc::clone(config);

        let handle = thread::spawn(move || {
            check_url_internal(&config, &url, poll_interval)
                .unwrap_or_else(|err| error!("{}: {}", url, err));
        });

        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
