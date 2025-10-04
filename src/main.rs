use niri_ipc::{Event, Request, Response, socket::Socket};

use crate::{
    action::WindowAction,
    condition::WindowCond,
    rules::{ConvertedRules, Rule, Rules, WindowRule},
};

mod action;
mod condition;
mod rules;

fn main() -> std::io::Result<()> {
    let mut read_soc = Socket::connect()?;
    let mut current = 0;
    let rules: Rules = Rules(vec![Rule::Window(WindowRule(
        vec![
            WindowCond::IsFloating(true),
            WindowCond::AppID("org.keepassxc.KeePassXC".to_string(), false),
        ],
        vec![WindowAction::MoveToWorkspace(None)],
    ))]);

    let mut converted_rules: Option<ConvertedRules> = None;

    let reply = read_soc.send(Request::EventStream)?;
    if matches!(reply, Ok(Response::Handled)) {
        let mut read_event = read_soc.read_events();
        while let Ok(event) = read_event() {
            match event {
                Event::WorkspacesChanged { workspaces } => {
                    converted_rules = Some(rules.clone().convert(&workspaces));
                    if let Some(ws) = workspaces.into_iter().find(|w| w.is_active) {
                        current = ws.id;
                    };
                }
                Event::WindowOpenedOrChanged { window } => {
                    if current != 0 {
                        if let Some(r) = &converted_rules {
                            r.try_window(&window, current);
                        }
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
