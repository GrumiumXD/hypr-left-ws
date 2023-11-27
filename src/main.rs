use hyprland::{
    data::{Monitors, Workspaces},
    dispatch::{
        Dispatch, DispatchType, MonitorIdentifier, WorkspaceIdentifier,
        WorkspaceIdentifierWithSpecial,
    },
    shared::HyprData,
};
use std::env;
use std::error;

fn main() -> std::result::Result<(), Box<dyn error::Error>> {
    let target_ws_id = env::args()
        .nth(1)
        .ok_or("no arg provided")?
        .parse::<i32>()?;

    let workspaces = Workspaces::get()?;
    let target_ws = workspaces.iter().find(|&w| w.id == target_ws_id);

    // check if the ws already exists
    if let Some(ws) = target_ws {
        let monitors = Monitors::get()?;

        let current_monitor = monitors
            .iter()
            .find(|&m| m.focused)
            .ok_or("no current monitor found")?;

        if ws.name != current_monitor.name {
            let target_monitor = monitors.iter().find(|&m| m.active_workspace.id == ws.id);

            if let Some(tm) = target_monitor {
                // swap workspaces on current and target monitor
                Dispatch::call(DispatchType::SwapActiveWorkspaces(
                    MonitorIdentifier::Name(&current_monitor.name),
                    MonitorIdentifier::Name(&tm.name),
                ))?;
                // nothing else to do
                return Ok(());
            }

            // move the target ws to the current monitor
            Dispatch::call(DispatchType::MoveWorkspaceToMonitor(
                WorkspaceIdentifier::Id(ws.id),
                MonitorIdentifier::Current,
            ))?;
        }
    }

    // switch to target workspace
    Dispatch::call(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Id(
        target_ws_id,
    )))?;
    
    Ok(())
}
