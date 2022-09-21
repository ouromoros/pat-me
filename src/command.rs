use powershell_script::PsScriptBuilder;

pub struct CommandResult {
    pub status: bool,
    pub detail: Option<CommandResultDetail>
}

pub struct CommandResultDetail {
    pub command: String,
    pub out: Option<String>,
    pub err: Option<String>,
}

pub fn run_command(c: &str) -> CommandResult {
    if cfg!(target_os = "windows") {
        let ps = PsScriptBuilder::new()
            .no_profile(false)
            .non_interactive(true)
            .hidden(true)
            .build();
        let output = ps.run(c).expect("run powershell command failed");
        CommandResult {
            status: output.success(),
            detail: Some(CommandResultDetail {
                command: c.to_string(),
                out: output.stdout(),
                err: output.stderr(),
            })
        }
    } else {
        unimplemented!()
    }
}

pub fn lookback() -> CommandResult {
    let status = if cfg!(target_os = "windows") {
        unimplemented!()
    } else if cfg!(target_os = "linux") {
        unimplemented!()
    } else {
        unimplemented!()
    };
    CommandResult { status: status, detail: None }
}
