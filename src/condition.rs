use niri_ipc::{Window, Workspace};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub enum WindowCond {
    AppTitle(String, bool),
    AppID(String, bool),
    WindowIn(String, bool),
    IsFloating(bool),
}

pub enum ConvertedWindowCond {
    NotAppTitle(String),
    NotAppID(String),
    NotWindowIn(u64),
    AppTitle(String),
    AppID(String),
    WindowIn(u64),
    IsFloating(bool),
}

impl ConvertedWindowCond {
    pub fn from(c: WindowCond, state: &Vec<Workspace>) -> Option<Self> {
        match c {
            WindowCond::AppTitle(title, invert) => {
                if invert {
                    Some(ConvertedWindowCond::NotAppTitle(title))
                } else {
                    Some(ConvertedWindowCond::AppTitle(title))
                }
            }
            WindowCond::AppID(id, invert) => {
                if invert {
                    Some(ConvertedWindowCond::NotAppID(id))
                } else {
                    Some(ConvertedWindowCond::AppID(id))
                }
            }
            WindowCond::WindowIn(wsname, invert) => {
                let ws = state.iter().find(|w| {
                    if let Some(name) = &w.name {
                        *name == wsname
                    } else {
                        false
                    }
                });
                if let Some(w) = ws {
                    if invert {
                        Some(ConvertedWindowCond::NotWindowIn(w.id))
                    } else {
                        Some(ConvertedWindowCond::WindowIn(w.id))
                    }
                } else {
                    None
                }
            }
            WindowCond::IsFloating(val) => Some(ConvertedWindowCond::IsFloating(val)),
        }
    }
    pub fn matches(&self, window: &Window) -> bool {
        match self {
            ConvertedWindowCond::NotAppTitle(title) => {
                if let Some(t) = &window.title {
                    !t.contains(title)
                } else {
                    true
                }
            }
            ConvertedWindowCond::NotAppID(id) => {
                if let Some(t) = &window.app_id {
                    !t.contains(id)
                } else {
                    true
                }
            }
            ConvertedWindowCond::NotWindowIn(id) => {
                if let Some(t) = &window.workspace_id {
                    t != id
                } else {
                    true
                }
            }
            ConvertedWindowCond::AppTitle(title) => {
                if let Some(t) = &window.title {
                    t.contains(title)
                } else {
                    false
                }
            }
            ConvertedWindowCond::AppID(id) => {
                if let Some(t) = &window.app_id {
                    t.contains(id)
                } else {
                    false
                }
            }
            ConvertedWindowCond::WindowIn(id) => {
                if let Some(t) = &window.workspace_id {
                    t == id
                } else {
                    false
                }
            }
            ConvertedWindowCond::IsFloating(val) => &window.is_floating == val,
        }
    }
}
