// SPDX-FileCopyrightText: 2026 The mwifiex_nl80211_test_mode authors
// SPDX-License-Identifier: GPL-2.0-only

use std::error::Error;
use std::fmt;
use std::str::FromStr;

use inquire::{Confirm, CustomType, InquireError, Select, Text};

use crate::command::{
    ActiveSubchannel, AntennaMode, ChannelBandwidth, LtfSymbol, MfgCmd, Modulation, RfBand,
    SignalBandwidth, SpatialStreamAllocation, StandaloneHeTbMode, TxPathId, UlBandwidth,
};
use crate::ffi::{CAL_DATA_LEN, MAC_ADDR_LENGTH};
use crate::netlink::MwifiexNetlinkInterface;
use crate::util::parse_mac_addr;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum CommandChoice {
    RfTestMode,
    TxAntenna,
    RxAntenna,
    RadioMode,
    RfBand,
    ChannelBandwidth,
    RfChannel,
    GetAndResetPer,
    TxPower,
    TxContinuous,
    TxFrame,
    TriggerFrame,
    HeTbTx,
    OtpCalData,
    OtpMacAddr,
    Exit,
}

impl CommandChoice {
    fn all() -> Vec<Self> {
        vec![
            Self::RfTestMode,
            Self::TxAntenna,
            Self::RxAntenna,
            Self::RadioMode,
            Self::RfBand,
            Self::ChannelBandwidth,
            Self::RfChannel,
            Self::GetAndResetPer,
            Self::TxPower,
            Self::TxContinuous,
            Self::TxFrame,
            Self::TriggerFrame,
            Self::HeTbTx,
            Self::OtpCalData,
            Self::OtpMacAddr,
            Self::Exit,
        ]
    }

    fn ask_for_confirmation(&self) -> bool {
        matches!(self, Self::OtpCalData | Self::OtpMacAddr)
    }
}

impl fmt::Display for CommandChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::RfTestMode => "rf_test_mode",
            Self::TxAntenna => "tx_antenna",
            Self::RxAntenna => "rx_antenna",
            Self::RadioMode => "radio_mode",
            Self::RfBand => "rf_band",
            Self::ChannelBandwidth => "channel_bandwidth",
            Self::RfChannel => "rf_channel",
            Self::GetAndResetPer => "get_and_reset_per",
            Self::TxPower => "tx_power",
            Self::TxContinuous => "tx_continuous",
            Self::TxFrame => "tx_frame",
            Self::TriggerFrame => "trigger_frame",
            Self::HeTbTx => "he_tb_tx",
            Self::OtpCalData => "otp_cal_data",
            Self::OtpMacAddr => "otp_mac_addr",
            Self::Exit => "exit",
        };
        write!(f, "{text}")
    }
}

const ANTENNA_CHOICES: &[(&str, AntennaMode)] = &[
    ("A", AntennaMode::A),
    ("B", AntennaMode::B),
    ("AB", AntennaMode::AB),
];

const RF_BAND_CHOICES: &[(&str, RfBand)] = &[("2.4 GHz", RfBand::Ghz2_4), ("5 GHz", RfBand::Ghz5)];

const BANDWIDTH_CHOICES: &[(&str, ChannelBandwidth)] = &[
    ("20 MHz", ChannelBandwidth::Bw20),
    ("40 MHz", ChannelBandwidth::Bw40),
    ("80 MHz", ChannelBandwidth::Bw80),
];

const MODULATION_CHOICES: &[(&str, Modulation)] = &[
    ("CCK", Modulation::Cck),
    ("OFDM", Modulation::Ofdm),
    ("MCS", Modulation::Mcs),
];

const TX_PATH_CHOICES: &[(&str, TxPathId)] =
    &[("A", TxPathId::A), ("B", TxPathId::B), ("AB", TxPathId::AB)];

const ACTIVE_SUBCHANNEL_CHOICES: &[(&str, ActiveSubchannel)] = &[
    ("Lower", ActiveSubchannel::Lower),
    ("Upper", ActiveSubchannel::Upper),
    ("Both", ActiveSubchannel::Both),
];

