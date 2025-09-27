use niri_ipc::{Action, Event, Request, Response, WorkspaceReferenceArg, socket::Socket};
use std::error::Error;

fn move_window_to_current(
    window_id: u64,
    ws: WorkspaceReferenceArg,
) -> Result<Response, Box<dyn Error>> {
    let mut socket = Socket::connect()?;
    Ok(socket.send(Request::Action(Action::MoveWindowToWorkspace {
        window_id: Some(window_id),
        reference: ws,
        focus: false,
    }))??)
}

fn main() -> std::io::Result<()> {
    let mut read_soc = Socket::connect()?;
    let mut current = 0;

    let reply = read_soc.send(Request::EventStream)?;
    if matches!(reply, Ok(Response::Handled)) {
        let mut read_event = read_soc.read_events();
        while let Ok(event) = read_event() {
            match event {
                Event::WindowOpenedOrChanged { window } => {
                    if current > 0
                        && let Some(title) = window.title
                        && let Some(appid) = window.app_id
                    {
                        if title == "Unlock Database - KeePassXC"
                            && appid == "org.keepassxc.KeePassXC"
                        {
                            // Responses are ignored
                            let _ = move_window_to_current(
                                window.id,
                                WorkspaceReferenceArg::Id(current),
                            );
                        }
                    }
                }
                Event::WorkspacesChanged { workspaces } => {
                    if let Some(ws) = workspaces.into_iter().find(|w| w.is_active) {
                        current = ws.id;
                    };
                }
                Event::WorkspaceActivated { id, focused: _ } => {
                    current = id;
                }
                _ => {}
            }
        }
    }
    Ok(())
}
