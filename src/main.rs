use std::env;
use sway_workspace_manager::{run, Command, Workspaces};
use swayipc::Connection;

const USAGE: &str =  "\
Usage:
    sway-workspace-manager switch|move|create|move-to-new|swap [prev|next|start|end|<number>] [--cycle] [--extra]
    sway-workspace-manager reorder
    sway-workspace-manager rename <new-name>
";

fn main() -> Result<(), swayipc::Error> {
    let mut connection = Connection::new()?;

    let command = match Command::new(env::args()) {
        Ok(command) => command,
        Err(msg) => {
            eprintln!("{USAGE}");
            return Err(swayipc::Error::CommandFailed(format!("Parse error: {msg}")));
        }
    };

    let workspaces = Workspaces::get(&mut connection)?.ordered(&mut connection)?;

    run(&mut connection, workspaces, command)?;

    Ok(())
}
