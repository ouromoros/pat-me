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
            .show().unwrap(); 
    }
}

pub struct Email;

impl Notify for Email {
    fn do_notify(&self, content: &Content) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::{Beep, Notify, Desktop};
    #[test]
    pub fn test_notification() {
        let content = &super::Content { title: "".to_string(), msg: "".to_string() };
        Desktop.do_notify(content)
    }

    #[test]
    pub fn test_beep() {
        let content = &super::Content { title: "".to_string(), msg: "".to_string() };
        Beep.do_notify(content)
    }
}