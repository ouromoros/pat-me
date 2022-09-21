mod notify;
mod config;
mod command;
mod common;

use core::panic;
use clap::Parser;

use command::CommandResult;
use common::NotifyMethod;

#[derive(Parser, Debug)]
#[clap(version, about)]
struct Cli {
    #[clap(default_value = "default")]
    method: NotifyMethod,

    #[clap(short, long)]
    title: Option<String>,
    #[clap(short, long)]
    msg: Option<String>,

    #[clap(short, long)]
    lookback: bool,

    #[clap(short, long)]
    command: Option<String>
}

#[derive(Debug, Clone)]
struct ParseNotifyError(String);

impl std::fmt::Display for ParseNotifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "error encountered: {}", self.0)
    }
}
impl std::error::Error for ParseNotifyError {}

fn parse_notify(config: &config::Config, args: &Cli) -> Result<Box<dyn notify::Notify>, ParseNotifyError> {
    let method = if let NotifyMethod::Default = args.method {
        config.default_method.parse().expect("default method in config malformed")
    } else {
        args.method
    };

    let notify: Box<dyn notify::Notify> = match method {
        NotifyMethod::Desktop => Box::new(notify::Desktop),
        NotifyMethod::Email => Box::new(notify::Email),
        NotifyMethod::Beep => Box::new(notify::Beep),
        NotifyMethod::Echo => Box::new(notify::Echo),
        _ => panic!("unsupported notify method: {:?}", method),
    };
    Ok(notify)
}

fn parse_content(_config: &config::Config, args: &Cli, result: &Option<CommandResult>) -> notify::Content {
    let mut title = "".to_string();
    let mut msg = "".to_string();
    if let Some(command_result) = result {
        title = if let Some(detail) = &command_result.detail {
            format!("Command '{}' finished execution", detail.command)
        } else {
            "Some command finished execution".to_string()
        };

        msg = if let Some(detail) = &command_result.detail {
            format!("command: {}\nstatus: {}\noutput: {}\nerror output: {}\n",
                detail.command,
                command_result.status,
                detail.out.clone().unwrap_or_default(),
                detail.err.clone().unwrap_or_default(),
            )
        } else {
            format!("status: {}\n", command_result.status)
        };
    }

    if let Some(t) = &args.title {
        title = t.clone();
    }
    if let Some(m) = &args.msg {
        msg = m.clone()
    }

    notify::Content {
        title: title.to_string(),
        msg: msg.to_string(),
    }
}

fn main() {
    let config: config::Config = match confy::load("patme", None) {
        Ok(conf) => conf,
        Err(e) => panic!("Load configuration failed: {:?}", e),
    };

    let cli = Cli::parse();

    let notify = match parse_notify(&config, &cli){
        Ok(noti) => noti,
        Err(e) => panic!("Parse notification failed: {:?}", e)
    };

    let command_result: Option<command::CommandResult> = if cli.lookback {
        Some(command::lookback())
    } else if let Some(c) = &cli.command {
        Some(command::run_command(c))
    } else { None };

    let content = parse_content(&config, &cli, &command_result);

    notify.do_notify(&content)
}
