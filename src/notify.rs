use lettre::{transport::smtp::authentication::Credentials, Transport};

pub struct Content {
    pub title: String,
    pub msg: String,
}

pub trait Notify {
    fn do_notify(&self, content: &Content);
}

pub struct Beep;

impl Notify for Beep {
    fn do_notify(&self, _content: &Content) {
        // ASCII beep character
        println!("\x07");
    }
}

pub struct Echo;

impl Notify for Echo {
    fn do_notify(&self, content: &Content) {
        println!("title: {}", content.title);
        println!("msg: {}", content.msg);
    }
}

pub struct Desktop;

impl Notify for Desktop {
    fn do_notify(&self, content: &Content) {
        notify_rust::Notification::new()
            .summary(content.title.as_ref())
            .body(content.msg.as_ref())
            .timeout(0)
            .show()
            .unwrap();
    }
}

pub struct Email {
    pub username: String,
    pub password: String,
    pub server: String,
}

impl Notify for Email {
    fn do_notify(&self, content: &Content) {
        let email = lettre::Message::builder()
            .to(self.username.parse().unwrap())
            .from(self.username.parse().unwrap())
            .subject(content.title.clone())
            .body(content.msg.clone())
            .unwrap();
        let creds = Credentials::new(self.username.clone(), self.password.clone());
        let mailer = lettre::SmtpTransport::relay(&self.server)
            .unwrap()
            .credentials(creds)
            .build();

        mailer.send(&email).expect("send mail failed");
    }
}

#[cfg(test)]
mod tests {
    use super::{Beep, Desktop, Echo, Email, Notify};
    #[test]
    #[ignore]
    pub fn test_desktop() {
        let content = &super::Content {
            title: "Ahoy".to_string(),
            msg: "Be aware, Amigo.".to_string(),
        };
        Desktop.do_notify(content)
    }

    #[test]
    #[ignore]
    pub fn test_beep() {
        let content = &super::Content {
            title: "Ahoy".to_string(),
            msg: "Be aware, Amigo.".to_string(),
        };
        Beep.do_notify(content)
    }

    #[test]
    #[ignore]
    pub fn test_email() {
        let content = &super::Content {
            title: "Ahoy".to_string(),
            msg: "Be aware, Amigo.".to_string(),
        };
        let email = Email {
            username: "12345@qq.com".to_string(),
            password: "12345".to_string(),
            server: "smtp.qq.com".to_string(),
        };
        email.do_notify(content);
    }

    #[test]
    #[ignore]
    pub fn test_echo() {
        let content = &super::Content {
            title: "Ahoy".to_string(),
            msg: "Be aware, Amigo.".to_string(),
        };
        Echo.do_notify(content);
    }
}
