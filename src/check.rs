use crate::mails::{Config, MailNotificationBuilder};
use actix::prelude::*;
use actix_web::{client, client::ClientResponse, HttpMessage};
use failure::{err_msg, Fallible, ResultExt};
use futures::future::Future;
use reqwest::Client;
use std::{sync::Arc, thread, time::Duration};


/// Actor
pub struct FileWatcher {
    urls: String,
    config: Config,
    interval: u64,
    // todo init in new
    last_modified: Option<String>,
}

impl FileWatcher {
    pub fn new(urls: String, config: Config, interval: u64) -> Self {
        Self {
            urls,
            config,
            interval,
            last_modified: None,
        }
    }

    fn check(&mut self) {
        let future = get_last_modified(&self.urls).map(|s| {
            info!("Last-modified is: {}", s);
        });
        Arbiter::spawn(future);
        //.map_err(|e| error!("Error polling {}: {}", &self.urls, e.)); // FIXME error handling;

    }
}

/// Declare actor and its context
impl Actor for FileWatcher {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        // add stream
        ctx.run_interval(Duration::from_secs(1), |act, _ctx| {
            println!("Hello, {}!", act.urls);
            act.check();
        });
    }
}

fn get_last_modified(url: &str) -> impl Future<Item = String, Error = ()> {
    info!("Polling URL: {}", url);
    client::head(url)
        .finish()
        .expect("Error building the HEAD request") // FIXME error handling
        .send()
        .map_err(|e| error!("Error polling: {}", e)) // FIXME error handling - we should move it out of this function or print the url
        .and_then(|resp: ClientResponse| {
            if let Some(last_mod) = resp.headers().get("last-modified") {
                let s = last_mod.to_str().expect("to_str failed").to_string();
                Ok(s)
            } else {
                error!("The URL doesn't support last-modified");
                Err(())
            }
        })
}

/*fn check_url_internal(config: &Arc<Config>, url: &str, poll_interval: u64) -> Fallible<()> {
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
}*/
