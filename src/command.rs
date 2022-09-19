mod command {
    pub struct CommandResult {
        pub status: bool,
        pub detail: Option<CommandResultDetail>
    }

    pub struct CommandResultDetail {
        pub command: String,
        pub out: String,
        pub err: String,
    }
}
