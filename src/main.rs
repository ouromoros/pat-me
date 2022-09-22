mod command;
mod common;
mod config;
mod notify;

use clap::Parser;
use core::panic;

use command::CommandResult;
use common::NotifyMethod;

#[derive(Parser, Debug)]
#[clap(version, about)]
/// Easy-to-use CLI notification tool
struct Cli {
    #[clap(default_value = "default")]
    /// Notification method. Current support: desktop, email, beep
    method: NotifyMethod,

    #[clap(short, long)]
    /// Notification title
    title: Option<String>,
    #[clap(short, long)]
    /// Notification body
    msg: Option<String>,

    // #[clap(short, long)]
    // lookback: bool,
    #[clap(short, long)]
    /// Command to execute
    command: Option<String>,

    #[clap(long)]
    /// Specify custom config path
    config: Option<String>,
    #[clap(long)]
    /// Open config file
    open_config: bool,
}

#[derive(Debug, Clone)]
struct ParseNotifyError(String);

impl std::fmt::Display for ParseNotifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "error encountered: {}", self.0)
    }
}
impl std::error::Error for ParseNotifyError {}

fn parse_notify(
    config: &config::Config,
    args: &Cli,
) -> Result<Box<dyn notify::Notify>, ParseNotifyError> {
    let method = if let NotifyMethod::Default = args.method {
        config
            .default_method
            .parse()
            .expect("default method in config malformed")
    } else {
        args.method
    };

    let notify: Box<dyn notify::Notify> = match method {
        NotifyMethod::Desktop => Box::new(notify::Desktop),
        NotifyMethod::Email => Box::new(parse_email(config)?),
        NotifyMethod::Beep => Box::new(notify::Beep),
        NotifyMethod::Echo => Box::new(notify::Echo),
        _ => panic!("unsupported notify method: {:?}", method),
    };
    Ok(notify)
}

fn parse_email(config: &config::Config) -> Result<notify::Email, ParseNotifyError> {
    match &config.email_config {
        None => Err(ParseNotifyError(
            "email config is not configured".to_string(),
        )),
        Some(conf) => Ok(notify::Email {
            username: conf.username.clone(),
            password: conf.password.clone(),
            server: conf.server.clone(),
        }),
    }
}

fn parse_content(
    config: &config::Config,
    args: &Cli,
    result: &Option<CommandResult>,
) -> notify::Content {
    let mut title = config.default_title.clone();
    let mut msg = config.default_msg.clone();
    if let Some(command_result) = result {
        if let Some(detail) = &command_result.detail {
            title = format!("Command '{}' finished execution", detail.command);
        }

        msg = if let Some(detail) = &command_result.detail {
            format!(
                "command: {}\nstatus: {}\noutput: {}\nerror output: {}\n",
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

fn open_config_file() {
    let file_path = confy::get_configuration_file_path("patme", None).unwrap();
    if cfg!(target_os = "windows") {
        std::process::Command::new("explorer")
            .arg(file_path.to_string_lossy().to_string())
            .spawn()
            .unwrap();
    } else {
        std::process::Command::new("vi")
            .arg(file_path.to_string_lossy().to_string())
            .status()
            .unwrap();
    }
}

fn main() {
    let cli = Cli::parse();

    let config_path = match &cli.config {
        Some(s) => s.clone(),
        None => confy::get_configuration_file_path("patme", None)
            .unwrap()
            .to_string_lossy()
            .to_string(),
    };
    let config = match confy::load_path(&config_path) {
        Ok(conf) => conf,
        Err(e) => panic!("Load configuration ({}) failed: {:?}", &config_path, e),
    };

    if cli.open_config {
        open_config_file();
        return;
    }

    let notify = match parse_notify(&config, &cli) {
        Ok(noti) => noti,
        Err(e) => panic!("Parse notification failed: {:?}", e),
    };

    let command_result: Option<command::CommandResult> = if false {
        Some(command::lookback())
    } else if let Some(c) = &cli.command {
        Some(command::run_command(c))
    } else {
        None
    };

    let content = parse_content(&config, &cli, &command_result);

    notify.do_notify(&content)
}
