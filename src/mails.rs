use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::{EmailAddress, Envelope, SendableEmail, SmtpClient, Transport};
use lettre_email::error::Error;
use lettre_email::{Email, EmailBuilder};
use std::borrow::Cow;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config<'a> {
    target: &'a str,
    email: &'a str,
    passwd: &'a str,
    #[serde(default)]
    smtp: Cow<'a, str>,
}

pub fn send_mail(email: SendableEmail, config: &Config) {
    let smtp: &str = &config.smtp;
    let mut mailer = SmtpClient::new_simple(smtp)
        .expect("Failed to create StmpClient")
        .credentials(Credentials::new(
            config.email.to_string(),
            config.passwd.to_string(),
        )).transport();

    mailer.send(email).expect("Failed to send the e-mail");
}

pub struct CommandStatusMail {
    pub url: String,
}

impl CommandStatusMail {
    pub fn create_email(&self, config: &Config) -> Result<Email, Error> {
        let body = format!(
            "A watched file has been changed: {}\n\
             Last-Modified: {}",
            self.url, "placeholder"
        );

        EmailBuilder::new()
            .to(config.target)
            .from(config.email)
            .subject("Remote file changed")
            .text(body)
            .build()
    }
}

/*#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::process::ExitStatusExt;
    use std::borrow::Cow;

    const CONFIG: Config = Config {
        target: "target",
        email: "email",
        passwd: "passwd",
        smtp: Cow::Borrowed("smtp"),
    };

    #[test]
    fn test_mail_creation() {
        let status = CommandStatusMail {
            cmdline: vec!["foo", "bar", "baz"],
            duration: Duration::from_millis(1024),
            status: ExitStatus::from_raw(0),
            jobname: Some("greatest"),
        };

        let email = status.create_email(&CONFIG).unwrap();
        let body = format!("{}", email);

        assert!(body.contains("foo bar baz"));
        assert!(body.contains("To: <target>"));
        assert!(body.contains("From: <email>"));
        assert!(body.contains("1.024 s"));
        assert!(body.contains("code: 0"));
        assert!(body.contains("Job name: greatest"));
    }
}*/
