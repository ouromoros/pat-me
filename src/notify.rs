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

#[cfg(test)]
mod tests {
    use notify_rust::Notification;

    use super::{Beep, Notify};
    #[test]
    pub fn test_notification() {
        Notification::new()
            .summary("Ahoy")
            .body("Be aware, Amigo.")
            .show().unwrap();
    }

    #[test]
    pub fn test_beep() {
        let content = &super::Content { title: "".to_string(), msg: "".to_string() };
        Beep.do_notify(content)
    }
}