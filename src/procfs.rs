// SPDX-FileCopyrightText: 2026 The mwifiex_nl80211_test_mode authors
// SPDX-License-Identifier: GPL-2.0-only

use std::error::Error;
use std::str::FromStr;

use crate::command::{
    ActiveSubchannel, AntennaMode, ChannelBandwidth, LtfSymbol, MfgCmd, Modulation, RfBand,
    SignalBandwidth, SpatialStreamAllocation, StandaloneHeTbMode, TxPathId, UlBandwidth,
};
use crate::ffi::MAC_ADDR_LENGTH;
use crate::netlink::MwifiexNetlinkInterface;
use crate::util::{format_request_response, parse_mac_addr};

pub fn run_procfs_command(
    handle: &dyn MwifiexNetlinkInterface,
    input: &str,
) -> Result<(), Box<dyn Error>> {
    let cmd = parse_procfs_command(input)?;
    let response = handle.send_mfg_cmd(&cmd)?;

    println!("{}", format_request_response(&cmd, &response));

    Ok(())
}

/// Parse a legacy procfs command string into a [`MfgCmd`].
pub fn parse_procfs_command(input: &str) -> Result<MfgCmd, Box<dyn Error>> {
    let input = input.trim();

    // Commands without '=' (query commands)
    if input == "get_and_reset_per" {
        return Ok(MfgCmd::GetAndResetPer {
            rx_total_packet_count: 0,
            rx_multi_broadcast_packet_count: 0,
            rx_frame_check_sequence_errors: 0,
        });
    }

    // Commands with '=' separator
    let (key, value) = input
        .split_once('=')
        .ok_or_else(|| format!("invalid procfs command: missing '=' in '{input}'"))?;

    let key = key.trim();
    let tokens: Vec<&str> = value.split_whitespace().collect();

    let cmd = match key {
        "rf_test_mode" => MfgCmd::RfTestMode {
            enable: parse::<u32>(&tokens, 0, "enable")? != 0,
        },
        "tx_antenna" => MfgCmd::TxAntenna {
            mode: AntennaMode::try_from(parse::<u32>(&tokens, 0, "mode")?)?,
        },
        "rx_antenna" => MfgCmd::RxAntenna {
            mode: AntennaMode::try_from(parse::<u32>(&tokens, 0, "mode")?)?,
        },
        "radio_mode" => MfgCmd::RadioMode {
            radio0: parse(&tokens, 0, "radio0")?,
            radio1: parse(&tokens, 1, "radio1")?,
        },
        "band" => MfgCmd::RfBand {
            band: RfBand::try_from(parse::<u32>(&tokens, 0, "band")?)?,
        },
        "bw" => MfgCmd::ChannelBandwidth {
            bandwidth: ChannelBandwidth::try_from(parse::<u32>(&tokens, 0, "bandwidth")?)?,
        },
        "channel" => MfgCmd::RfChannel {
            channel: parse(&tokens, 0, "channel")?,
        },
        "tx_power" => MfgCmd::TxPower {
            power_dbm: parse(&tokens, 0, "power")?,
            modulation: Modulation::try_from(parse::<u32>(&tokens, 1, "modulation")?)?,
            path: TxPathId::try_from(parse::<u32>(&tokens, 2, "path_id")?)?,
        },
        "tx_continuous" => MfgCmd::TxContinuous {
            enable: parse::<u32>(&tokens, 0, "start/stop")? != 0,
            continuous_wave_mode: parse_or_default::<u32>(&tokens, 1, "continuous_wave_mode") != 0,
            payload_pattern: parse_or_default(&tokens, 2, "payload_pattern"),
            cs_mode: parse_or_default::<u32>(&tokens, 3, "cs_mode") != 0,
            active_subchannel: ActiveSubchannel::try_from(parse_or_default::<u32>(
                &tokens,
                4,
                "active_subchannel",
            ))?,
            tx_rate: parse_or_default(&tokens, 5, "tx_rate"),
        },
        "tx_frame" => MfgCmd::TxFrame {
            enable: parse::<u32>(&tokens, 0, "start/stop")? != 0,
            tx_rate: parse_or_default(&tokens, 1, "tx_rate"),
            payload_pattern: parse_or_default(&tokens, 2, "payload_pattern"),
            payload_length: parse_or_default(&tokens, 3, "payload_length"),
            adjust_burst_sifs_gap: parse_or_default::<u32>(&tokens, 4, "adjust_burst_sifs_gap")
                != 0,
            adjust_sifs_us: parse_or_default(&tokens, 5, "adjust_sifs_us"),
            short_preamble: parse_or_default::<u32>(&tokens, 6, "short_preamble") != 0,
            active_subchannel: ActiveSubchannel::try_from(parse_or_default::<u32>(
                &tokens,
                7,
                "active_subchannel",
            ))?,
            short_gi: parse_or_default::<u32>(&tokens, 8, "short_gi") != 0,
            adv_coding: parse_or_default::<u32>(&tokens, 9, "adv_coding") != 0,
            beamforming: parse_or_default::<u32>(&tokens, 10, "beamforming") != 0,
            greenfield_mode: parse_or_default::<u32>(&tokens, 11, "greenfield_mode") != 0,
            stbc: parse_or_default::<u32>(&tokens, 12, "stbc") != 0,
            signal_bw: SignalBandwidth::try_from(parse_or(
                &tokens,
                13,
                "signal_bw",
                0xFFFF_FFFFu32,
            ))?,
            num_pkt: parse_or(&tokens, 14, "num_pkt", -1i32),
            max_pkt_ext: parse_or(&tokens, 15, "max_pkt_ext", -1i32),
            beam_change: parse_or(&tokens, 16, "beam_change", -1i32),
            dcm: parse_or(&tokens, 17, "dcm", -1i32),
            doppler: parse_or(&tokens, 18, "doppler", -1i32),
            midamble_period: parse_or(&tokens, 19, "midamble_period", -1i32),
            q_num: parse_or(&tokens, 20, "q_num", -1i32),
            bssid: match tokens.get(21) {
                Some(s) => parse_mac_addr(s)?,
                None => [0u8; MAC_ADDR_LENGTH],
            },
        },
        "trigger_frame" => MfgCmd::TriggerFrame {
            enable: parse::<u32>(&tokens, 0, "enable_TX")? != 0,
            standalone_hetb: StandaloneHeTbMode::try_from(parse_or_default::<u32>(
                &tokens,
                1,
                "standalone_hetb",
            ))?,
            frame_ctrl_type: parse_or_default(&tokens, 2, "frame_ctrl_type"),
            frame_ctrl_subtype: parse_or_default(&tokens, 3, "frame_ctrl_subtype"),
            frame_duration: parse_or_default(&tokens, 4, "frame_duration"),
            trigger_type: parse_or_default(&tokens, 5, "trigger_type"),
            ul_len: parse_or_default(&tokens, 6, "ul_len"),
            more_tf: parse_or_default::<u32>(&tokens, 7, "more_tf") != 0,
            cs_required: parse_or_default::<u32>(&tokens, 8, "cs_required") != 0,
            ul_bw: UlBandwidth::try_from(parse_or_default::<u64>(&tokens, 9, "ul_bw"))?,
            ltf_type: parse_or_default(&tokens, 10, "ltf_type"),
            ltf_mode: parse_or_default::<u32>(&tokens, 11, "ltf_mode") != 0,
            ltf_symbol: LtfSymbol::try_from(parse_or_default::<u64>(&tokens, 12, "ltf_symbol"))?,
            ul_stbc: parse_or_default::<u32>(&tokens, 13, "ul_stbc") != 0,
            ldpc_ess: parse_or_default::<u32>(&tokens, 14, "ldpc_ess") != 0,
            ap_tx_pwr: parse_or_default(&tokens, 15, "ap_tx_pwr"),
            pre_fec_pad_fct: parse_or_default(&tokens, 16, "pre_fec_pad_fct"),
            pe_disambig: parse_or_default::<u32>(&tokens, 17, "pe_disambig") != 0,
            spatial_reuse: parse_or_default(&tokens, 18, "spatial_reuse"),
            doppler: parse_or_default::<u32>(&tokens, 19, "doppler") != 0,
            he_sig2: parse_or_default(&tokens, 20, "he_sig2"),
            aid12: parse_or_default(&tokens, 21, "aid12"),
            ru_alloc_reg: parse_or_default::<u32>(&tokens, 22, "ru_alloc_reg") != 0,
            ru_alloc: parse_or_default(&tokens, 23, "ru_alloc"),
            ul_coding_type: parse_or_default::<u32>(&tokens, 24, "ul_coding_type") != 0,
            ul_mcs: parse_or_default(&tokens, 25, "ul_mcs"),
            ul_dcm: parse_or_default::<u32>(&tokens, 26, "ul_dcm") != 0,
            ss_alloc: SpatialStreamAllocation::try_from(parse_or_default::<u32>(
                &tokens, 27, "ss_alloc",
            ))?,
            ul_target_rssi: parse_or_default(&tokens, 28, "ul_target_rssi"),
            mpdu_mu_sf: parse_or_default(&tokens, 29, "mpdu_mu_sf"),
            tid_al: parse_or_default(&tokens, 30, "tid_al"),
            ac_pl: parse_or_default::<u32>(&tokens, 31, "ac_pl") != 0,
            pref_ac: parse_or_default(&tokens, 32, "pref_ac"),
        },
        "he_tb_tx" => MfgCmd::HeTbTx {
            enable: parse::<u32>(&tokens, 0, "enable")? != 0,
            qnum: parse_or_default(&tokens, 1, "qnum"),
            aid: parse_or_default(&tokens, 2, "aid"),
            axq0_mu_timer: parse_or_default(&tokens, 3, "axq0_mu_timer"),
            tx_pwr: parse_or_default(&tokens, 4, "tx_pwr"),
        },
        _ => return Err(format!("unknown procfs command: '{key}={value}'").into()),
    };

    Ok(cmd)
}

