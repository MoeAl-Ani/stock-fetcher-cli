use std::{fs, process};
use serde::{Deserialize, Serialize};
extern crate lettre;
extern crate lettre_email;

use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;


pub struct EmailService {
    email_config: EmailConfig,
}
impl EmailService {
    pub fn new(config_path: &String) -> Self {
        let config = EmailConfig::load(config_path);
        EmailService {email_config: config}
    }

    pub fn send(&self, from: &str, to: &str, subject: &str, content: &str) {

        let email = EmailBuilder::new()
            .to(to)
            .from(from)
            .subject("stock-fetcher update")
            .html("<h1>the current price hitting the threshold!</h1>")
            .build()
            .unwrap();

        let mut mailer = SmtpClient::new_simple(&self.email_config.smtp)
            .unwrap()
            .credentials(Credentials::new(self.email_config.user_name.clone().into(), self.email_config.password.clone().into()))
            .transport();

        let result = mailer.send(email.into());


        if result.is_ok() {
            println!("email sending succeeded!")
        } else {
            println!("email sending failed {}", result.err().unwrap().to_string())
        }


    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EmailConfig {
    user_name: String,
    password: String,
    smtp: String,
    sender_email: String,
    port: i32,
}

impl EmailConfig {
    pub fn load(config_path: &String) -> Self {
        let email_config_data = fs::read_to_string(config_path).unwrap_or_else(|err| {
            eprintln!("error reading file {}", err);
            process::exit(1);
        });
        let email_config: EmailConfig = serde_json::from_str(email_config_data.as_str()).unwrap_or_else(|err| {
            eprintln!("error deserializing file content {}", err);
            process::exit(1);
        });
        email_config
    }
}

#[cfg(test)]
mod test {
    use crate::email::{EmailConfig, EmailService};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_read_config() {
        let config = EmailConfig::load(&"email_config.json".to_string());
        assert_eq!("smtp-relay.gmail.com", config.smtp);
        assert_eq!(587, config.port);
    }
}