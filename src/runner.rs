// SPDX-FileCopyrightText: 2026 The mwifiex_nl80211_test_mode authors
// SPDX-License-Identifier: GPL-2.0-only

use colored::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fs::read_to_string, thread::sleep, time::Duration};

use crate::{command::MfgCmd, netlink::MwifiexNetlinkInterface};

/// Control flow steps for command sequence files.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlStep {
    /// Wait for the user to press Enter before continuing.
    WaitForConfirmation {
        /// Optional message to display to the user.
        #[serde(default)]
        message: Option<String>,
    },
    /// Wait a specific amount of time before continuing.
    Delay {
        /// Duration in ms to wait
        duration: u64,
    },
}

/// Wrapper enum for command sequence YAML files.
///
/// Each step in a command sequence is either a testmode command or a control
/// flow operation such as waiting for user input or a timed delay.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SequenceStep {
    /// A control flow step (wait for input or delay).
    Control(ControlStep),
    /// A testmode command to send to the firmware.
    Command(MfgCmd),
}

pub fn run_sequence_file(
    handle: &dyn MwifiexNetlinkInterface,
    file_path: &str,
    cli_variables: &[(String, String)],
) -> Result<(), Box<dyn Error>> {
    println!("Running:");
    println!("{file_path}");

    let steps = parse_sequence_file(&read_to_string(file_path)?, cli_variables)?;

    println!("Sending:");
    for (idx, step) in steps.iter().enumerate() {
        match step {
            SequenceStep::Command(cmd) => {
                match handle.send_mfg_cmd(cmd) {
                    Ok(response) => {
                        println!("{}", format!("{idx}: {cmd:#?} => {response:#?}").green());
                    }
                    Err(err) => {
                        println!("{}", format!("{idx}: Error: {cmd:#?} => {err}").red());

                        if inquire::Confirm::new("Continue?")
                            .with_default(false)
                            .prompt()
                            .is_ok_and(|c| !c)
                        {
                            break;
                        }
                    }
                };
                // mwifiex driver/card needs time to process the command. Sleep for a little while
                // otherwise subsequent commands will fail if sent to fast.
                std::thread::sleep(Duration::from_millis(500));
            }
            SequenceStep::Control(ControlStep::WaitForConfirmation { message }) => {
                println!(
                    "{idx}: {}",
                    message.as_deref().unwrap_or("Press Enter to continue...")
                );
                std::io::stdin().read_line(&mut String::new())?;
            }
            SequenceStep::Control(ControlStep::Delay { duration }) => {
                println!("{idx}: Waiting for {}ms...", duration);
                sleep(Duration::from_millis(*duration));
            }
        }
    }
    println!("Done");

    Ok(())
}

