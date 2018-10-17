use failure::{Fallible, ResultExt};
use lettre::{smtp::authentication::Credentials, SmtpClient, Transport};
use lettre_email::{Email, EmailBuilder};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub target: String,
    pub email: String,
    pub passwd: String,
    #[serde(default)]
    pub smtp: String, // TODO defaults
}

pub struct MailNotificationBuilder {
    pub url: String,
}

impl MailNotificationBuilder {
    pub fn send(&self, config: &Config) -> Fallible<()> {
        let smtp: &str = &config.smtp;
        let mut mailer = SmtpClient::new_simple(smtp)
            .context("Failed to create StmpClient")?
            .credentials(Credentials::new(
                config.email.to_string(),
                config.passwd.to_string(),
            )).transport();

        let email = self
            .create_email(config)
            .context("Error creating the e-mail")?;
        mailer
            .send(email.into())
            .context("Failed to send the e-mail")?;
        Ok(())
    }

    pub fn create_email(&self, config: &Config) -> Fallible<Email> {
        let body = format!(
            "A watched file has been changed: {}\n\
             Last-Modified: {}",
            self.url, "placeholder"
        );

        EmailBuilder::new()
            .to(config.target.clone())
            .from(config.email.clone())
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