const SIGNAL_BW_CHOICES: &[(&str, SignalBandwidth)] = &[
    ("Default", SignalBandwidth::Default),
    ("20 MHz", SignalBandwidth::Bw20),
    ("40 MHz", SignalBandwidth::Bw40),
    ("80 MHz", SignalBandwidth::Bw80),
];

const STANDALONE_HETB_CHOICES: &[(&str, StandaloneHeTbMode)] = &[
    ("Disabled", StandaloneHeTbMode::Disabled),
    ("Trigger-based", StandaloneHeTbMode::TriggerBased),
    ("Standalone", StandaloneHeTbMode::Standalone),
    ("SU-OFDMA", StandaloneHeTbMode::SuOfdma),
];

const UL_BW_CHOICES: &[(&str, UlBandwidth)] = &[
    ("20 MHz", UlBandwidth::Bw20),
    ("40 MHz", UlBandwidth::Bw40),
    ("80 MHz", UlBandwidth::Bw80),
];

const LTF_SYMBOL_CHOICES: &[(&str, LtfSymbol)] = &[
    ("1x HELTF (1SS)", LtfSymbol::OneXHELTF),
    ("2x HELTF (2SS)", LtfSymbol::TwoXHELTF),
];

const SS_ALLOC_CHOICES: &[(&str, SpatialStreamAllocation)] = &[
    ("1 SS", SpatialStreamAllocation::OneSS),
    ("2 SS", SpatialStreamAllocation::TwoSS),
];

pub fn run_interactive(handle: &dyn MwifiexNetlinkInterface) -> Result<(), Box<dyn Error>> {
    loop {
        let choice = match Select::new("Choose command to send", CommandChoice::all()).prompt() {
            Ok(CommandChoice::Exit) => break,
            Ok(choice) => {
                if choice.ask_for_confirmation()
                    && !prompt_confirm(
                        &format!(
                            "{choice} is a potentially hardware altering operation, do you want to continue?"
                        ),
                        false,
                    )?
                {
                    continue;
                }
                choice
            }
            Err(InquireError::OperationCanceled) | Err(InquireError::OperationInterrupted) => {
                println!("Interactive mode canceled.");
                return Ok(());
            }
            Err(err) => return Err(err.into()),
        };

        let cmd = choice.try_into()?;
        match handle.send_mfg_cmd(&cmd) {
            Ok(resp) => println!("Sent command {cmd:?} => Response: {resp:?}"),
            Err(err) => println!("Failed to send command: {err}"),
        }

        if !prompt_confirm("Send another command?", true)? {
            break;
        }
    }

    Ok(())
}

impl TryInto<MfgCmd> for CommandChoice {
    type Error = Box<dyn Error>;