fn parse_sequence_file(
    raw_yaml: &str,
    cli_variables: &[(String, String)],
) -> Result<Vec<SequenceStep>, Box<dyn Error>> {
    let mut raw_yaml = raw_yaml.to_owned();

    #[derive(Deserialize, Default)]
    struct VariablesSection {
        #[serde(default)]
        variables: HashMap<String, String>,
    }

    let mut variables = serde_saphyr::from_str::<VariablesSection>(&raw_yaml)
        .unwrap_or_default()
        .variables;

    for (key, value) in cli_variables {
        if let Some(var) = variables.get_mut(key) {
            *var = value.clone()
        } else {
            println!(
                "{}",
                format!("{key}={value} was not applied as {key} isn't defined as a variable.")
                    .yellow()
            )
        }
    }

    if !variables.is_empty() {
        println!("Variables:");
        for (key, value) in variables.iter() {
            println!("  {key}={value}");
            raw_yaml = raw_yaml.replace(&format!("${{{key}}}"), value);
        }
    }
    #[derive(Deserialize)]
    struct SequenceFile {
        steps: Vec<SequenceStep>,
    }

    serde_saphyr::from_str::<SequenceFile>(&raw_yaml)
        .map(|f| f.steps)
        .or_else(|_| serde_saphyr::from_str(&raw_yaml))
        .map_err(|err| format!("Malformed YAML - failed to read sequence file. {err}").into())
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use crate::command::{
        ActiveSubchannel, AntennaMode, ChannelBandwidth, Modulation, RfBand, TxPathId,
    };

    use super::*;

    #[test]
    fn deserialize_sequence_step_command() {
        let input = r#"
        rf_test_mode:
            enable: true
        "#;

        let step: SequenceStep = serde_saphyr::from_str(input).expect("failed to deserialize yaml");

        assert_eq!(
            step,
            SequenceStep::Command(MfgCmd::RfTestMode { enable: true })
        );
    }

    #[test]
    fn deserialize_sequence_step_wait_for_confirmation_with_message() {
        let input = r#"
        wait_for_confirmation:
            message: "Connect the antenna, then press Enter..."
        "#;

        let step: SequenceStep = serde_saphyr::from_str(input).expect("failed to deserialize yaml");

        assert_eq!(
            step,
            SequenceStep::Control(ControlStep::WaitForConfirmation {
                message: Some("Connect the antenna, then press Enter...".to_string()),
            })
        );
    }

    #[test]
    fn deserialize_sequence_step_wait_for_confirmation_no_message() {
        let input = r#"
        wait_for_confirmation: {}
        "#;

        let step: SequenceStep = serde_saphyr::from_str(input).expect("failed to deserialize yaml");

        assert_eq!(
            step,
            SequenceStep::Control(ControlStep::WaitForConfirmation { message: None })
        );
    }

    #[test]
    fn deserialize_sequence_step_delay() {
        let input = r#"
        delay:
            duration: 2000
        "#;

        let step: SequenceStep = serde_saphyr::from_str(input).expect("failed to deserialize yaml");

        assert_eq!(
            step,
            SequenceStep::Control(ControlStep::Delay { duration: 2000 })
        );
    }

    #[test]
    fn deserialize_mixed_sequence() {
        let input = r#"
        - rf_test_mode:
            enable: true
        - radio_mode:
            radio0: 11
            radio1: 0
        - wait_for_confirmation:
            message: "Attach probe"
        - tx_antenna:
            mode: a
        - delay:
            duration: 500
        - tx_continuous:
            enable: true
            tx_rate: 1
        - wait_for_confirmation: {}
        - tx_continuous:
            enable: false
        "#;

        let steps: Vec<SequenceStep> =
            serde_saphyr::from_str(input).expect("failed to deserialize yaml");

        assert_eq!(
            steps,
            vec![
                SequenceStep::Command(MfgCmd::RfTestMode { enable: true }),
                SequenceStep::Command(MfgCmd::RadioMode {
                    radio0: 11,
                    radio1: 0
                }),
                SequenceStep::Control(ControlStep::WaitForConfirmation {
                    message: Some("Attach probe".to_string()),
                }),
                SequenceStep::Command(MfgCmd::TxAntenna {
                    mode: AntennaMode::A
                }),
                SequenceStep::Control(ControlStep::Delay { duration: 500 }),
                SequenceStep::Command(MfgCmd::TxContinuous {
                    enable: true,
                    continuous_wave_mode: false,
                    payload_pattern: 0,
                    cs_mode: false,
                    active_subchannel: ActiveSubchannel::Lower,
                    tx_rate: 1,
                }),
                SequenceStep::Control(ControlStep::WaitForConfirmation { message: None }),
                SequenceStep::Command(MfgCmd::TxContinuous {
                    enable: false,
                    continuous_wave_mode: false,
                    payload_pattern: 0,
                    cs_mode: false,
                    active_subchannel: ActiveSubchannel::Lower,
                    tx_rate: 0,
                }),
            ]
        );
    }

    #[test]
    fn deserialize_as_sequence_steps() {
        let input = r#"
        - rf_test_mode:
            enable: true
        - radio_mode:
            radio0: 0
            radio1: 11
        - tx_antenna:
            mode: a
        - rf_channel:
            channel: 6
        "#;

        let steps: Vec<SequenceStep> =
            serde_saphyr::from_str(input).expect("failed to deserialize yaml");

        assert_eq!(
            steps,
            vec![
                SequenceStep::Command(MfgCmd::RfTestMode { enable: true }),
                SequenceStep::Command(MfgCmd::RadioMode {
                    radio0: 0,
                    radio1: 11
                }),
                SequenceStep::Command(MfgCmd::TxAntenna {
                    mode: AntennaMode::A
                }),
                SequenceStep::Command(MfgCmd::RfChannel { channel: 6 })
            ]
        );
    }

    #[test]
    fn deserialize_sequence_file_with_variables() {
        let input = r#"
        variables:
          channel: 1
          band: ghz2_4
          bandwidth: bw20
          modulation: cck
          tx_rate: 1
          radio0: 11
          radio1: 0
          power_dbm: -1
          tx_path: a

        steps:
          - radio_mode:
              radio0: ${radio0}
              radio1: ${radio1}

          - rf_band:
              band: ${band}

          - channel_bandwidth:
              bandwidth: ${bandwidth}

          - rf_channel:
              channel: ${channel}

          - tx_power:
              power_dbm: ${power_dbm}
              modulation: ${modulation}
              path: ${tx_path}

          - tx_continuous:
              enable: true
              continuous_wave_mode: false
              payload_pattern: 0xAAA
              cs_mode: false
              active_subchannel: both
              tx_rate: ${tx_rate}
        "#;

        let steps = parse_sequence_file(input, &[]).expect("failed to deserialize sequence file");

        assert_eq!(
            steps,
            vec![
                SequenceStep::Command(MfgCmd::RadioMode {
                    radio0: 11,
                    radio1: 0
                }),
                SequenceStep::Command(MfgCmd::RfBand {
                    band: RfBand::Ghz2_4
                }),
                SequenceStep::Command(MfgCmd::ChannelBandwidth {
                    bandwidth: ChannelBandwidth::Bw20
                }),
                SequenceStep::Command(MfgCmd::RfChannel { channel: 1 }),
                SequenceStep::Command(MfgCmd::TxPower {
                    power_dbm: -1,
                    modulation: Modulation::Cck,
                    path: TxPathId::A,
                }),
                SequenceStep::Command(MfgCmd::TxContinuous {
                    enable: true,
                    continuous_wave_mode: false,
                    payload_pattern: 0xAAA,
                    cs_mode: false,
                    active_subchannel: ActiveSubchannel::Both,
                    tx_rate: 1,
                }),
            ]
        )
    }

    #[test]
    fn deserialize_sequence_file_with_variables_overwrite() {
        let input = r#"
        variables:
          channel: 1
          band: ghz2_4
          bandwidth: bw20
          modulation: cck
          tx_rate: 1
          radio0: 11
          radio1: 0
          power_dbm: -1
          tx_path: a

        steps:
          - radio_mode:
              radio0: ${radio0}
              radio1: ${radio1}

          - rf_band:
              band: ${band}

          - channel_bandwidth:
              bandwidth: ${bandwidth}

          - rf_channel:
              channel: ${channel}

          - tx_power:
              power_dbm: ${power_dbm}
              modulation: ${modulation}
              path: ${tx_path}

          - tx_continuous:
              enable: true
              continuous_wave_mode: false
              payload_pattern: 0xAAA
              cs_mode: false
              active_subchannel: both
              tx_rate: ${tx_rate}
        "#;

        let cli_variables = &[
            ("channel".to_owned(), "23".to_owned()),
            ("band".to_owned(), "ghz5".to_owned()),
            ("bandwidth".to_owned(), "bw80".to_owned()),
            ("modulation".to_owned(), "mcs".to_owned()),
            ("tx_rate".to_owned(), "42".to_owned()),
            ("radio0".to_owned(), "0".to_owned()),
            ("radio1".to_owned(), "11".to_owned()),
            ("power_dbm".to_owned(), "64".to_owned()),
            ("tx_path".to_owned(), "a_b".to_owned()),
        ];

        let steps =
            parse_sequence_file(input, cli_variables).expect("failed to deserialize sequence file");

        assert_eq!(
            steps,
            vec![
                SequenceStep::Command(MfgCmd::RadioMode {
                    radio0: 0,
                    radio1: 11
                }),
                SequenceStep::Command(MfgCmd::RfBand { band: RfBand::Ghz5 }),
                SequenceStep::Command(MfgCmd::ChannelBandwidth {
                    bandwidth: ChannelBandwidth::Bw80
                }),
                SequenceStep::Command(MfgCmd::RfChannel { channel: 23 }),
                SequenceStep::Command(MfgCmd::TxPower {
                    power_dbm: 64,
                    modulation: Modulation::Mcs,
                    path: TxPathId::AB,
                }),
                SequenceStep::Command(MfgCmd::TxContinuous {
                    enable: true,
                    continuous_wave_mode: false,
                    payload_pattern: 0xAAA,
                    cs_mode: false,
                    active_subchannel: ActiveSubchannel::Both,
                    tx_rate: 42,
                }),
            ]
        )
    }

    #[test]
    fn parse_and_validate_examples() {
        fn process_yaml_files<T: FnMut(&PathBuf)>(dir: &std::path::Path, validate_fn: &mut T) {
            for entry in std::fs::read_dir(dir)
                .unwrap_or_else(|e| panic!("failed to read {dir:?}: {e}"))
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                if path.is_dir() {
                    process_yaml_files(&path, validate_fn);
                } else if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                    validate_fn(&path);
                }
            }
        }

        process_yaml_files(Path::new("examples"), &mut |path| {
            let contents =
                read_to_string(path).unwrap_or_else(|e| panic!("failed to read {path:?}: {e}"));
            let _ = parse_sequence_file(&contents, &[])
                .unwrap_or_else(|e| panic!("failed to deserialize {path:?}: {e}"));
        });
    }
}
