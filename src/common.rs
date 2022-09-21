#[derive(Debug, Clone, Copy)]
pub enum NotifyMethod {
    Default,
    Email,
    Beep,
    Desktop,
    Echo,
}

#[derive(Debug, Clone)]
pub struct ParseMethodError(String);

impl std::fmt::Display for ParseMethodError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} does not match any default or custom notify command", self.0)
    }
}
impl std::error::Error for ParseMethodError {}

impl core::fmt::Display for NotifyMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl core::str::FromStr for NotifyMethod {
    type Err = ParseMethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let method = match s {
            "default" => NotifyMethod::Default,
            "email" => NotifyMethod::Email,
            "beep" => NotifyMethod::Beep,
            "desktop" => NotifyMethod::Desktop,
            "echo" => NotifyMethod::Echo,
            _ => return Err(ParseMethodError(s.to_string()))
        };
        Ok(method)
    }
}
