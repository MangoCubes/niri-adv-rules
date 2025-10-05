use std::{fs::File, io::Read, path::PathBuf};

use clap::Parser;
use home::home_dir;
use niri_ipc::{Event, Request, Response, socket::Socket};
use serde_json::Error;

use crate::rules::{ConvertedRules, Rules};

mod action;
mod condition;
mod rules;

#[derive(Parser)]
struct Cli {
    /// Path to the config file
    #[clap(short, long, value_parser)]
    config: Option<PathBuf>,
}

fn read_config(file_path: &str) -> Result<Rules, Error> {
    let mut file = File::open(file_path).expect("Unable to open file.");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Unable to read file.");

    serde_json::from_str(&data)
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let default_path = home_dir()
        .expect("You don't have a home??")
        .join(".config/niri-adv-rules/config.json");
    let config_path = cli.config.unwrap_or(PathBuf::from(default_path));
    let rules =
        read_config(config_path.to_str().unwrap()).expect("Failed to parse config into rules.");

    println!(
        "Rules read successfully! Rules:\n{}",
        serde_json::to_string(&rules)
            .expect("Rules have been parsed successfully but cannot be converted back to string??")
    );

    let mut read_soc = Socket::connect()?;
    let mut current = 0;
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
