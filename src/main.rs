// SPDX-FileCopyrightText: 2026 The mwifiex_nl80211_test_mode authors
// SPDX-License-Identifier: GPL-2.0-only

use std::error::Error;
use std::process::ExitCode;

use clap::Parser;
use colored::*;

use mwifiex_nl80211_test_mode::command::CardType;
use mwifiex_nl80211_test_mode::interactive::run_interactive;
use mwifiex_nl80211_test_mode::netlink::{
    MwifiexDryRunHandle, MwifiexNetlinkHandle, MwifiexNetlinkInterface,
};
use mwifiex_nl80211_test_mode::runner::run_sequence_file;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Mwifiex network interface
    #[arg(short, long, default_value_t = String::from("mlan0"))]
    interface: String,
    /// Launch interactive shell
    #[arg(long)]
    interactive: bool,
    /// Path to YAML sequence file
    #[arg(short, long)]
    file: Option<String>,
    /// Only simulate sending commands
    #[arg(short, long)]
    dry_run: bool,
    /// Set the Mwifiex card type
    #[arg(short, long, value_enum, default_value_t = CardType::IW610)]
    card_type: CardType,
    /// Set a variable value, overriding defaults from the YAML file.
    /// Can be specified multiple times. Format: --set KEY=VALUE
    #[arg(short = 's', long = "set", value_parser = parse_key_val)]
    set: Vec<(String, String)>,
}

fn parse_key_val(s: &str) -> Result<(String, String), String> {
    s.split_once('=')
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .ok_or_else(|| format!("invalid KEY=VALUE format: no `=` found in `{s}`"))
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let handle: Box<dyn MwifiexNetlinkInterface> = if args.dry_run {
        Box::new(MwifiexDryRunHandle {
            card_type: args.card_type,
        })
    } else {
        Box::new(MwifiexNetlinkHandle::from_interface(
            &args.interface,
            args.card_type,
        )?)
    };

    if args.interactive {
        return run_interactive(handle.as_ref());
    }

    if let Some(path) = &args.file {
        return run_sequence_file(handle.as_ref(), path, &args.set);
    }

    Ok(())
}

fn main() -> ExitCode {
    if let Err(e) = run() {
        println!("{}", format!("error: {e}").red());
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
