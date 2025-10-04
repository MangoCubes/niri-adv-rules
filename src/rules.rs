use std::error::Error;

use niri_ipc::{Request, Window, Workspace, socket::Socket};

use crate::{
    action::{ConvertedWindowAction, WindowAction},
    condition::{ConvertedWindowCond, WindowCond},
};

#[derive(Clone)]
pub struct Rules(pub Vec<Rule>);

#[derive(Clone)]
pub struct WindowRule(pub Vec<WindowCond>, pub Vec<WindowAction>);

#[derive(Clone)]
pub enum Rule {
    Window(WindowRule),
}

impl Rules {
    pub fn convert(self, ws: &Vec<Workspace>) -> ConvertedRules {
        let mut wr = vec![];
        for r in self.0 {
            match r {
                Rule::Window(window_rule) => wr.push(window_rule),
            }
        }
        ConvertedRules {
            window: wr
                .into_iter()
                .map(|r| ConvertedWindowRule::from(r, ws))
                .collect(),
        }
    }
}

pub struct ConvertedWindowRule {
    pub conds: Vec<ConvertedWindowCond>,
    pub action: Vec<ConvertedWindowAction>,
}

impl ConvertedWindowRule {
    pub fn from(r: WindowRule, state: &Vec<Workspace>) -> Self {
        let conds =
            r.0.into_iter()
                .filter_map(|c| ConvertedWindowCond::from(c, state))
                .collect();
        let action =
            r.1.into_iter()
                .filter_map(|a| ConvertedWindowAction::from(a, state))
                .collect();
        Self { conds, action }
    }
    pub fn run(&self, window: &Window, current: u64) -> Result<bool, Box<dyn Error>> {
        for c in &self.conds {
            if !c.matches(window) {
                return Ok(false);
            }
        }

        let mut socket = Socket::connect()?;
        let actions: Vec<Request> = self
            .action
            .iter()
            .map(|a| a.get_action(window, current))
            .collect();
        for a in actions {
            let _ = socket.send(a)?;
        }
        return Ok(true);
    }
}

pub struct ConvertedRules {
    pub window: Vec<ConvertedWindowRule>,
}

impl ConvertedRules {
    pub fn try_window(&self, window: &Window, current: u64) {
        for r in &self.window {
            match r.run(window, current) {
                Ok(matched) => {
                    if matched {
                        return ();
                    }
                }
                Err(_) => {
                    return ();
                }
            };
        }
    }
}
