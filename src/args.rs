use clap::{App, Arg};

const APP: &str = env!("CARGO_PKG_NAME");

pub fn get_parser<'a, 'b>() -> App<'a, 'b> {
    App::new(APP)
        .version(crate_version!())
        .author(crate_authors!())
        .about("Poor man's slurm")
        .arg(
            Arg::with_name("interval")
                .short("i")
                .long("interval")
                .required(true)
                .takes_value(true)
                .help("Set the poll interval (in seconds)"),
        )
        .arg(
            Arg::with_name("url")
                .short("u")
                .long("url")
                .required(true)
                .multiple(true)
                .takes_value(true)
                .help("The URL to poll"),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let matches =
            get_parser().get_matches_from(&["./executable", "-i", "15", "-u", "www.example.com"]);

        assert_eq!(matches.value_of("interval").unwrap(), "15");
        assert_eq!(matches.value_of("url").unwrap(), "www.example.com");
    }

    #[test]
    fn test_mutli_urls() {
        let matches = get_parser().get_matches_from(&[
            "./executable",
            "-i",
            "15",
            "-u",
            "www.xd.com",
            "-u",
            "www.example.com",
        ]);

        assert_eq!(matches.value_of("interval").unwrap(), "15");
        assert_eq!(
            matches.values_of("url").unwrap().collect::<Vec<_>>(),
            vec!["www.xd.com", "www.example.com"]
        );
    }
}
