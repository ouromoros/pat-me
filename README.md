# patme

```
patme 0.1.0
Easy-to-use CLI notification tool

USAGE:
    patme.exe [OPTIONS] [METHOD]

ARGS:
    <METHOD>    Notification method. Current support: desktop, email, beep [default: default]

OPTIONS:
    -c, --command <COMMAND>    Command to execute
        --config <CONFIG>      Specify custom config path
    -h, --help                 Print help information
    -m, --msg <MSG>            Notification body
        --open-config          Open config file
    -t, --title <TITLE>        Notification title
    -V, --version              Print version information
```

## Config

```yaml
---
default_method: desktop
default_title: "[Empty Title]"
default_msg: "Oops, empty message body."
email_config:
  username: 12345@qq.com
  password: 12345
  server: smtp.qq.com

```