    fn try_into(self) -> Result<MfgCmd, Self::Error> {
        Ok(match self {
            CommandChoice::RfTestMode => MfgCmd::RfTestMode {
                enable: prompt_confirm("Enable RF test mode?", true)?,
            },
            CommandChoice::TxAntenna => MfgCmd::TxAntenna {
                mode: prompt_choice("TX antenna mode", ANTENNA_CHOICES)?,
            },
            CommandChoice::RxAntenna => MfgCmd::RxAntenna {
                mode: prompt_choice("RX antenna mode", ANTENNA_CHOICES)?,
            },
            CommandChoice::RadioMode => MfgCmd::RadioMode {
                radio0: prompt("radio0", 1)?,
                radio1: prompt("radio1", 11)?,
            },
            CommandChoice::RfBand => MfgCmd::RfBand {
                band: prompt_choice("RF band", RF_BAND_CHOICES)?,
            },
            CommandChoice::ChannelBandwidth => MfgCmd::ChannelBandwidth {
                bandwidth: prompt_choice("Channel bandwidth", BANDWIDTH_CHOICES)?,
            },
            CommandChoice::RfChannel => MfgCmd::RfChannel {
                channel: prompt("RF channel", 36)?,
            },
            CommandChoice::GetAndResetPer => MfgCmd::GetAndResetPer {
                rx_total_packet_count: 0,
                rx_multi_broadcast_packet_count: 0,
                rx_frame_check_sequence_errors: 0,
            },
            CommandChoice::TxPower => MfgCmd::TxPower {
                power_dbm: prompt("TX power (dBm)", 16)?,
                modulation: prompt_choice("Modulation", MODULATION_CHOICES)?,
                path: prompt_choice("TX path", TX_PATH_CHOICES)?,
            },
            CommandChoice::TxContinuous => MfgCmd::TxContinuous {
                enable: prompt_confirm("Start transmit?", true)?,
                continuous_wave_mode: prompt_confirm("Enable continuous wave mode?", false)?,
                payload_pattern: prompt("Payload pattern", 2730)?,
                cs_mode: prompt_confirm("Enable carrier suppression mode?", false)?,
                active_subchannel: prompt_choice("Active subchannel", ACTIVE_SUBCHANNEL_CHOICES)?,
                tx_rate: prompt("TX rate", 8)?,
            },
            CommandChoice::TxFrame => MfgCmd::TxFrame {
                enable: prompt_confirm("Start transmit?", true)?,
                tx_rate: prompt("TX rate", 8)?,
                payload_pattern: prompt("Payload pattern", 2730)?,
                payload_length: prompt("Payload length", 598)?,
                adjust_burst_sifs_gap: prompt_confirm("Adjust burst SIFS gap?", false)?,
                adjust_sifs_us: prompt("Adjust SIFS (us)", 20)?,
                short_preamble: prompt_confirm("Enable short preamble?", false)?,
                active_subchannel: prompt_choice("Active subchannel", ACTIVE_SUBCHANNEL_CHOICES)?,
                short_gi: prompt_confirm("Enable short GI?", false)?,
                adv_coding: prompt_confirm("Enable advanced coding?", false)?,
                beamforming: prompt_confirm("Enable beamforming?", false)?,
                greenfield_mode: prompt_confirm("Enable greenfield mode?", false)?,
                stbc: prompt_confirm("Enable STBC?", false)?,
                signal_bw: prompt_choice("Signal BW", SIGNAL_BW_CHOICES)?,
                num_pkt: prompt("Number of packets", -1)?,
                max_pkt_ext: prompt("Max packet extension", -1)?,
                beam_change: prompt("Beam change", -1)?,
                dcm: prompt("DCM", -1)?,
                doppler: prompt("Doppler", -1)?,
                midamble_period: prompt("Midamble period", -1)?,
                q_num: prompt("Queue number", -1)?,
                bssid: prompt_mac_addr("BSSID", "05:43:3f:c4:51:00")?,
            },
            CommandChoice::TriggerFrame => MfgCmd::TriggerFrame {
                enable: prompt_confirm("Enable TX?", true)?,
                standalone_hetb: prompt_choice("Standalone HETB mode", STANDALONE_HETB_CHOICES)?,
                frame_ctrl_type: prompt("Frame control type", 1)?,
                frame_ctrl_subtype: prompt("Frame control subtype", 2)?,
                frame_duration: prompt("Frame duration", 5484)?,
                trigger_type: prompt("Trigger type", 0)?,
                ul_len: prompt("UL length", 1000)?,
                more_tf: prompt_confirm("More TF?", false)?,
                cs_required: prompt_confirm("CS required?", true)?,
                ul_bw: prompt_choice("UL bandwidth", UL_BW_CHOICES)?,
                ltf_type: prompt("LTF type", 1)?,
                ltf_mode: prompt_confirm("Enable LTF mode?", false)?,
                ltf_symbol: prompt_choice("LTF symbol", LTF_SYMBOL_CHOICES)?,
                ul_stbc: prompt_confirm("Enable UL STBC?", false)?,
                ldpc_ess: prompt_confirm("Enable LDPC ESS?", true)?,
                ap_tx_pwr: prompt("AP TX power", 0)?,
                pre_fec_pad_fct: prompt("Pre-FEC pad factor", 1)?,
                pe_disambig: prompt_confirm("Enable PE disambiguity?", false)?,
                spatial_reuse: prompt("Spatial reuse", 65535)?,
                doppler: prompt_confirm("Enable doppler?", false)?,
                he_sig2: prompt("HE-SIG2", 511)?,
                aid12: prompt("AID12", 5)?,
                ru_alloc_reg: prompt_confirm("Enable RU alloc reg?", false)?,
                ru_alloc: prompt("RU alloc", 106)?,
                ul_coding_type: prompt_confirm("Enable UL coding type?", true)?,
                ul_mcs: prompt("UL MCS", 0)?,
                ul_dcm: prompt_confirm("Enable UL DCM?", false)?,
                ss_alloc: prompt_choice("Spatial stream allocation", SS_ALLOC_CHOICES)?,
                ul_target_rssi: prompt("UL target RSSI", 90)?,
                mpdu_mu_sf: prompt("MPDU MU SF", 0)?,
                tid_al: prompt("TID allocation", 0)?,
                ac_pl: prompt_confirm("Enable AC policy?", false)?,
                pref_ac: prompt("Preferred AC", 0)?,
            },
            CommandChoice::HeTbTx => MfgCmd::HeTbTx {
                enable: prompt_confirm("Enable trigger response?", true)?,
                qnum: prompt("Queue number", 1)?,
                aid: prompt("AID", 5)?,
                axq0_mu_timer: prompt("AXQ0 MU timer", 400)?,
                tx_pwr: prompt("TX power", 9)?,
            },
            CommandChoice::OtpCalData => {
                let write = prompt_confirm("Write OTP calibration data?", false)?;
                let cal_data = if write {
                    prompt_hex_data(
                        &format!("Calibration data as hex bytes (max {CAL_DATA_LEN} bytes)"),
                        CAL_DATA_LEN,
                    )?
                } else {
                    Vec::new()
                };
                MfgCmd::OtpCalData { write, cal_data }
            }
            CommandChoice::OtpMacAddr => {
                let write = prompt_confirm("Write OTP MAC address?", false)?;
                let default_mac = if write {
                    "00:11:22:33:44:55"
                } else {
                    "00:00:00:00:00:00"
                };
                MfgCmd::OtpMacAddr {
                    write,
                    mac_addr: prompt_mac_addr("MAC address", default_mac)?,
                }
            }
            CommandChoice::Exit => return Err("Exit is not a command".into()),
        })
    }
}

