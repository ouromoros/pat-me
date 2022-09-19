mod notify;
mod config;
mod command;

use core::panic;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about)]
struct Cli {
    method: Option<NotifyMethod>,
    title: Option<String>,
    msg: Option<String>,
}

#[derive(Debug)]
enum NotifyMethod {
    Default,
    Email,
    Beep,
    Desktop,
}

#[derive(Debug, Clone)]
struct ParseMethodError(String);

impl std::fmt::Display for ParseMethodError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} does not match any default or custom notify command", self.0)
    }
}
impl std::error::Error for ParseMethodError {}

#[derive(Debug, Clone)]
struct ParseNotifyError(String);

impl std::fmt::Display for ParseNotifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "error encountered: {}", self.0)
    }
}
impl std::error::Error for ParseNotifyError {}

impl core::str::FromStr for NotifyMethod {
    type Err = ParseMethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let method = match s {
            "default" => NotifyMethod::Default,
            "email" => NotifyMethod::Email,
            "beep" => NotifyMethod::Beep,
            "desktop" => NotifyMethod::Desktop,
            _ => return Err(ParseMethodError(s.to_string()))
        };
        Ok(method)
    }
}

fn parse_notify(config: &config::Config, args: &Cli) -> Result<Box<dyn notify::Notify>, ParseNotifyError> {
    Ok(Box::new(notify::Beep))
}

fn main() {
    let config: config::Config = match confy::load("patme", None) {
        Ok(conf) => conf,
        Err(e) => panic!("Load configuration failed"),
    };

    let cli = Cli::parse();

    let notify = match parse_notify(&config, &cli){
        Ok(noti) => noti,
        Err(e) => panic!("Parse notification failed")
    };

    let content = crate::notify::Content { title: "".to_string(), msg: "".to_string() };

    notify.do_notify(&content)
}
