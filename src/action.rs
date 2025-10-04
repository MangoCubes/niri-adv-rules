use niri_ipc::{Action, Request, Window, Workspace, WorkspaceReferenceArg};

#[derive(Clone)]
pub enum WindowAction {
    MoveToWorkspace(Option<String>),
}

pub enum ConvertedWindowAction {
    MoveToWorkspace(Option<u64>),
}

impl ConvertedWindowAction {
    pub fn get_action(&self, window: &Window, current: u64) -> Request {
        match &self {
            ConvertedWindowAction::MoveToWorkspace(wsid) => {
                let ws = if let Some(id) = wsid {
                    WorkspaceReferenceArg::Id(*id)
                } else {
                    WorkspaceReferenceArg::Id(current)
                };
                Request::Action(Action::MoveWindowToWorkspace {
                    window_id: Some(window.id),
                    reference: ws,
                    focus: false,
                })
            }
        }
    }
    pub fn from(r: WindowAction, state: &Vec<Workspace>) -> Option<Self> {
        match r {
            WindowAction::MoveToWorkspace(wsname) => match wsname {
                Some(wsn) => {
                    let ws = state.iter().find(|w| {
                        if let Some(name) = &w.name {
                            *name == wsn
                        } else {
                            false
                        }
                    });
                    match ws {
                        Some(w) => Some(ConvertedWindowAction::MoveToWorkspace(Some(w.id))),
                        None => None,
                    }
                }
                None => Some(ConvertedWindowAction::MoveToWorkspace(None)),
            },
        }
    }
}