fn prompt_choice<T>(message: &str, choices: &[(&str, T)]) -> Result<T, Box<dyn Error>>
where
    T: Copy,
{
    let options = choices.iter().map(|(key, _)| *key).collect();
    let choice = Select::new(message, options).prompt()?;

    for (key, val) in choices {
        if *key == choice {
            return Ok(*val);
        }
    }

    unreachable!()
}

fn prompt<T>(message: &str, default: T) -> Result<T, Box<dyn Error>>
where
    T: Clone + FromStr + ToString,
{
    Ok(CustomType::<T>::new(message)
        .with_default(default)
        .prompt()?)
}

fn prompt_confirm(message: &str, default: bool) -> Result<bool, Box<dyn Error>> {
    Ok(Confirm::new(message).with_default(default).prompt()?)
}

fn prompt_mac_addr(message: &str, default: &str) -> Result<[u8; MAC_ADDR_LENGTH], Box<dyn Error>> {
    let input = Text::new(message).with_default(default).prompt()?;
    parse_mac_addr(&input)
}

fn prompt_hex_data(message: &str, length: usize) -> Result<Vec<u8>, Box<dyn Error>> {
    let input = Text::new(message)
        .with_help_message("Example: aa55ff00 or aa:55:ff:00")
        .prompt()?;

    let cleaned: String = input.chars().filter(|ch| ch.is_ascii_hexdigit()).collect();

    if cleaned.is_empty() {
        return Err("Hex input is empty".into());
    }

    if !cleaned.len().is_multiple_of(2) {
        return Err("Hex input must contain an even number of digits".into());
    }

    let bytes = cleaned
        .as_bytes()
        .chunks(2)
        .map(|chunk| {
            let segment = std::str::from_utf8(chunk)?;
            u8::from_str_radix(segment, 16)
                .map_err(|err| format!("Invalid hex byte '{segment}': {err}").into())
        })
        .collect::<Result<Vec<u8>, Box<dyn Error>>>()?;

    if bytes.len() > length {
        return Err(format!(
            "Provided data too long: {} bytes (max {length})",
            bytes.len()
        )
        .into());
    }

    Ok(bytes)
}