/// Get a token at a specific index, or return an error.
fn parse<T: FromStr>(tokens: &[&str], idx: usize, name: &str) -> Result<T, Box<dyn Error>> {
    tokens
        .get(idx)
        .ok_or_else(|| format!("missing argument {name} at position {idx}").into())
        .and_then(|value| {
            if let Some(hex) = value.to_ascii_lowercase().strip_prefix("0x") {
                if let Ok(hex) = u64::from_str_radix(hex, 16) {
                    T::from_str(&hex.to_string())
                } else {
                    return Err(format!(
                        "invalid hex string: {value} for argument {name} at position {idx}"
                    )
                    .into());
                }
            } else {
                T::from_str(value)
            }
            .map_err(|_| {
                format!("invalid value: {value} for argument {name} at position {idx}").into()
            })
        })
}

fn parse_or<T: FromStr>(tokens: &[&str], idx: usize, name: &str, default: T) -> T {
    parse(tokens, idx, name).unwrap_or(default)
}

fn parse_or_default<T: FromStr + Default>(tokens: &[&str], idx: usize, name: &str) -> T {
    parse_or(tokens, idx, name, T::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_rf_test_mode_enable() {
        let cmd = parse_procfs_command("rf_test_mode=1").unwrap();
        assert_eq!(cmd, MfgCmd::RfTestMode { enable: true });
    }

    #[test]
    fn parse_rf_test_mode_disable() {
        let cmd = parse_procfs_command("rf_test_mode=0").unwrap();
        assert_eq!(cmd, MfgCmd::RfTestMode { enable: false });
    }

    #[test]
    fn parse_tx_antenna_path_a() {
        let cmd = parse_procfs_command("tx_antenna=1").unwrap();
        assert_eq!(
            cmd,
            MfgCmd::TxAntenna {
                mode: AntennaMode::A
            }
        );
    }

    #[test]
    fn parse_tx_antenna_path_ab() {
        let cmd = parse_procfs_command("tx_antenna=3").unwrap();
        assert_eq!(
            cmd,
            MfgCmd::TxAntenna {
                mode: AntennaMode::AB
            }
        );
    }

    #[test]
    fn parse_rx_antenna_path_b() {
        let cmd = parse_procfs_command("rx_antenna=2").unwrap();
        assert_eq!(
            cmd,
            MfgCmd::RxAntenna {
                mode: AntennaMode::B
            }
        );
    }

    #[test]
    fn parse_radio_mode_two_radios() {
        let cmd = parse_procfs_command("radio_mode=11 11").unwrap();
        assert_eq!(
            cmd,
            MfgCmd::RadioMode {
                radio0: 11,
                radio1: 11
            }
        );
    }

    #[test]
    fn parse_band_2_4_ghz() {
        let cmd = parse_procfs_command("band=0").unwrap();
        assert_eq!(
            cmd,
            MfgCmd::RfBand {
                band: RfBand::Ghz2_4
            }
        );
    }

    #[test]
    fn parse_band_5_ghz() {
        let cmd = parse_procfs_command("band=1").unwrap();
        assert_eq!(cmd, MfgCmd::RfBand { band: RfBand::Ghz5 });
    }

    #[test]
    fn parse_bw_20() {
        let cmd = parse_procfs_command("bw=0").unwrap();
        assert_eq!(
            cmd,
            MfgCmd::ChannelBandwidth {
                bandwidth: ChannelBandwidth::Bw20
            }
        );
    }

    #[test]
    fn parse_bw_40() {
        let cmd = parse_procfs_command("bw=1").unwrap();
        assert_eq!(
            cmd,
            MfgCmd::ChannelBandwidth {
                bandwidth: ChannelBandwidth::Bw40
            }
        );
    }

    #[test]
    fn parse_channel_6() {
        let cmd = parse_procfs_command("channel=6").unwrap();
        assert_eq!(cmd, MfgCmd::RfChannel { channel: 6 });
    }

    #[test]
    fn parse_channel_36() {
        let cmd = parse_procfs_command("channel=36").unwrap();
        assert_eq!(cmd, MfgCmd::RfChannel { channel: 36 });
    }

    #[test]
    fn parse_get_and_reset_per() {
        let cmd = parse_procfs_command("get_and_reset_per").unwrap();
        assert_eq!(
            cmd,
            MfgCmd::GetAndResetPer {
                rx_total_packet_count: 0,
                rx_multi_broadcast_packet_count: 0,
                rx_frame_check_sequence_errors: 0,
            }
        );
    }

    #[test]
    fn parse_tx_power_full() {
        let cmd = parse_procfs_command("tx_power=15 1 0").unwrap();
        assert_eq!(
            cmd,
            MfgCmd::TxPower {
                power_dbm: 15,
                modulation: Modulation::Ofdm,
                path: TxPathId::A,
            }
        );
    }

    #[test]
    fn parse_tx_continuous_start() {
        let cmd = parse_procfs_command("tx_continuous=1 0 0 0 0 7").unwrap();
        assert_eq!(
            cmd,
            MfgCmd::TxContinuous {
                enable: true,
                continuous_wave_mode: false,
                payload_pattern: 0,
                cs_mode: false,
                active_subchannel: ActiveSubchannel::Lower,
                tx_rate: 7,
            }
        );
    }

    #[test]
    fn parse_tx_continuous_stop() {
        let cmd = parse_procfs_command("tx_continuous=0").unwrap();
        assert_eq!(
            cmd,
            MfgCmd::TxContinuous {
                enable: false,
                continuous_wave_mode: false,
                payload_pattern: 0,
                cs_mode: false,
                active_subchannel: ActiveSubchannel::Lower,
                tx_rate: 0,
            }
        );
    }

    #[test]
    fn parse_tx_continuous_cw_mode() {
        let cmd = parse_procfs_command("tx_continuous=1 1 0 0 0 0").unwrap();
        assert_eq!(
            cmd,
            MfgCmd::TxContinuous {
                enable: true,
                continuous_wave_mode: true,
                payload_pattern: 0,
                cs_mode: false,
                active_subchannel: ActiveSubchannel::Lower,
                tx_rate: 0,
            }
        );
    }

    #[test]
    fn parse_tx_frame_basic() {
        let cmd = parse_procfs_command("tx_frame=1 7 0 1024").unwrap();
        assert_eq!(
            cmd,
            MfgCmd::TxFrame {
                enable: true,
                tx_rate: 7,
                payload_pattern: 0,
                payload_length: 1024,
                adjust_burst_sifs_gap: false,
                adjust_sifs_us: 0,
                short_preamble: false,
                active_subchannel: ActiveSubchannel::Lower,
                short_gi: false,
                adv_coding: false,
                beamforming: false,
                greenfield_mode: false,
                stbc: false,
                signal_bw: SignalBandwidth::Default,
                num_pkt: -1,
                max_pkt_ext: -1,
                beam_change: -1,
                dcm: -1,
                doppler: -1,
                midamble_period: -1,
                q_num: -1,
                bssid: [0; MAC_ADDR_LENGTH],
            }
        );
    }

    #[test]
    fn parse_tx_frame_stop() {
        let cmd = parse_procfs_command("tx_frame=0").unwrap();
        assert_eq!(
            cmd,
            MfgCmd::TxFrame {
                enable: false,
                tx_rate: 0,
                payload_pattern: 0,
                payload_length: 0,
                adjust_burst_sifs_gap: false,
                adjust_sifs_us: 0,
                short_preamble: false,
                active_subchannel: ActiveSubchannel::Lower,
                short_gi: false,
                adv_coding: false,
                beamforming: false,
                greenfield_mode: false,
                stbc: false,
                signal_bw: SignalBandwidth::Default,
                num_pkt: -1,
                max_pkt_ext: -1,
                beam_change: -1,
                dcm: -1,
                doppler: -1,
                midamble_period: -1,
                q_num: -1,
                bssid: [0; MAC_ADDR_LENGTH],
            }
        );
    }

    #[test]
    fn parse_tx_frame_with_bssid() {
        let cmd = parse_procfs_command(
            "tx_frame=1 12 0 1024 0 0 0 0 0 0 0 0 0 0 10000 0 0 0 0 0 0 00:11:22:33:44:55",
        )
        .unwrap();
        assert_eq!(
            cmd,
            MfgCmd::TxFrame {
                enable: true,
                tx_rate: 12,
                payload_pattern: 0,
                payload_length: 1024,
                adjust_burst_sifs_gap: false,
                adjust_sifs_us: 0,
                short_preamble: false,
                active_subchannel: ActiveSubchannel::Lower,
                short_gi: false,
                adv_coding: false,
                beamforming: false,
                greenfield_mode: false,
                stbc: false,
                signal_bw: SignalBandwidth::Bw20,
                num_pkt: 10000,
                max_pkt_ext: 0,
                beam_change: 0,
                dcm: 0,
                doppler: 0,
                midamble_period: 0,
                q_num: 0,
                bssid: [0x00, 0x11, 0x22, 0x33, 0x44, 0x55],
            }
        );
    }

    #[test]
    fn parse_tx_frame_he_er_su() {
        // Example from AN14114: 5 GHz HE ER SU TX frame
        let cmd = parse_procfs_command("tx_frame=1 12 0 1024 0 0 0 0 0 1 0 0 0 4 0 2 0 0 0 0 -1")
            .unwrap();
        assert_eq!(
            cmd,
            MfgCmd::TxFrame {
                enable: true,
                tx_rate: 12,
                payload_pattern: 0,
                payload_length: 1024,
                adjust_burst_sifs_gap: false,
                adjust_sifs_us: 0,
                short_preamble: false,
                active_subchannel: ActiveSubchannel::Lower,
                short_gi: false,
                adv_coding: true,
                beamforming: false,
                greenfield_mode: false,
                stbc: false,
                signal_bw: SignalBandwidth::Bw80,
                num_pkt: 0,
                max_pkt_ext: 2,
                beam_change: 0,
                dcm: 0,
                doppler: 0,
                midamble_period: 0,
                q_num: -1,
                bssid: [0; MAC_ADDR_LENGTH],
            }
        );
    }

    #[test]
    fn parse_trigger_frame_basic() {
        let cmd = parse_procfs_command(
            "trigger_frame=1 0 1 2 5484 0 1000 0 0 0 1 0 0 0 0 0 1 0 65535 0 511 5 0 68 0 7 0 0 90 0 0 0 0",
        )
        .unwrap();
        assert_eq!(
            cmd,
            MfgCmd::TriggerFrame {
                enable: true,
                standalone_hetb: StandaloneHeTbMode::Disabled,
                frame_ctrl_type: 1,
                frame_ctrl_subtype: 2,
                frame_duration: 5484,
                trigger_type: 0,
                ul_len: 1000,
                more_tf: false,
                cs_required: false,
                ul_bw: UlBandwidth::Bw20,
                ltf_type: 1,
                ltf_mode: false,
                ltf_symbol: LtfSymbol::OneXHELTF,
                ul_stbc: false,
                ldpc_ess: false,
                ap_tx_pwr: 0,
                pre_fec_pad_fct: 1,
                pe_disambig: false,
                spatial_reuse: 65535,
                doppler: false,
                he_sig2: 511,
                aid12: 5,
                ru_alloc_reg: false,
                ru_alloc: 68,
                ul_coding_type: false,
                ul_mcs: 7,
                ul_dcm: false,
                ss_alloc: SpatialStreamAllocation::OneSS,
                ul_target_rssi: 90,
                mpdu_mu_sf: 0,
                tid_al: 0,
                ac_pl: false,
                pref_ac: 0,
            }
        );
    }

    #[test]
    fn parse_trigger_frame_disable() {
        let cmd = parse_procfs_command("trigger_frame=0").unwrap();

        assert_eq!(
            cmd,
            MfgCmd::TriggerFrame {
                enable: false,
                standalone_hetb: StandaloneHeTbMode::Disabled,
                frame_ctrl_type: 0,
                frame_ctrl_subtype: 0,
                frame_duration: 0,
                trigger_type: 0,
                ul_len: 0,
                more_tf: false,
                cs_required: false,
                ul_bw: UlBandwidth::Bw20,
                ltf_type: 0,
                ltf_mode: false,
                ltf_symbol: LtfSymbol::OneXHELTF,
                ul_stbc: false,
                ldpc_ess: false,
                ap_tx_pwr: 0,
                pre_fec_pad_fct: 0,
                pe_disambig: false,
                spatial_reuse: 0,
                doppler: false,
                he_sig2: 0,
                aid12: 0,
                ru_alloc_reg: false,
                ru_alloc: 0,
                ul_coding_type: false,
                ul_mcs: 0,
                ul_dcm: false,
                ss_alloc: SpatialStreamAllocation::OneSS,
                ul_target_rssi: 0,
                mpdu_mu_sf: 0,
                tid_al: 0,
                ac_pl: false,
                pref_ac: 0,
            }
        );
    }

    #[test]
    fn parse_he_tb_tx_enable() {
        let cmd = parse_procfs_command("he_tb_tx=1 1 5 400 20").unwrap();

        assert_eq!(
            cmd,
            MfgCmd::HeTbTx {
                enable: true,
                qnum: 1,
                aid: 5,
                axq0_mu_timer: 400,
                tx_pwr: 20,
            }
        );
    }

    #[test]
    fn parse_he_tb_tx_disable() {
        let cmd = parse_procfs_command("he_tb_tx=0").unwrap();

        assert_eq!(
            cmd,
            MfgCmd::HeTbTx {
                enable: false,
                qnum: 0,
                aid: 0,
                axq0_mu_timer: 0,
                tx_pwr: 0,
            }
        );
    }

    #[test]
    fn parse_whitespace_tolerance() {
        let cmd = parse_procfs_command("  channel=11  ").unwrap();
        assert_eq!(cmd, MfgCmd::RfChannel { channel: 11 });
    }

    // AN14114: "Set TX continuous mode" examples
    #[test]
    fn an14114_tx_continuous_packet_mode() {
        // tx_continuous=1 0 0xAAA 0 3 0x8 — continuous packet mode, 0xAAA pattern,
        // both sub-channels, 12 Mbps rate
        let cmd = parse_procfs_command("tx_continuous=1 0 0xAAA 0 3 0x8").unwrap();
        assert_eq!(
            cmd,
            MfgCmd::TxContinuous {
                enable: true,
                continuous_wave_mode: false,
                payload_pattern: 0xAAA,
                cs_mode: false,
                active_subchannel: ActiveSubchannel::Both,
                tx_rate: 8,
            }
        );
    }

    #[test]
    fn an14114_tx_continuous_wave_mode() {
        // tx_continuous=1 1 0xAAA 0 3 0x8 — start continuous wave mode
        let cmd = parse_procfs_command("tx_continuous=1 1 0xAAA 0 3 0x8").unwrap();
        assert_eq!(
            cmd,
            MfgCmd::TxContinuous {
                enable: true,
                continuous_wave_mode: true,
                payload_pattern: 0xAAA,
                cs_mode: false,
                active_subchannel: ActiveSubchannel::Both,
                tx_rate: 8,
            }
        );
    }

    #[test]
    fn an14114_tx_continuous_wave_mode_stop() {
        // tx_continuous=0 1 0xAAA 0 3 0x8 — stop continuous wave mode
        let cmd = parse_procfs_command("tx_continuous=0 1 0xAAA 0 3 0x8").unwrap();
        assert_eq!(
            cmd,
            MfgCmd::TxContinuous {
                enable: false,
                continuous_wave_mode: true,
                payload_pattern: 0xAAA,
                cs_mode: false,
                active_subchannel: ActiveSubchannel::Both,
                tx_rate: 8,
            }
        );
    }

    // AN14114: "2.4 GHz TX command sequence using TX_continuous"
    #[test]
    fn an14114_tx_continuous_2_4_ghz_he_mcs8() {
        // tx_continuous=1 1 0xAAA 0 3 0x2108 — CW mode, HE SS1 MCS8 rate
        let cmd = parse_procfs_command("tx_continuous=1 1 0xAAA 0 3 0x2108").unwrap();
        assert_eq!(
            cmd,
            MfgCmd::TxContinuous {
                enable: true,
                continuous_wave_mode: true,
                payload_pattern: 0xAAA,
                cs_mode: false,
                active_subchannel: ActiveSubchannel::Both,
                tx_rate: 0x2108,
            }
        );
    }

    // AN14114: "88W9098 5 GHz TX command sequence using tx_continuous"
    #[test]
    fn an14114_tx_continuous_88w9098_5_ghz_he_mcs9() {
        // tx_continuous=1 0 0xAAA 0 3 0x2109 — continuous packet mode, HE SS1 MCS9
        let cmd = parse_procfs_command("tx_continuous=1 0 0xAAA 0 3 0x2109").unwrap();
        assert_eq!(
            cmd,
            MfgCmd::TxContinuous {
                enable: true,
                continuous_wave_mode: false,
                payload_pattern: 0xAAA,
                cs_mode: false,
                active_subchannel: ActiveSubchannel::Both,
                tx_rate: 0x2109,
            }
        );
    }

    #[test]
    fn an14114_tx_continuous_88w9098_5_ghz_cw_mode() {
        // tx_continuous=1 1 0xAAA 0 3 0x2109 — CW mode, HE SS1 MCS9
        let cmd = parse_procfs_command("tx_continuous=1 1 0xAAA 0 3 0x2109").unwrap();
        assert_eq!(
            cmd,
            MfgCmd::TxContinuous {
                enable: true,
                continuous_wave_mode: true,
                payload_pattern: 0xAAA,
                cs_mode: false,
                active_subchannel: ActiveSubchannel::Both,
                tx_rate: 0x2109,
            }
        );
    }

    #[test]
    fn an14114_tx_continuous_88w9098_5_ghz_cw_stop() {
        // tx_continuous=0 1 0xAAA 0 3 0x2109 — stop CW mode
        let cmd = parse_procfs_command("tx_continuous=0 1 0xAAA 0 3 0x2109").unwrap();
        assert_eq!(
            cmd,
            MfgCmd::TxContinuous {
                enable: false,
                continuous_wave_mode: true,
                payload_pattern: 0xAAA,
                cs_mode: false,
                active_subchannel: ActiveSubchannel::Both,
                tx_rate: 0x2109,
            }
        );
    }

    // AN14114: "Set TX frame" example
    #[test]
    fn an14114_tx_frame_12_mbps() {
        // tx_frame=1 0x8 0xAAA 0x256 0 20 0 0 0 0 0 0 0 -1 -1 -1 -1 -1 -1 -1 -1
        // 12 Mbps, 0xAAA pattern, 0x256 (598) packet length
        let cmd = parse_procfs_command(
            "tx_frame=1 0x8 0xAAA 0x256 0 20 0 0 0 0 0 0 0 -1 -1 -1 -1 -1 -1 -1 -1",
        )
        .unwrap();
        assert_eq!(
            cmd,
            MfgCmd::TxFrame {
                enable: true,
                tx_rate: 0x8,
                payload_pattern: 0xAAA,
                payload_length: 0x256,
                adjust_burst_sifs_gap: false,
                adjust_sifs_us: 20,
                short_preamble: false,
                active_subchannel: ActiveSubchannel::Lower,
                short_gi: false,
                adv_coding: false,
                beamforming: false,
                greenfield_mode: false,
                stbc: false,
                signal_bw: SignalBandwidth::Default,
                num_pkt: -1,
                max_pkt_ext: -1,
                beam_change: -1,
                dcm: -1,
                doppler: -1,
                midamble_period: -1,
                q_num: -1,
                bssid: [0; MAC_ADDR_LENGTH],
            }
        );
    }

    // AN14114: "88W9098 5 GHz TX command sequence using tx_frame"
    #[test]
    fn an14114_tx_frame_88w9098_5_ghz_he_mcs11() {
        // tx_frame=1 0x210b 0xAAA 0x100 1 20 0 0 0 1 0 0 0 -1 -1 -1 -1 -1 -1 -1 -1
        // HE SS1 MCS11, adv_coding enabled, adjust_burst_sifs_gap enabled
        let cmd = parse_procfs_command(
            "tx_frame=1 0x210b 0xAAA 0x100 1 20 0 0 0 1 0 0 0 -1 -1 -1 -1 -1 -1 -1 -1",
        )
        .unwrap();
        assert_eq!(
            cmd,
            MfgCmd::TxFrame {
                enable: true,
                tx_rate: 0x210b,
                payload_pattern: 0xAAA,
                payload_length: 0x100,
                adjust_burst_sifs_gap: true,
                adjust_sifs_us: 20,
                short_preamble: false,
                active_subchannel: ActiveSubchannel::Lower,
                short_gi: false,
                adv_coding: true,
                beamforming: false,
                greenfield_mode: false,
                stbc: false,
                signal_bw: SignalBandwidth::Default,
                num_pkt: -1,
                max_pkt_ext: -1,
                beam_change: -1,
                dcm: -1,
                doppler: -1,
                midamble_period: -1,
                q_num: -1,
                bssid: [0; MAC_ADDR_LENGTH],
            }
        );
    }

    // AN14114: "5 GHz TX command sequence using tx_frame for HE-ER SU"
    #[test]
    fn an14114_tx_frame_88w9098_he_er_su_mcs1() {
        // tx_frame=1 0x2101 0xAAA 0x100 1 20 1 0 0 0 0 0 0 -1 -1 -1 -1 -1 -1 -1 -1
        // HE SS1 MCS1, short_preamble for HE-ER SU
        let cmd = parse_procfs_command(
            "tx_frame=1 0x2101 0xAAA 0x100 1 20 1 0 0 0 0 0 0 -1 -1 -1 -1 -1 -1 -1 -1",
        )
        .unwrap();
        assert_eq!(
            cmd,
            MfgCmd::TxFrame {
                enable: true,
                tx_rate: 0x2101,
                payload_pattern: 0xAAA,
                payload_length: 0x100,
                adjust_burst_sifs_gap: true,
                adjust_sifs_us: 20,
                short_preamble: true,
                active_subchannel: ActiveSubchannel::Lower,
                short_gi: false,
                adv_coding: false,
                beamforming: false,
                greenfield_mode: false,
                stbc: false,
                signal_bw: SignalBandwidth::Default,
                num_pkt: -1,
                max_pkt_ext: -1,
                beam_change: -1,
                dcm: -1,
                doppler: -1,
                midamble_period: -1,
                q_num: -1,
                bssid: [0; MAC_ADDR_LENGTH],
            }
        );
    }

    // AN14114: "88W9098 2.4 GHz TX command sequence using tx_frame"
    #[test]
    fn an14114_tx_frame_88w9098_2_4_ghz_he_mcs4() {
        // tx_frame=1 0x2104 0xAAA 0x100 1 20 0 0 0 0 0 0 0 -1 -1 -1 -1 -1 -1 -1 -1
        // HE SS1 MCS4
        let cmd = parse_procfs_command(
            "tx_frame=1 0x2104 0xAAA 0x100 1 20 0 0 0 0 0 0 0 -1 -1 -1 -1 -1 -1 -1 -1",
        )
        .unwrap();
        assert_eq!(
            cmd,
            MfgCmd::TxFrame {
                enable: true,
                tx_rate: 0x2104,
                payload_pattern: 0xAAA,
                payload_length: 0x100,
                adjust_burst_sifs_gap: true,
                adjust_sifs_us: 20,
                short_preamble: false,
                active_subchannel: ActiveSubchannel::Lower,
                short_gi: false,
                adv_coding: false,
                beamforming: false,
                greenfield_mode: false,
                stbc: false,
                signal_bw: SignalBandwidth::Default,
                num_pkt: -1,
                max_pkt_ext: -1,
                beam_change: -1,
                dcm: -1,
                doppler: -1,
                midamble_period: -1,
                q_num: -1,
                bssid: [0; MAC_ADDR_LENGTH],
            }
        );
    }

    // AN14114: "Configure the golden unit" — trigger_frame for UL-OFDMA
    #[test]
    fn an14114_trigger_frame_golden_unit() {
        // trigger_frame=1 1 1 2 5484 0 1000 0 0 0 1 0 0 0 1 0 1 0 65535 0 511 5 0 0 1 0x2102 0 0 90 0 0 0 0
        // standalone_hetb=1 (trigger-based), ldpc_ess=1, ul_coding_type=1, ul_mcs=0x2102
        let cmd = parse_procfs_command(
            "trigger_frame=1 1 1 2 5484 0 1000 0 0 0 1 0 0 0 1 0 1 0 65535 0 511 5 0 0 1 0x2102 0 0 90 0 0 0 0",
        )
        .unwrap();
        assert_eq!(
            cmd,
            MfgCmd::TriggerFrame {
                enable: true,
                standalone_hetb: StandaloneHeTbMode::TriggerBased,
                frame_ctrl_type: 1,
                frame_ctrl_subtype: 2,
                frame_duration: 5484,
                trigger_type: 0,
                ul_len: 1000,
                more_tf: false,
                cs_required: false,
                ul_bw: UlBandwidth::Bw20,
                ltf_type: 1,
                ltf_mode: false,
                ltf_symbol: LtfSymbol::OneXHELTF,
                ul_stbc: false,
                ldpc_ess: true,
                ap_tx_pwr: 0,
                pre_fec_pad_fct: 1,
                pe_disambig: false,
                spatial_reuse: 65535,
                doppler: false,
                he_sig2: 511,
                aid12: 5,
                ru_alloc_reg: false,
                ru_alloc: 0,
                ul_coding_type: true,
                ul_mcs: 0x2102,
                ul_dcm: false,
                ss_alloc: SpatialStreamAllocation::OneSS,
                ul_target_rssi: 90,
                mpdu_mu_sf: 0,
                tid_al: 0,
                ac_pl: false,
                pref_ac: 0,
            }
        );
    }

    // AN14114: "Testing standalone UL-OFDMA"
    #[test]
    fn an14114_trigger_frame_standalone() {
        // trigger_frame=1 2 1 2 5484 0 1000 0 0 2 1 0 0 0 1 0 1 0 65535 0 511 5 0 67 1 0 0 0 90 0 0 0 0
        // standalone_hetb=2 (standalone), ul_bw=2 (80 MHz), ru_alloc=67, ul_coding_type=1
        let cmd = parse_procfs_command(
            "trigger_frame=1 2 1 2 5484 0 1000 0 0 2 1 0 0 0 1 0 1 0 65535 0 511 5 0 67 1 0 0 0 90 0 0 0 0",
        )
        .unwrap();
        assert_eq!(
            cmd,
            MfgCmd::TriggerFrame {
                enable: true,
                standalone_hetb: StandaloneHeTbMode::Standalone,
                frame_ctrl_type: 1,
                frame_ctrl_subtype: 2,
                frame_duration: 5484,
                trigger_type: 0,
                ul_len: 1000,
                more_tf: false,
                cs_required: false,
                ul_bw: UlBandwidth::Bw80,
                ltf_type: 1,
                ltf_mode: false,
                ltf_symbol: LtfSymbol::OneXHELTF,
                ul_stbc: false,
                ldpc_ess: true,
                ap_tx_pwr: 0,
                pre_fec_pad_fct: 1,
                pe_disambig: false,
                spatial_reuse: 65535,
                doppler: false,
                he_sig2: 511,
                aid12: 5,
                ru_alloc_reg: false,
                ru_alloc: 67,
                ul_coding_type: true,
                ul_mcs: 0,
                ul_dcm: false,
                ss_alloc: SpatialStreamAllocation::OneSS,
                ul_target_rssi: 90,
                mpdu_mu_sf: 0,
                tid_al: 0,
                ac_pl: false,
                pref_ac: 0,
            }
        );
    }

    // AN14114: "Configure the golden unit" — tx_frame for UL-OFDMA golden unit
    #[test]
    fn an14114_tx_frame_golden_unit_ofdma() {
        // tx_frame=1 0x2102 0xabababab 0x256 0 20 0 0 0 0 0 0 0 -1 -1 -1 -1 -1 -1 -1 -1
        // HE SS1 MCS2 for trigger frame generation
        let cmd = parse_procfs_command(
            "tx_frame=1 0x2102 0xabababab 0x256 0 20 0 0 0 0 0 0 0 -1 -1 -1 -1 -1 -1 -1 -1",
        )
        .unwrap();
        assert_eq!(
            cmd,
            MfgCmd::TxFrame {
                enable: true,
                tx_rate: 0x2102,
                payload_pattern: 0xabababab,
                payload_length: 0x256,
                adjust_burst_sifs_gap: false,
                adjust_sifs_us: 20,
                short_preamble: false,
                active_subchannel: ActiveSubchannel::Lower,
                short_gi: false,
                adv_coding: false,
                beamforming: false,
                greenfield_mode: false,
                stbc: false,
                signal_bw: SignalBandwidth::Default,
                num_pkt: -1,
                max_pkt_ext: -1,
                beam_change: -1,
                dcm: -1,
                doppler: -1,
                midamble_period: -1,
                q_num: -1,
                bssid: [0; MAC_ADDR_LENGTH],
            }
        );
    }

    // AN14114: "Testing standalone UL-OFDMA" — tx_frame for standalone
    #[test]
    fn an14114_tx_frame_standalone_ofdma() {
        // tx_frame=1 0x2100 0xabababab 0x256 0 20 0 0 0 0 0 0 0 -1 -1 -1 -1 -1 -1 -1 -1
        // HE SS1 MCS0 for standalone UL-OFDMA
        let cmd = parse_procfs_command(
            "tx_frame=1 0x2100 0xabababab 0x256 0 20 0 0 0 0 0 0 0 -1 -1 -1 -1 -1 -1 -1 -1",
        )
        .unwrap();
        assert_eq!(
            cmd,
            MfgCmd::TxFrame {
                enable: true,
                tx_rate: 0x2100,
                payload_pattern: 0xabababab,
                payload_length: 0x256,
                adjust_burst_sifs_gap: false,
                adjust_sifs_us: 20,
                short_preamble: false,
                active_subchannel: ActiveSubchannel::Lower,
                short_gi: false,
                adv_coding: false,
                beamforming: false,
                greenfield_mode: false,
                stbc: false,
                signal_bw: SignalBandwidth::Default,
                num_pkt: -1,
                max_pkt_ext: -1,
                beam_change: -1,
                dcm: -1,
                doppler: -1,
                midamble_period: -1,
                q_num: -1,
                bssid: [0; MAC_ADDR_LENGTH],
            }
        );
    }

    // AN14114: "Configure the golden unit and DUT" — he_tb_tx with 9 dBm
    #[test]
    fn an14114_he_tb_tx_dut_9dbm() {
        // he_tb_tx=1 1 5 400 9 — HE TB-TX with TX power 9 dBm
        let cmd = parse_procfs_command("he_tb_tx=1 1 5 400 9").unwrap();
        assert_eq!(
            cmd,
            MfgCmd::HeTbTx {
                enable: true,
                qnum: 1,
                aid: 5,
                axq0_mu_timer: 400,
                tx_pwr: 9,
            }
        );
    }

    #[test]
    fn parse_unknown_command_fails() {
        let result = parse_procfs_command("unknown_cmd=1");
        assert!(result.is_err());
    }

    #[test]
    fn parse_missing_equals_fails() {
        let result = parse_procfs_command("tx_antenna 1");
        assert!(result.is_err());
    }

    #[test]
    fn parse_invalid_antenna_mode_fails() {
        assert!(parse_procfs_command("tx_antenna=5").is_err());
    }

    #[test]
    fn parse_radio_mode_missing_radio1_fails() {
        let result = parse_procfs_command("radio_mode=11");
        assert!(result.is_err());
    }

    #[test]
    fn parse_tx_power_missing_args_fails() {
        let result = parse_procfs_command("tx_power=15");
        assert!(result.is_err());
    }
}
