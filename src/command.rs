// SPDX-FileCopyrightText: 2026 The mwifiex_nl80211_test_mode authors
// SPDX-License-Identifier: GPL-2.0-only

use std::error::Error;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use zerocopy::FromBytes;

use crate::ffi::{
    CAL_DATA_LEN, GenericCfg, HeTbTx, HostCmd, IeeetypesBasicHeTrigUserInfo,
    IeeetypesCtlBasicTrigHdr, IeeetypesFrameCtrl, IeeetypesHeTrigComInfo, IeeetypesHeTrigUserInfo,
    IeeetypesHeTrigUserInfoBits, IeeetypesHeTrigUserInfoRssi, MAC_ADDR_LENGTH, MfgCmdHeader,
    MwifiexMfgCmd, OtpCalDataRdWr, OtpMacAddrRdWr, TxCont, TxFrame2,
};

/// MWIFIEX compatible WiFi Cards
///
/// Source: mlan_decl.h
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
pub enum CardType {
    /// 8887 card type
    #[value(name = "8887")]
    _8887,
    /// 8897 card type
    #[value(name = "8897")]
    _8897,
    /// 8977 card type
    #[value(name = "8977")]
    _8977,
    /// 8997 card type
    #[value(name = "8997")]
    _8997,
    /// 8987 card type
    #[value(name = "8987")]
    _8987,
    /// 9098 card type
    #[value(name = "9098")]
    _9098,
    /// 9097 card type
    #[value(name = "9097")]
    _9097,
    /// 8978 card type
    #[value(name = "8978")]
    _8978,
    /// 9177 card type
    #[value(name = "9177")]
    _9177,
    /// 8801 card type
    #[value(name = "8801")]
    _8801,
    /// OWL card type
    IW624,
    /// Black bird card type
    AW693,
    /// IW610 card type
    IW610,
}

/// Antenna path selection for TX/RX antenna configuration.
///
/// Source: AN14114 "Set TX/RX antenna configuration".
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AntennaMode {
    /// Path A. Use for 1x1 devices.
    A = 1,
    /// Path B
    B = 2,
    /// Paths A and B
    AB = 3,
}

impl TryFrom<u32> for AntennaMode {
    type Error = String;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::A),
            2 => Ok(Self::B),
            3 => Ok(Self::AB),
            _ => Err(format!("invalid antenna mode: {v}")),
        }
    }
}

/// TX signal path selection for power configuration.
///
/// Source: AN14114 "Set TX power".
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TxPathId {
    /// Path A. Use for 1x1 devices.
    A = 0,
    /// Path B
    B = 1,
    /// Paths A and B
    AB = 2,
}

impl TryFrom<u32> for TxPathId {
    type Error = String;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::A),
            1 => Ok(Self::B),
            2 => Ok(Self::AB),
            _ => Err(format!("invalid tx path id: {v}")),
        }
    }
}

/// RF band selection.
///
/// Source: AN14114 "Set the operating RF band".
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RfBand {
    /// 2.4 GHz band
    Ghz2_4 = 0,
    /// 5 GHz band
    Ghz5 = 1,
}

impl TryFrom<u32> for RfBand {
    type Error = String;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Ghz2_4),
            1 => Ok(Self::Ghz5),
            _ => Err(format!("invalid rf band: {v}")),
        }
    }
}

/// Channel bandwidth selection.
///
/// Source: AN14114 "Set the channel bandwidth".
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelBandwidth {
    /// 20 MHz
    Bw20 = 0,
    /// 40 MHz
    Bw40 = 1,
    /// 80 MHz
    Bw80 = 4,
}

impl TryFrom<u32> for ChannelBandwidth {
    type Error = String;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Bw20),
            1 => Ok(Self::Bw40),
            4 => Ok(Self::Bw80),
            _ => Err(format!("invalid channel bandwidth: {v}")),
        }
    }
}

/// Modulation selection for TX power configuration.
///
/// Source: AN14114 "Set TX power".
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Modulation {
    /// CCK modulation
    Cck = 0,
    /// OFDM modulation
    Ofdm = 1,
    /// MCS modulation
    Mcs = 2,
}

impl TryFrom<u32> for Modulation {
    type Error = String;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Cck),
            1 => Ok(Self::Ofdm),
            2 => Ok(Self::Mcs),
            _ => Err(format!("invalid modulation: {v}")),
        }
    }
}

/// Active subchannel selection for TX parameters.
///
/// Source: AN14114 "Set TX continuous mode" and "Set TX frame".
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActiveSubchannel {
    /// Lower subchannel
    #[default]
    Lower = 0,
    /// Upper subchannel
    Upper = 1,
    /// Both subchannels
    Both = 3,
}

impl TryFrom<u32> for ActiveSubchannel {
    type Error = String;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Lower),
            1 => Ok(Self::Upper),
            3 => Ok(Self::Both),
            _ => Err(format!("invalid active subchannel: {v}")),
        }
    }
}

/// TX frame signal bandwidth selection.
///
/// Source: AN14114 "Set TX frame".
#[repr(i32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalBandwidth {
    /// 20 MHz.
    Bw20 = 0,
    /// 40 MHz.
    Bw40 = 1,
    /// 80 MHz.
    Bw80 = 4,
    /// Firmware default.
    #[default]
    Default = -1,
}

impl TryFrom<u32> for SignalBandwidth {
    type Error = String;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v as i32 {
            0 => Ok(Self::Bw20),
            1 => Ok(Self::Bw40),
            4 => Ok(Self::Bw80),
            -1 => Ok(Self::Default),
            other => Err(format!("invalid signal bandwidth: {other}")),
        }
    }
}

/// Standalone HE TB mode selection.
///
/// Source: AN14114 "Configure the golden unit and DUT for UL-OFDMA transmission".
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StandaloneHeTbMode {
    #[default]
    Disabled = 0,
    TriggerBased = 1,
    Standalone = 2,
    SuOfdma = 3,
}

impl TryFrom<u32> for StandaloneHeTbMode {
    type Error = String;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Disabled),
            1 => Ok(Self::TriggerBased),
            2 => Ok(Self::Standalone),
            3 => Ok(Self::SuOfdma),
            _ => Err(format!("invalid standalone he tb mode: {v}")),
        }
    }
}

/// UL trigger frame bandwidth selection.
///
/// Source: AN14114 "Configure the golden unit and DUT for UL-OFDMA transmission".
#[repr(u64)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UlBandwidth {
    #[default]
    Bw20 = 0,
    Bw40 = 1,
    Bw80 = 2,
}

impl TryFrom<u64> for UlBandwidth {
    type Error = String;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Bw20),
            1 => Ok(Self::Bw40),
            2 => Ok(Self::Bw80),
            _ => Err(format!("invalid ul bandwidth: {v}")),
        }
    }
}

/// LTF symbol selection.
///
/// Source: AN14114 "Configure the golden unit and DUT for UL-OFDMA transmission".
#[repr(u64)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LtfSymbol {
    #[default]
    #[serde(rename = "1xheltf")]
    OneXHELTF = 0,
    #[serde(rename = "2xheltf")]
    TwoXHELTF = 1,
}

impl TryFrom<u64> for LtfSymbol {
    type Error = String;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::OneXHELTF),
            1 => Ok(Self::TwoXHELTF),
            _ => Err(format!("invalid ltf symbol: {v}")),
        }
    }
}

/// Spatial stream allocation selection.
///
/// Source: AN14114 "Configure the golden unit and DUT for UL-OFDMA transmission".
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpatialStreamAllocation {
    #[default]
    #[serde(rename = "1ss")]
    OneSS = 0,
    #[serde(rename = "2ss")]
    TwoSS = 1,
}

impl TryFrom<u32> for SpatialStreamAllocation {
    type Error = String;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::OneSS),
            1 => Ok(Self::TwoSS),
            _ => Err(format!("invalid spatial stream allocation: {v}")),
        }
    }
}

/// Wrapper enum for all testmode command structs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MfgCmd {
    /// Enable or disable Wi-Fi RF test mode (rf_test_mode).
    ///
    /// Source: AN14114 "Wi-Fi RF test mode".
    RfTestMode {
        /// Enable (true) or disable (false) RF test mode.
        enable: bool,
    },
    /// Set the TX antenna path selection (tx_antenna).
    ///
    /// Source: AN14114 "Set TX/RX antenna configuration".
    TxAntenna {
        /// TX antenna mode.
        mode: AntennaMode,
    },
    /// Set the RX antenna path selection (rx_antenna).
    ///
    /// Source: AN14114 "Set TX/RX antenna configuration".
    RxAntenna {
        /// RX antenna mode.
        mode: AntennaMode,
    },
    /// Select the operating radio mode for radio0 and radio1 (radio_mode).
    ///
    /// Source: AN14114 "Set the radio mode".
    RadioMode {
        /// Radio mode index for radio 0 (device-specific indices per AN14114).
        radio0: u32,
        /// Radio mode index for radio 1 (device-specific indices per AN14114).
        radio1: u32,
    },
    /// Select the RF band (band).
    ///
    /// Source: AN14114 "Set the operating RF band".
    RfBand {
        /// RF band selection.
        band: RfBand,
    },
    /// Select the channel bandwidth (bw).
    ///
    /// Source: AN14114 "Set the channel bandwidth".
    ChannelBandwidth {
        /// Channel bandwidth selection.
        bandwidth: ChannelBandwidth,
    },
    /// Set the RF channel (channel).
    ///
    /// Source: AN14114 "Set the RF channel".
    RfChannel {
        /// RF channel number (see supported channels list in AN14114).
        channel: u16,
    },
    /// Get and reset packet error rate counters (get_and_reset_per).
    ///
    /// Source: AN14114 "Get and reset the packet error rate".
    GetAndResetPer {
        /// Total RX unicast/multicast/broadcast packet count
        #[serde(skip)]
        rx_total_packet_count: u32,
        /// RX multicast/broadcast packet count
        #[serde(skip)]
        rx_multi_broadcast_packet_count: u32,
        /// RX FCS error count
        #[serde(skip)]
        rx_frame_check_sequence_errors: u32,
    },
    /// Configure transmit power (tx_power).
    ///
    /// Source: AN14114 "Set TX power".
    TxPower {
        /// TX power in dBm, range -1 to 24 and 64 on newer cards; -1 lets firmware decide.
        power_dbm: i32,
        /// Signal modulation type.
        modulation: Modulation,
        /// TX path selection.
        path: TxPathId,
    },
    /// Configure continuous TX parameters (tx_continuous).
    ///
    /// Source: AN14114 "Set TX continuous mode".
    TxContinuous {
        /// Start/stop transmit.
        enable: bool,
        /// Continuous wave mode enable. 0 = packet, 1 = CW.
        #[serde(default)]
        continuous_wave_mode: bool,
        /// Payload pattern value, range 0 to 0xFFFF_FFFF.
        #[serde(default)]
        payload_pattern: u32,
        /// Carrier suppression enable (packet mode only).
        #[serde(default)]
        cs_mode: bool,
        /// Active subchannel selection: 0 = lower, 1 = upper, 3 = both.
        #[serde(default)]
        active_subchannel: ActiveSubchannel,
        /// TX data rate index (legacy/HT/VHT rate table in AN14114).
        #[serde(default)]
        tx_rate: u32,
    },
    /// Configure frame-based TX parameters (tx_frame).
    ///
    /// Source: AN14114 "Set TX frame".
    TxFrame {
        /// Start/stop transmit.
        enable: bool,
        /// TX data rate index (legacy/HT/VHT rate table in AN14114).
        #[serde(default)]
        tx_rate: u32,
        /// Payload pattern value, range 0 to 0xFFFF_FFFF.
        #[serde(default)]
        payload_pattern: u32,
        /// Payload length in bytes, range 1 to 0x400.
        #[serde(default)]
        payload_length: u32,
        /// Adjust burst SIFS gap.
        #[serde(default)]
        adjust_burst_sifs_gap: bool,
        /// Burst SIFS duration in microseconds, range 10 to 255.
        #[serde(default)]
        adjust_sifs_us: u32,
        /// Short preamble enable.
        #[serde(default)]
        short_preamble: bool,
        /// Active subchannel selection: 0 = lower, 1 = upper, 3 = both.
        #[serde(default)]
        active_subchannel: ActiveSubchannel,
        /// Short guard interval selection (see AN14114 for per-phy meaning).
        #[serde(default)]
        short_gi: bool,
        /// Advanced coding enable.
        #[serde(default)]
        adv_coding: bool,
        /// Beamforming enable.
        #[serde(default)]
        beamforming: bool,
        /// Greenfield mode enable.
        #[serde(default)]
        greenfield_mode: bool,
        /// STBC enable.
        #[serde(default)]
        stbc: bool,
        /// Signal bandwidth: 0 = 20 MHz, 1 = 40 MHz, 4 = 80 MHz, -1 = default.
        #[serde(default)]
        signal_bw: SignalBandwidth,
        /// Number of packets.
        #[serde(default = "default_neg_one")]
        num_pkt: i32,
        /// Max packet extension.
        #[serde(default = "default_neg_one")]
        max_pkt_ext: i32,
        /// Beam change.
        #[serde(default = "default_neg_one")]
        beam_change: i32,
        /// DCM enable.
        #[serde(default = "default_neg_one")]
        dcm: i32,
        /// Doppler enable.
        #[serde(default = "default_neg_one")]
        doppler: i32,
        /// Midamble periodicity.
        #[serde(default = "default_neg_one")]
        midamble_period: i32,
        /// Trigger response queue number.
        #[serde(default = "default_neg_one")]
        q_num: i32,
        /// BSSID address in format xx:xx:xx:xx:xx:xx.
        #[serde(default)]
        bssid: [u8; MAC_ADDR_LENGTH],
    },
    /// Configure HE trigger frame parameters (trigger_frame).
    ///
    /// Source: AN14114 "Configure the golden unit and DUT for UL-OFDMA transmission".
    TriggerFrame {
        /// Enable transmit.
        enable: bool,
        /// Standalone HE TB mode: 0 = disable, 1 = trigger-based, 2 = standalone, 3 = SU-OFDMA.
        #[serde(default)]
        standalone_hetb: StandaloneHeTbMode,
        /// Frame control type (set to 1 in examples).
        #[serde(default)]
        frame_ctrl_type: u16,
        /// Frame control subtype (set to 2 in examples).
        #[serde(default)]
        frame_ctrl_subtype: u16,
        /// Frame duration (set to 5484 in examples).
        #[serde(default)]
        frame_duration: u16,
        /// Trigger type (set to 0 in examples).
        #[serde(default)]
        trigger_type: u64,
        /// UL length (set to 1000 in examples).
        #[serde(default)]
        ul_len: u64,
        /// More TF flag.
        #[serde(default)]
        more_tf: bool,
        /// CS required flag.
        #[serde(default)]
        cs_required: bool,
        /// UL bandwidth: 0 = 20 MHz, 1 = 40 MHz, 2 = 80 MHz.
        #[serde(default)]
        ul_bw: UlBandwidth,
        /// LTF type (set to 1 in examples).
        #[serde(default)]
        ltf_type: u64,
        /// LTF mode.
        #[serde(default)]
        ltf_mode: bool,
        /// LTF symbol: 0 = 1xHELTF (1SS), 1 = 2xHELTF (2SS).
        #[serde(default)]
        ltf_symbol: LtfSymbol,
        /// UL STBC flag.
        #[serde(default)]
        ul_stbc: bool,
        /// LDPC ESS flag.
        #[serde(default)]
        ldpc_ess: bool,
        /// AP TX power (set to 0 in examples).
        #[serde(default)]
        ap_tx_pwr: u64,
        /// Pre-FEC padding factor (set to 1 in examples).
        #[serde(default)]
        pre_fec_pad_fct: u64,
        /// PE disambiguity flag.
        #[serde(default)]
        pe_disambig: bool,
        /// Spatial reuse value (set to 65535 in examples).
        #[serde(default)]
        spatial_reuse: u64,
        /// Doppler flag.
        #[serde(default)]
        doppler: bool,
        /// HE-SIG2 value (set to 511 in examples).
        #[serde(default)]
        he_sig2: u64,
        /// AID12 value (set to 5 in examples).
        #[serde(default)]
        aid12: u32,
        /// RU allocation register flag.
        #[serde(default)]
        ru_alloc_reg: bool,
        /// RU allocation index (see RU index tables in AN14114).
        #[serde(default)]
        ru_alloc: u32,
        /// UL coding type flag.
        #[serde(default)]
        ul_coding_type: bool,
        /// UL MCS index (see data rates table in AN14114).
        #[serde(default)]
        ul_mcs: u32,
        /// UL DCM flag.
        #[serde(default)]
        ul_dcm: bool,
        /// Spatial stream allocation: 0 = 1SS, 1 = 2SS.
        #[serde(default)]
        ss_alloc: SpatialStreamAllocation,
        /// UL target RSSI (set to 90 in examples).
        #[serde(default)]
        ul_target_rssi: u8,
        /// MPDU MU SF (set to 0 in examples).
        #[serde(default)]
        mpdu_mu_sf: u8,
        /// TID allocation (set to 0 in examples).
        #[serde(default)]
        tid_al: u8,
        /// AC policy flag.
        #[serde(default)]
        ac_pl: bool,
        /// Preferred AC (set to 0 in examples).
        #[serde(default)]
        pref_ac: u8,
    },
    /// Configure HE TB TX trigger-response parameters (he_tb_tx).
    ///
    /// Source: AN14114 "Configure the golden unit and DUT for UL-OFDMA transmission".
    HeTbTx {
        /// Enter/exit trigger response mode.
        enable: bool,
        /// Trigger response queue number (1 = trigger-based test by default).
        #[serde(default)]
        qnum: u16,
        /// Station AID (set to 5 in examples).
        #[serde(default)]
        aid: u16,
        /// AXQ0 MU timer in 8 ms units (suggested 400 in examples).
        #[serde(default)]
        axq0_mu_timer: u16,
        /// TX power in dBm.
        #[serde(default)]
        tx_pwr: i16,
    },
    /// Read/write OTP calibration data (otp_cal_data_rd_wr).
    ///
    /// Source: Mwifiex driver
    OtpCalData {
        /// Read/write flag: false = read, true = write.
        write: bool,
        /// OTP calibration data buffer (hex string of length 2800).
        cal_data: Vec<u8>, // Using Vec<u8> to allow variable length up to CAL_DATA_LEN
    },
    /// Read/write OTP MAC address (otp_mac_addr_rd_wr).
    ///
    /// Source: Mwifiex driver
    OtpMacAddr {
        /// Read/write flag: false = read, true = write.
        write: bool,
        /// MAC address buffer (hex string of length 12).
        mac_addr: [u8; MAC_ADDR_LENGTH],
    },
}

fn default_neg_one() -> i32 {
    -1
}

impl MfgCmd {
    /// Convert the command struct into a byte buffer for sending to the firmware.
    pub fn into_host_command_buffer(&self, card_type: CardType) -> Result<Vec<u8>, Box<dyn Error>> {
        // See woal_config_write in the mwifiex driver for how these commands are constructed.
        let command_number = self.command_number();

        let buffer = match self {
            MfgCmd::RfTestMode { .. } => GenericCfg::default()
                .into_host_cmd(command_number, true)
                .to_vec(),

            MfgCmd::TxAntenna { mode } => GenericCfg {
                data1: *mode as u32,
                data2: Default::default(),
                data3: Default::default(),
            }
            .into_host_cmd(command_number, true)
            .to_vec(),

            MfgCmd::RxAntenna { mode } => GenericCfg {
                data1: *mode as u32,
                data2: Default::default(),
                data3: Default::default(),
            }
            .into_host_cmd(command_number, true)
            .to_vec(),

            MfgCmd::RadioMode { radio0, radio1 } => GenericCfg {
                data1: *radio0,
                data2: *radio1,
                data3: Default::default(),
            }
            .into_host_cmd(command_number, true)
            .to_vec(),

            MfgCmd::RfBand { band } => GenericCfg {
                data1: *band as u32,
                data2: Default::default(),
                data3: Default::default(),
            }
            .into_host_cmd(command_number, true)
            .to_vec(),

            MfgCmd::ChannelBandwidth { bandwidth } => GenericCfg {
                data1: *bandwidth as u32,
                data2: Default::default(),
                data3: Default::default(),
            }
            .into_host_cmd(command_number, true)
            .to_vec(),

            MfgCmd::RfChannel { channel } => GenericCfg {
                data1: *channel as u32,
                data2: Default::default(),
                data3: Default::default(),
            }
            .into_host_cmd(command_number, true)
            .to_vec(),

            MfgCmd::GetAndResetPer { .. } => GenericCfg::default()
                .into_host_cmd(command_number, false)
                .to_vec(),

            MfgCmd::TxPower {
                power_dbm,
                modulation,
                path,
            } => {
                let power_dbm: i32 = match card_type {
                    CardType::_9098
                    | CardType::_9097
                    | CardType::_9177
                    | CardType::IW624
                    | CardType::AW693
                    | CardType::IW610 => {
                        // Port of PowerLevelToDUT11Bits logic
                        let power_dbm = if *power_dbm == -1 // 0xFFFF_FFFFu32 == -1_i32
                            || *power_dbm > 64
                            || *power_dbm < -64
                        {
                            -1
                        } else {
                            let mut z = power_dbm * 16;
                            if z < 0 {
                                z += 2048;
                            }
                            z
                        };

                        if power_dbm > 384 {
                            return Err("power_dbm must be <= 384".into());
                        }

                        power_dbm
                    }
                    _ => {
                        if *power_dbm > 24 {
                            return Err("power_dbm must be <= 24".into());
                        }
                        *power_dbm
                    }
                };

                GenericCfg {
                    data1: power_dbm as u32,
                    data2: *modulation as u32,
                    data3: *path as u32,
                }
            }
            .into_host_cmd(command_number, true)
            .to_vec(),

            MfgCmd::TxContinuous {
                enable,
                continuous_wave_mode,
                payload_pattern,
                cs_mode,
                active_subchannel,
                tx_rate,
            } => TxCont {
                enable_tx: u32::from(*enable),
                cw_mode: u32::from(*continuous_wave_mode),
                payload_pattern: *payload_pattern,
                cs_mode: u32::from(*cs_mode),
                act_sub_ch: *active_subchannel as u32,
                tx_rate: *tx_rate,
                rsvd: Default::default(),
            }
            .into_host_cmd(command_number, true)
            .to_vec(),

            MfgCmd::TxFrame {
                enable,
                tx_rate,
                payload_pattern,
                payload_length,
                adjust_burst_sifs_gap,
                adjust_sifs_us,
                short_preamble,
                active_subchannel,
                short_gi,
                adv_coding,
                beamforming,
                greenfield_mode,
                stbc,
                signal_bw,
                num_pkt,
                max_pkt_ext,
                beam_change,
                dcm,
                doppler,
                midamble_period,
                q_num,
                bssid,
            } => TxFrame2 {
                enable: u32::from(*enable),
                data_rate: *tx_rate,
                frame_pattern: *payload_pattern,
                frame_length: *payload_length,
                bssid: *bssid,
                adjust_burst_sifs: u16::from(*adjust_burst_sifs_gap),
                burst_sifs_in_us: *adjust_sifs_us,
                short_preamble: u32::from(*short_preamble),
                act_sub_ch: *active_subchannel as u32,
                short_gi: u32::from(*short_gi),
                adv_coding: u32::from(*adv_coding),
                tx_bf: u32::from(*beamforming),
                gf_mode: u32::from(*greenfield_mode),
                stbc: u32::from(*stbc),
                signal_bw: *signal_bw as i32 as u32,
                num_pkt: *num_pkt as u32,
                max_pe: *max_pkt_ext as u32,
                beam_change: *beam_change as u32,
                dcm: *dcm as u32,
                doppler: *doppler as u32,
                mid_p: *midamble_period as u32,
                q_num: *q_num as u32,
                rsvd: Default::default(),
            }
            .into_host_cmd(command_number, true)
            .to_vec(),

            MfgCmd::TriggerFrame {
                enable,
                standalone_hetb,
                frame_ctrl_type,
                frame_ctrl_subtype,
                frame_duration,
                trigger_type,
                ul_len,
                more_tf,
                cs_required,
                ul_bw,
                ltf_type,
                ltf_mode,
                ltf_symbol,
                ul_stbc,
                ldpc_ess,
                ap_tx_pwr,
                pre_fec_pad_fct,
                pe_disambig,
                spatial_reuse,
                doppler,
                he_sig2,
                aid12,
                ru_alloc_reg,
                ru_alloc,
                ul_coding_type,
                ul_mcs,
                ul_dcm,
                ss_alloc,
                ul_target_rssi,
                mpdu_mu_sf,
                tid_al,
                ac_pl,
                pref_ac,
            } => IeeetypesCtlBasicTrigHdr {
                enable_tx: u32::from(*enable),
                standalone_hetb: *standalone_hetb as u32,
                frm_ctl: IeeetypesFrameCtrl::new()
                    .with_frame_type(*frame_ctrl_type)
                    .with_sub_type(*frame_ctrl_subtype),
                duration: *frame_duration,
                dest_addr: Default::default(), // Not filled by wlan_cmd_mfg_config_trigger_frame
                src_addr: Default::default(),  // Not filled by wlan_cmd_mfg_config_trigger_frame
                trig_common_field: IeeetypesHeTrigComInfo::new()
                    .with_trigger_type(*trigger_type)
                    .with_ul_len(*ul_len)
                    .with_more_tf(u64::from(*more_tf))
                    .with_cs_required(u64::from(*cs_required))
                    .with_ul_bw(*ul_bw as u64)
                    .with_ltf_type(*ltf_type)
                    .with_ltf_mode(u64::from(*ltf_mode))
                    .with_ltf_symbol(*ltf_symbol as u64)
                    .with_ul_stbc(u64::from(*ul_stbc))
                    .with_ldpc_ess(u64::from(*ldpc_ess))
                    .with_ap_tx_pwr(*ap_tx_pwr)
                    .with_pre_fec_pad_fct(*pre_fec_pad_fct)
                    .with_pe_disambig(u64::from(*pe_disambig))
                    .with_spatial_reuse(*spatial_reuse)
                    .with_doppler(u64::from(*doppler))
                    .with_he_sig2(*he_sig2),
                trig_user_info_field: IeeetypesHeTrigUserInfo {
                    bits: IeeetypesHeTrigUserInfoBits::new()
                        .with_aid12(*aid12)
                        .with_ru_alloc_reg(u32::from(*ru_alloc_reg))
                        .with_ru_alloc(*ru_alloc)
                        .with_ul_coding_type(u32::from(*ul_coding_type))
                        .with_ul_mcs(*ul_mcs)
                        .with_ul_dcm(u32::from(*ul_dcm))
                        .with_ss_alloc(*ss_alloc as u32),
                    rssi: IeeetypesHeTrigUserInfoRssi::new().with_ul_target_rssi(*ul_target_rssi),
                },
                basic_trig_user_info: IeeetypesBasicHeTrigUserInfo::new()
                    .with_mpdu_mu_sf(*mpdu_mu_sf)
                    .with_tid_al(*tid_al)
                    .with_ac_pl(u8::from(*ac_pl))
                    .with_pref_ac(*pref_ac),
            }
            .into_host_cmd(command_number, true)
            .to_vec(),

            MfgCmd::HeTbTx {
                enable,
                qnum,
                aid,
                axq0_mu_timer,
                tx_pwr,
            } => HeTbTx {
                enable: u16::from(*enable),
                qnum: *qnum,
                aid: *aid,
                axq_mu_timer: *axq0_mu_timer,
                tx_power: *tx_pwr,
            }
            .into_host_cmd(command_number, true)
            .to_vec(),

            MfgCmd::OtpCalData { write, cal_data } => {
                if cal_data.len() > CAL_DATA_LEN {
                    return Err(format!("cal_data length exceeds maximum of {CAL_DATA_LEN}").into());
                }

                let mut cal_data_buffer = [0u8; CAL_DATA_LEN];
                cal_data_buffer[..cal_data.len()].copy_from_slice(cal_data);

                OtpCalDataRdWr {
                    cal_data_status: 0, // Probably filled by firmware response
                    cal_data_len: cal_data.len() as u32,
                    cal_data: cal_data_buffer,
                }
                .into_host_cmd(command_number, *write)
                .to_vec()
            }

            MfgCmd::OtpMacAddr { write, mac_addr } => OtpMacAddrRdWr {
                mac_addr: *mac_addr,
            }
            .into_host_cmd(command_number, *write)
            .to_vec(),
        };

        Ok(buffer)
    }

    pub fn from_host_command_response(&self, buffer: &[u8]) -> Result<Self, Box<dyn Error>> {
        let command_number = self.command_number();

        let header = parse_header::<()>(buffer)?;

        if header.result != 0 {
            return Err(format!("{:#?}", header).into());
        }

        if header.body.mfg_cmd != command_number {
            let resp_cmd = header.body.mfg_cmd;
            return Err(format!(
                "Received response with wrong command code. Expected {command_number} but got {resp_cmd}.",
            )
            .into());
        }

        Ok(match self {
            MfgCmd::RfTestMode { .. } => MfgCmd::RfTestMode {
                enable: header.body.mfg_cmd != 0,
            },

            MfgCmd::TxAntenna { .. } => {
                let body: GenericCfg = parse_body(buffer)?;
                MfgCmd::TxAntenna {
                    mode: AntennaMode::try_from(body.data1)?,
                }
            }

            MfgCmd::RxAntenna { .. } => {
                let body: GenericCfg = parse_body(buffer)?;
                MfgCmd::RxAntenna {
                    mode: AntennaMode::try_from(body.data1)?,
                }
            }

            MfgCmd::RadioMode { .. } => {
                let body: GenericCfg = parse_body(buffer)?;
                MfgCmd::RadioMode {
                    radio0: body.data1,
                    radio1: body.data2,
                }
            }

            MfgCmd::RfBand { .. } => {
                let body: GenericCfg = parse_body(buffer)?;
                MfgCmd::RfBand {
                    band: RfBand::try_from(body.data1)?,
                }
            }

            MfgCmd::ChannelBandwidth { .. } => {
                let body: GenericCfg = parse_body(buffer)?;
                MfgCmd::ChannelBandwidth {
                    bandwidth: ChannelBandwidth::try_from(body.data1)?,
                }
            }

            MfgCmd::RfChannel { .. } => {
                let body: GenericCfg = parse_body(buffer)?;
                MfgCmd::RfChannel {
                    channel: body.data1 as u16,
                }
            }

            MfgCmd::GetAndResetPer { .. } => {
                let body: GenericCfg = parse_body(buffer)?;
                MfgCmd::GetAndResetPer {
                    rx_total_packet_count: body.data1,
                    rx_multi_broadcast_packet_count: body.data2,
                    rx_frame_check_sequence_errors: body.data3,
                }
            }

            MfgCmd::TxPower { .. } => {
                let body: GenericCfg = parse_body(buffer)?;
                MfgCmd::TxPower {
                    power_dbm: body.data1 as i32,
                    modulation: Modulation::try_from(body.data2)?,
                    path: TxPathId::try_from(body.data3)?,
                }
            }

            MfgCmd::TxContinuous { .. } => {
                let body: TxCont = parse_body(buffer)?;
                MfgCmd::TxContinuous {
                    enable: body.enable_tx != 0,
                    continuous_wave_mode: body.cw_mode != 0,
                    payload_pattern: body.payload_pattern,
                    cs_mode: body.cs_mode != 0,
                    active_subchannel: ActiveSubchannel::try_from(body.act_sub_ch)?,
                    tx_rate: body.tx_rate,
                }
            }

            MfgCmd::TxFrame { .. } => {
                let body: TxFrame2 = parse_body(buffer)?;
                MfgCmd::TxFrame {
                    enable: body.enable != 0,
                    tx_rate: body.data_rate,
                    payload_pattern: body.frame_pattern,
                    payload_length: body.frame_length,
                    adjust_burst_sifs_gap: body.adjust_burst_sifs != 0,
                    adjust_sifs_us: body.burst_sifs_in_us,
                    short_preamble: body.short_preamble != 0,
                    active_subchannel: ActiveSubchannel::try_from(body.act_sub_ch)?,
                    short_gi: body.short_gi != 0,
                    adv_coding: body.adv_coding != 0,
                    beamforming: body.tx_bf != 0,
                    greenfield_mode: body.gf_mode != 0,
                    stbc: body.stbc != 0,
                    signal_bw: SignalBandwidth::try_from(body.signal_bw)?,
                    num_pkt: body.num_pkt as i32,
                    max_pkt_ext: body.max_pe as i32,
                    beam_change: body.beam_change as i32,
                    dcm: body.dcm as i32,
                    doppler: body.doppler as i32,
                    midamble_period: body.mid_p as i32,
                    q_num: body.q_num as i32,
                    bssid: body.bssid,
                }
            }

            MfgCmd::TriggerFrame { .. } => {
                let body: IeeetypesCtlBasicTrigHdr = parse_body(buffer)?;
                let com = body.trig_common_field;
                let bits = body.trig_user_info_field.bits;
                let rssi = body.trig_user_info_field.rssi;
                let basic = body.basic_trig_user_info;
                let frm_ctl = body.frm_ctl;
                MfgCmd::TriggerFrame {
                    enable: body.enable_tx != 0,
                    standalone_hetb: StandaloneHeTbMode::try_from(body.standalone_hetb)?,
                    frame_ctrl_type: frm_ctl.frame_type(),
                    frame_ctrl_subtype: frm_ctl.sub_type(),
                    frame_duration: body.duration,
                    trigger_type: com.trigger_type(),
                    ul_len: com.ul_len(),
                    more_tf: com.more_tf() != 0,
                    cs_required: com.cs_required() != 0,
                    ul_bw: UlBandwidth::try_from(com.ul_bw())?,
                    ltf_type: com.ltf_type(),
                    ltf_mode: com.ltf_mode() != 0,
                    ltf_symbol: LtfSymbol::try_from(com.ltf_symbol())?,
                    ul_stbc: com.ul_stbc() != 0,
                    ldpc_ess: com.ldpc_ess() != 0,
                    ap_tx_pwr: com.ap_tx_pwr(),
                    pre_fec_pad_fct: com.pre_fec_pad_fct(),
                    pe_disambig: com.pe_disambig() != 0,
                    spatial_reuse: com.spatial_reuse(),
                    doppler: com.doppler() != 0,
                    he_sig2: com.he_sig2(),
                    aid12: bits.aid12(),
                    ru_alloc_reg: bits.ru_alloc_reg() != 0,
                    ru_alloc: bits.ru_alloc(),
                    ul_coding_type: bits.ul_coding_type() != 0,
                    ul_mcs: bits.ul_mcs(),
                    ul_dcm: bits.ul_dcm() != 0,
                    ss_alloc: SpatialStreamAllocation::try_from(bits.ss_alloc())?,
                    ul_target_rssi: rssi.ul_target_rssi(),
                    mpdu_mu_sf: basic.mpdu_mu_sf(),
                    tid_al: basic.tid_al(),
                    ac_pl: basic.ac_pl() != 0,
                    pref_ac: basic.pref_ac(),
                }
            }

            MfgCmd::HeTbTx { .. } => {
                let body: HeTbTx = parse_body(buffer)?;
                MfgCmd::HeTbTx {
                    enable: body.enable != 0,
                    qnum: body.qnum,
                    aid: body.aid,
                    axq0_mu_timer: body.axq_mu_timer,
                    tx_pwr: body.tx_power,
                }
            }

            MfgCmd::OtpCalData { write: rd_wr, .. } => {
                let body: OtpCalDataRdWr = parse_body(buffer)?;
                let len = (body.cal_data_len as usize).min(CAL_DATA_LEN);
                MfgCmd::OtpCalData {
                    write: *rd_wr,
                    cal_data: body.cal_data[..len].to_vec(),
                }
            }

            MfgCmd::OtpMacAddr { write: rd_wr, .. } => {
                let body: OtpMacAddrRdWr = parse_body(buffer)?;
                MfgCmd::OtpMacAddr {
                    write: *rd_wr,
                    mac_addr: body.mac_addr,
                }
            }
        })
    }

    fn command_number(&self) -> u32 {
        match self {
            MfgCmd::RfTestMode { enable: enabled } => {
                if *enabled {
                    1
                } else {
                    0
                }
            }
            MfgCmd::TxAntenna { .. } => 0x1004,
            MfgCmd::RxAntenna { .. } => 0x1005,
            MfgCmd::RadioMode { .. } => 0x1211,
            MfgCmd::RfBand { .. } => 0x1034,
            MfgCmd::ChannelBandwidth { .. } => 0x1044,
            MfgCmd::RfChannel { .. } => 0x100A,
            MfgCmd::GetAndResetPer { .. } => 0x1010,
            MfgCmd::TxPower { .. } => 0x1033,
            MfgCmd::TxContinuous { .. } => 0x1009,
            MfgCmd::TxFrame { .. } => 0x1021,
            MfgCmd::TriggerFrame { .. } => 0x110C,
            MfgCmd::HeTbTx { .. } => 0x110A,
            MfgCmd::OtpMacAddr { .. } => 0x108C,
            MfgCmd::OtpCalData { .. } => 0x121A,
        }
    }
}

fn parse_header<T: FromBytes>(buffer: &[u8]) -> Result<HostCmd<MfgCmdHeader<T>>, Box<dyn Error>> {
    let size = std::mem::size_of::<HostCmd<MfgCmdHeader<T>>>();
    HostCmd::<MfgCmdHeader<T>>::read_from_bytes(buffer.get(..size).ok_or_else(|| {
        format!("response buffer is smaller than {size}, can not parse response.")
    })?)
    .map_err(|e| format!("failed to parse response body {e:?}").into())
}

fn parse_body<T: FromBytes>(buffer: &[u8]) -> Result<T, Box<dyn Error>> {
    parse_header(buffer).map(|header| header.body.body)
}

#[cfg(test)]
mod tests {
    use zerocopy::IntoBytes;

    use crate::ffi::{HOST_CMD_CMD_MFG_COMMAND, HostCmd, MfgCmdHeader};

    use super::*;

    #[test]
    fn deserialize_rf_test_mode_cmd() {
        let input = r#"
        rf_test_mode:
            enable: true
        "#;

        let rf_test: MfgCmd = serde_saphyr::from_str(input).expect("failed to deserialize yaml");

        assert_eq!(rf_test, MfgCmd::RfTestMode { enable: true });
    }

    #[test]
    fn deserialize_tx_antenna_cmd() {
        let input = r#"
        tx_antenna:
            mode: a
        "#;

        let tx_ant: MfgCmd = serde_saphyr::from_str(input).expect("failed to deserialize yaml");

        assert_eq!(
            tx_ant,
            MfgCmd::TxAntenna {
                mode: AntennaMode::A
            }
        );
    }

    #[test]
    fn deserialize_rx_antenna_cmd() {
        let input = r#"
        rx_antenna:
            mode: b
        "#;

        let rx_ant: MfgCmd = serde_saphyr::from_str(input).expect("failed to deserialize yaml");

        assert_eq!(
            rx_ant,
            MfgCmd::RxAntenna {
                mode: AntennaMode::B
            }
        );
    }

    #[test]
    fn deserialize_radio_mode_cmd() {
        let input = r#"
        radio_mode:
            radio0: 1
            radio1: 0
        "#;

        let radio_mode: MfgCmd = serde_saphyr::from_str(input).expect("failed to deserialize yaml");

        assert_eq!(
            radio_mode,
            MfgCmd::RadioMode {
                radio0: 1,
                radio1: 0
            }
        );
    }

    #[test]
    fn deserialize_rf_band_cmd() {
        let input = r#"
        rf_band:
            band: ghz5
        "#;

        let band: MfgCmd = serde_saphyr::from_str(input).expect("failed to deserialize yaml");

        assert_eq!(band, MfgCmd::RfBand { band: RfBand::Ghz5 });
    }

    #[test]
    fn deserialize_channel_bandwidth_cmd() {
        let input = r#"
        channel_bandwidth:
            bandwidth: bw20
        "#;

        let bw: MfgCmd = serde_saphyr::from_str(input).expect("failed to deserialize yaml");

        assert_eq!(
            bw,
            MfgCmd::ChannelBandwidth {
                bandwidth: ChannelBandwidth::Bw20
            }
        );
    }

    #[test]
    fn deserialize_rf_channel_cmd() {
        let input = r#"
        rf_channel:
            channel: 36
        "#;

        let channel: MfgCmd = serde_saphyr::from_str(input).expect("failed to deserialize yaml");

        assert_eq!(channel, MfgCmd::RfChannel { channel: 36 });
    }

    #[test]
    fn deserialize_get_and_reset_per_cmd() {
        let input = r#"
        get_and_reset_per: {}
        "#;

        let per: MfgCmd = serde_saphyr::from_str(input).expect("failed to deserialize yaml");

        assert_eq!(
            per,
            MfgCmd::GetAndResetPer {
                rx_total_packet_count: 0,
                rx_multi_broadcast_packet_count: 0,
                rx_frame_check_sequence_errors: 0
            }
        );
    }

    #[test]
    fn deserialize_tx_power_cmd() {
        let input = r#"
        tx_power:
            power_dbm: 16
            modulation: mcs
            path: a
        "#;

        let power: MfgCmd = serde_saphyr::from_str(input).expect("failed to deserialize yaml");

        assert_eq!(
            power,
            MfgCmd::TxPower {
                power_dbm: 16,
                modulation: Modulation::Mcs,
                path: TxPathId::A,
            }
        );
    }

    #[test]
    fn deserialize_tx_continuous_cmd() {
        let input = r#"
        tx_continuous:
            enable: true
            continuous_wave_mode: false
            payload_pattern: 2730
            cs_mode: false
            active_subchannel: both
            tx_rate: 8
        "#;

        let cont: MfgCmd = serde_saphyr::from_str(input).expect("failed to deserialize yaml");

        assert_eq!(
            cont,
            MfgCmd::TxContinuous {
                enable: true,
                continuous_wave_mode: false,
                payload_pattern: 2730,
                cs_mode: false,
                active_subchannel: ActiveSubchannel::Both,
                tx_rate: 8,
            }
        );
    }

    #[test]
    fn deserialize_tx_continuous_disable_cmd() {
        let input = r#"
        tx_continuous:
            enable: false
        "#;

        let cont: MfgCmd = serde_saphyr::from_str(input).expect("failed to deserialize yaml");

        assert_eq!(
            cont,
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
    fn deserialize_tx_frame_cmd() {
        let input = r#"
        tx_frame:
            enable: true
            tx_rate: 8
            payload_pattern: 2730
            payload_length: 598
            adjust_burst_sifs_gap: false
            adjust_sifs_us: 20
            short_preamble: false
            active_subchannel: lower
            short_gi: false
            adv_coding: false
            beamforming: false
            greenfield_mode: false
            stbc: false
            signal_bw: default
            num_pkt: -1
            max_pkt_ext: -1
            beam_change: -1
            dcm: -1
            doppler: -1
            midamble_period: -1
            q_num: -1
            bssid: [5, 67, 63, 196, 81, 0]
        "#;

        let frame: MfgCmd = serde_saphyr::from_str(input).expect("failed to deserialize yaml");

        assert_eq!(
            frame,
            MfgCmd::TxFrame {
                enable: true,
                tx_rate: 8,
                payload_pattern: 2730,
                payload_length: 598,
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
                bssid: [5, 67, 63, 196, 81, 0],
            }
        );
    }

    #[test]
    fn deserialize_tx_frame_disable_cmd() {
        let input = r#"
        tx_frame:
            enable: false
        "#;

        let frame: MfgCmd = serde_saphyr::from_str(input).expect("failed to deserialize yaml");

        assert_eq!(
            frame,
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
    fn deserialize_trigger_frame_cmd() {
        let input = r#"
        trigger_frame:
            enable: true
            standalone_hetb: trigger_based
            frame_ctrl_type: 1
            frame_ctrl_subtype: 2
            frame_duration: 5484
            trigger_type: 0
            ul_len: 1000
            more_tf: false
            cs_required: false
            ul_bw: bw20
            ltf_type: 1
            ltf_mode: false
            ltf_symbol: 1xheltf
            ul_stbc: false
            ldpc_ess: true
            ap_tx_pwr: 0
            pre_fec_pad_fct: 1
            pe_disambig: false
            spatial_reuse: 65535
            doppler: false
            he_sig2: 511
            aid12: 5
            ru_alloc_reg: false
            ru_alloc: 0
            ul_coding_type: true
            ul_mcs: 8450
            ul_dcm: false
            ss_alloc: 1ss
            ul_target_rssi: 90
            mpdu_mu_sf: 0
            tid_al: 0
            ac_pl: false
            pref_ac: 0
        "#;

        let trigger: MfgCmd = serde_saphyr::from_str(input).expect("failed to deserialize yaml");

        assert_eq!(
            trigger,
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
                ul_mcs: 8450,
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
    fn deserialize_trigger_frame_disable_cmd() {
        let input = r#"
        trigger_frame:
            enable: false
        "#;

        let trigger: MfgCmd = serde_saphyr::from_str(input).expect("failed to deserialize yaml");

        assert_eq!(
            trigger,
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
    fn deserialize_he_tb_tx_cmd() {
        let input = r#"
        he_tb_tx:
            enable: true
            qnum: 1
            aid: 5
            axq0_mu_timer: 400
            tx_pwr: 9
        "#;

        let he_tb: MfgCmd = serde_saphyr::from_str(input).expect("failed to deserialize yaml");

        assert_eq!(
            he_tb,
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
    fn deserialize_he_tb_tx_disable_cmd() {
        let input = r#"
        he_tb_tx:
            enable: false
        "#;

        let he_tb: MfgCmd = serde_saphyr::from_str(input).expect("failed to deserialize yaml");

        assert_eq!(
            he_tb,
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
    fn deserialize_otp_cal_data_write_cmd() {
        let input = r#"
        otp_cal_data:
            write: true
            cal_data: [0x01, 0x02, 0x03, 0x04, 0x05]
        "#;

        let otp: MfgCmd = serde_saphyr::from_str(input).expect("failed to deserialize yaml");

        assert_eq!(
            otp,
            MfgCmd::OtpCalData {
                write: true,
                cal_data: vec![0x01, 0x02, 0x03, 0x04, 0x05],
            }
        );
    }

    #[test]
    fn deserialize_otp_cal_data_read_cmd() {
        let input = r#"
        otp_cal_data:
            write: false
            cal_data: []
        "#;

        let otp: MfgCmd = serde_saphyr::from_str(input).expect("failed to deserialize yaml");

        assert_eq!(
            otp,
            MfgCmd::OtpCalData {
                write: false,
                cal_data: vec![],
            }
        );
    }

    #[test]
    fn deserialize_otp_mac_addr_write_cmd() {
        let input = r#"
        otp_mac_addr:
            write: true
            mac_addr: [0x00, 0x11, 0x22, 0x33, 0x44, 0x55]
        "#;

        let otp: MfgCmd = serde_saphyr::from_str(input).expect("failed to deserialize yaml");

        assert_eq!(
            otp,
            MfgCmd::OtpMacAddr {
                write: true,
                mac_addr: [0x00, 0x11, 0x22, 0x33, 0x44, 0x55],
            }
        );
    }

    #[test]
    fn deserialize_otp_mac_addr_read_cmd() {
        let input = r#"
        otp_mac_addr:
            write: false
            mac_addr: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
        "#;

        let otp: MfgCmd = serde_saphyr::from_str(input).expect("failed to deserialize yaml");

        assert_eq!(
            otp,
            MfgCmd::OtpMacAddr {
                write: false,
                mac_addr: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            }
        );
    }

    #[test]
    fn deserialize_rf_test_mode_command_sequence_examples_vec() {
        let input = r#"
        - rf_test_mode:
            enable: true
        - radio_mode:
            radio0: 0
            radio1: 11
        - tx_antenna:
            mode: a
        - channel_bandwidth:
            bandwidth: bw20
        - rf_channel:
            channel: 6
        - tx_power:
            power_dbm: 8
            modulation: mcs
            path: a
        - tx_continuous:
            enable: true
            continuous_wave_mode: true
            payload_pattern: 2730
            cs_mode: false
            active_subchannel: both
            tx_rate: 8456
        - tx_continuous:
            enable: false
            continuous_wave_mode: true
            payload_pattern: 2730
            cs_mode: false
            active_subchannel: both
            tx_rate: 8456
        "#;

        let sequence: Vec<MfgCmd> =
            serde_saphyr::from_str(input).expect("failed to deserialize yaml");

        assert_eq!(
            sequence,
            vec![
                MfgCmd::RfTestMode { enable: true },
                MfgCmd::RadioMode {
                    radio0: 0,
                    radio1: 11
                },
                MfgCmd::TxAntenna {
                    mode: AntennaMode::A
                },
                MfgCmd::ChannelBandwidth {
                    bandwidth: ChannelBandwidth::Bw20,
                },
                MfgCmd::RfChannel { channel: 6 },
                MfgCmd::TxPower {
                    power_dbm: 8,
                    modulation: Modulation::Mcs,
                    path: TxPathId::A,
                },
                MfgCmd::TxContinuous {
                    enable: true,
                    continuous_wave_mode: true,
                    payload_pattern: 2730,
                    cs_mode: false,
                    active_subchannel: ActiveSubchannel::Both,
                    tx_rate: 8456,
                },
                MfgCmd::TxContinuous {
                    enable: false,
                    continuous_wave_mode: true,
                    payload_pattern: 2730,
                    cs_mode: false,
                    active_subchannel: ActiveSubchannel::Both,
                    tx_rate: 8456,
                },
            ]
        );
    }

    #[test]
    fn deserialize_rf_test_mode_command_sequence_examples_rx_vec() {
        let input = r#"
        - rf_test_mode:
            enable: true
        - radio_mode:
            radio0: 1
            radio1: 0
        - rf_band:
            band: ghz5
        - channel_bandwidth:
            bandwidth: bw40
        - rx_antenna:
            mode: a_b
        - rf_channel:
            channel: 36
        - get_and_reset_per: {}
        "#;

        let sequence: Vec<MfgCmd> =
            serde_saphyr::from_str(input).expect("failed to deserialize yaml");

        assert_eq!(
            sequence,
            vec![
                MfgCmd::RfTestMode { enable: true },
                MfgCmd::RadioMode {
                    radio0: 1,
                    radio1: 0
                },
                MfgCmd::RfBand { band: RfBand::Ghz5 },
                MfgCmd::ChannelBandwidth {
                    bandwidth: ChannelBandwidth::Bw40,
                },
                MfgCmd::RxAntenna {
                    mode: AntennaMode::AB
                },
                MfgCmd::RfChannel { channel: 36 },
                MfgCmd::GetAndResetPer {
                    rx_total_packet_count: 0,
                    rx_multi_broadcast_packet_count: 0,
                    rx_frame_check_sequence_errors: 0
                },
            ]
        );
    }

    fn validate_host_cmd_header(buffer: &[u8], expected_size: u16) -> &[u8] {
        let size_bytes = u16::to_le_bytes(expected_size);
        let command_bytes = HOST_CMD_CMD_MFG_COMMAND.to_le_bytes();

        assert_eq!(
            buffer[0..8],
            [
                command_bytes[0],
                command_bytes[1],
                size_bytes[0],
                size_bytes[1],
                0x00, // Sequence number
                0x00, // Sequence number
                0x00, // Result code
                0x00  // Result code
            ],
            "HostCmd header mismatch: expected size {:#x}",
            expected_size
        );

        // Return the MfgCmdHeader + body
        &buffer[8..]
    }

    fn validate_mfg_cmd_header(
        buffer: &[u8],
        expected_mfg_cmd: u32,
        expected_action: u16,
    ) -> &[u8] {
        let mfg_cmd_bytes = u32::to_le_bytes(expected_mfg_cmd);
        let action_bytes = u16::to_le_bytes(expected_action);

        assert_eq!(
            buffer[0..12],
            [
                mfg_cmd_bytes[0],
                mfg_cmd_bytes[1],
                mfg_cmd_bytes[2],
                mfg_cmd_bytes[3],
                action_bytes[0],
                action_bytes[1],
                0x00, // Device id
                0x00, // Device id
                0x00, // Error code
                0x00, // Error code
                0x00, // Error code
                0x00  // Error code
            ],
            "MfgCmd header mismatch: expected mfg_cmd {:#x} and action {:#x}",
            expected_mfg_cmd,
            expected_action
        );

        // Return the body
        &buffer[12..]
    }

    #[test]
    fn host_cmd_header() {
        let host_cmd: HostCmd<[u8; 4]> = HostCmd::default();

        let body = validate_host_cmd_header(host_cmd.as_bytes(), 12);

        assert_eq!(body.len(), 4);
        assert_eq!(body, &[0, 0, 0, 0]);
    }

    #[test]
    fn mfg_cmd_header() {
        let mfg_cmd_header: MfgCmdHeader<GenericCfg> = MfgCmdHeader::new(
            0xCAFE_B0BA,
            true,
            GenericCfg {
                data1: 0xFFEE_DDCC,
                data2: 0xBBAA_9988,
                data3: 0x7766_5544,
            },
        );

        let body = validate_mfg_cmd_header(mfg_cmd_header.as_bytes(), 0xCAFE_B0BA, 1);

        assert_eq!(
            body,
            &[
                0xCC, 0xDD, 0xEE, 0xFF, 0x88, 0x99, 0xAA, 0xBB, 0x44, 0x55, 0x66, 0x77
            ]
        );

        let mfg_cmd_header: MfgCmdHeader<GenericCfg> = MfgCmdHeader::new(
            0xCAFE_B0BA,
            false,
            GenericCfg {
                data1: 0xFFEE_DDCC,
                data2: 0xBBAA_9988,
                data3: 0x7766_5544,
            },
        );

        let body = validate_mfg_cmd_header(mfg_cmd_header.as_bytes(), 0xCAFE_B0BA, 0);

        assert_eq!(
            body,
            &[
                0xCC, 0xDD, 0xEE, 0xFF, 0x88, 0x99, 0xAA, 0xBB, 0x44, 0x55, 0x66, 0x77
            ]
        );
    }

    // How to read SDIO dump from MWIFIEX driver:
    //
    // XX  [SDIO CMD ] [ HOST CMD HEADER     ] [ MFG
    // 00: 24 00 01 00 89 00 20 00 68 00 00 00 01 00 00 00
    // XX: CMD HEADER            ] [ BODY
    // 10: 01 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    // 20: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00

    // echo "rf_test_mode=1" >> /proc/mwlan/adapter0/config
    // 00: 24 00 01 00 89 00 20 00 68 00 00 00 01 00 00 00
    // 10: 01 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    // 20: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    #[test]
    fn mfg_cmd_rf_test_mode_enable() {
        let host_cmd = MfgCmd::RfTestMode { enable: true }
            .into_host_command_buffer(CardType::IW610)
            .expect("failed to convert into buffer");

        let mfg_header = validate_host_cmd_header(&host_cmd, 32);
        let body = validate_mfg_cmd_header(mfg_header, 0x0001, 0x0001);

        // GenericCfg body (12 bytes): all zeros
        assert_eq!(body, &[0x00; 12]);
    }

    // echo "rf_test_mode=0" >> /proc/mwlan/adapter0/config
    // 00: 24 00 01 00 89 00 20 00 66 00 00 00 00 00 00 00
    // 10: 01 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    // 20: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    #[test]
    fn mfg_cmd_rf_test_mode_disable() {
        let host_cmd = MfgCmd::RfTestMode { enable: false }
            .into_host_command_buffer(CardType::IW610)
            .expect("failed to convert into buffer");

        let mfg_header = validate_host_cmd_header(&host_cmd, 32);
        let body = validate_mfg_cmd_header(mfg_header, 0x0000, 0x0001);

        // GenericCfg body (12 bytes): all zeros
        assert_eq!(body, &[0x00; 12]);
    }

    // echo "tx_antenna=1" >> /proc/mwlan/adapter0/config
    // 00: 24 00 01 00 89 00 20 00 69 00 00 00 04 10 00 00
    // 10: 01 00 00 00 00 00 00 00 01 00 00 00 00 00 00 00
    // 20: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    #[test]
    fn mfg_cmd_tx_antenna() {
        let host_cmd = MfgCmd::TxAntenna {
            mode: AntennaMode::A,
        }
        .into_host_command_buffer(CardType::IW610)
        .expect("failed to convert into buffer");

        let mfg_header = validate_host_cmd_header(&host_cmd, 32);
        let body = validate_mfg_cmd_header(mfg_header, 0x1004, 0x0001);

        // GenericCfg.data1: 0x00000001 (AntennaMode::A)
        assert_eq!(&body[0..4], &[0x01, 0x00, 0x00, 0x00]);
    }

    // echo "rx_antenna=2" >> /proc/mwlan/adapter0/config
    // 00: 24 00 01 00 89 00 20 00 27 00 00 00 05 10 00 00
    // 10: 01 00 00 00 00 00 00 00 02 00 00 00 00 00 00 00
    // 20: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    #[test]
    fn mfg_cmd_rx_antenna() {
        let host_cmd = MfgCmd::RxAntenna {
            mode: AntennaMode::B,
        }
        .into_host_command_buffer(CardType::IW610)
        .expect("failed to convert into buffer");

        let mfg_header = validate_host_cmd_header(&host_cmd, 32);
        let body = validate_mfg_cmd_header(mfg_header, 0x1005, 0x0001);

        // GenericCfg.data1: 0x00000002 (AntennaMode::B)
        assert_eq!(&body[0..4], &[0x02, 0x00, 0x00, 0x00]);
    }

    // echo "radio_mode=3 0" >> /proc/mwlan/adapter0/config
    // 00: 24 00 01 00 89 00 20 00 6c 00 00 00 11 12 00 00
    // 10: 01 00 00 00 00 00 00 00 03 00 00 00 00 00 00 00
    // 20: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    #[test]
    fn mfg_cmd_radio_mode() {
        let host_cmd = MfgCmd::RadioMode {
            radio0: 3,
            radio1: 0,
        }
        .into_host_command_buffer(CardType::IW610)
        .expect("failed to convert into buffer");

        let mfg_header = validate_host_cmd_header(&host_cmd, 32);
        let body = validate_mfg_cmd_header(mfg_header, 0x1211, 0x0001);

        // GenericCfg.data1: radio0 = 3
        assert_eq!(&body[0..4], &[0x03, 0x00, 0x00, 0x00]);
        // GenericCfg.data2: radio1 = 0
        assert_eq!(&body[4..8], &[0x00, 0x00, 0x00, 0x00]);
    }

    // echo "band=1" >> /proc/mwlan/adapter0/config
    // 00: 24 00 01 00 89 00 20 00 6d 00 00 00 34 10 00 00
    // 10: 01 00 00 00 00 00 00 00 01 00 00 00 00 00 00 00
    // 20: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    #[test]
    fn mfg_cmd_rf_band() {
        let host_cmd = MfgCmd::RfBand { band: RfBand::Ghz5 }
            .into_host_command_buffer(CardType::IW610)
            .expect("failed to convert into buffer");

        let mfg_header = validate_host_cmd_header(&host_cmd, 32);
        let body = validate_mfg_cmd_header(mfg_header, 0x1034, 0x0001);

        // GenericCfg.data1: 0x00000001 (RfBand::Ghz5)
        assert_eq!(&body[0..4], &[0x01, 0x00, 0x00, 0x00]);
    }

    // echo "bw=0" >> /proc/mwlan/adapter0/config
    // 00: 24 00 01 00 89 00 20 00 6e 00 00 00 44 10 00 00
    // 10: 01 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    // 20: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    #[test]
    fn mfg_cmd_channel_bandwidth() {
        let host_cmd = MfgCmd::ChannelBandwidth {
            bandwidth: ChannelBandwidth::Bw20,
        }
        .into_host_command_buffer(CardType::IW610)
        .expect("failed to convert into buffer");

        let mfg_header = validate_host_cmd_header(&host_cmd, 32);
        let body = validate_mfg_cmd_header(mfg_header, 0x1044, 0x0001);

        // GenericCfg.data1: 0x00000000 (Bw20)
        assert_eq!(&body[0..4], &[0x00, 0x00, 0x00, 0x00]);
    }

    // echo "channel=6" >> /proc/mwlan/adapter0/config
    // 00: 24 00 01 00 89 00 20 00 6f 00 00 00 0a 10 00 00
    // 10: 01 00 00 00 00 00 00 00 06 00 00 00 00 00 00 00
    // 20: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    #[test]
    fn mfg_cmd_rf_channel() {
        let host_cmd = MfgCmd::RfChannel { channel: 6 }
            .into_host_command_buffer(CardType::IW610)
            .expect("failed to convert into buffer");

        let mfg_header = validate_host_cmd_header(&host_cmd, 32);
        let body = validate_mfg_cmd_header(mfg_header, 0x100A, 0x0001);

        // GenericCfg.data1: 0x0000006 (channel 6)
        assert_eq!(&body[0..4], &[0x06, 0x00, 0x00, 0x00]);
    }

    // echo "get_and_reset_per" >> /proc/mwlan/adapter0/config
    // 00: 24 00 01 00 89 00 20 00 70 00 00 00 10 10 00 00
    // 10: 01 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    // 20: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    #[test]
    fn mfg_cmd_get_and_reset_per() {
        let host_cmd = MfgCmd::GetAndResetPer {
            rx_total_packet_count: 0,
            rx_multi_broadcast_packet_count: 0,
            rx_frame_check_sequence_errors: 0,
        }
        .into_host_command_buffer(CardType::IW610)
        .expect("failed to convert into buffer");

        let mfg_header = validate_host_cmd_header(&host_cmd, 32);
        let _body = validate_mfg_cmd_header(mfg_header, 0x1010, 0x0000);
    }

    // echo "tx_power=16 2 0" >> /proc/mwlan/adapter0/config
    // 00: 24 00 01 00 89 00 20 00 71 00 00 00 33 10 00 00
    // 10: 01 00 00 00 00 00 00 00 00 01 00 00 02 00 00 00
    // 20: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    #[test]
    fn mfg_cmd_tx_power_iw610() {
        let host_cmd = MfgCmd::TxPower {
            power_dbm: 16,
            modulation: Modulation::Mcs,
            path: TxPathId::A,
        }
        .into_host_command_buffer(CardType::IW610)
        .expect("failed to convert into buffer");

        let mfg_header = validate_host_cmd_header(&host_cmd, 32);
        let body = validate_mfg_cmd_header(mfg_header, 0x1033, 0x0001);

        // GenericCfg.data1: power_dbm = 16
        assert_eq!(&body[0..4], &[0x00, 0x01, 0x00, 0x00]);
        // GenericCfg.data2: modulation = 2 (Mcs)
        assert_eq!(&body[4..8], &[0x02, 0x00, 0x00, 0x00]);
        // GenericCfg.data3: path = 0 (TxPathId::A)
        assert_eq!(&body[8..12], &[0x00, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn mfg_cmd_tx_power_8987() {
        let host_cmd = MfgCmd::TxPower {
            power_dbm: 16,
            modulation: Modulation::Mcs,
            path: TxPathId::A,
        }
        .into_host_command_buffer(CardType::_8987)
        .expect("failed to convert into buffer");

        let mfg_header = validate_host_cmd_header(&host_cmd, 32);
        let body = validate_mfg_cmd_header(mfg_header, 0x1033, 0x0001);

        // GenericCfg.data1: power_dbm = 16
        assert_eq!(&body[0..4], &[0x10, 0x00, 0x00, 0x00]);
        // GenericCfg.data2: modulation = 2 (Mcs)
        assert_eq!(&body[4..8], &[0x02, 0x00, 0x00, 0x00]);
        // GenericCfg.data3: path = 0 (TxPathId::A)
        assert_eq!(&body[8..12], &[0x00, 0x00, 0x00, 0x00]);
    }

    // echo "tx_continuous=1 0 0xAAA 0 3 0x8" >> /proc/mwlan/adapter0/config
    // 00: 34 00 01 00 89 00 30 00 73 00 00 00 09 10 00 00
    // 10: 01 00 00 00 00 00 00 00 01 00 00 00 00 00 00 00
    // 20: aa 0a 00 00 00 00 00 00 03 00 00 00 08 00 00 00
    // 30: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    #[test]
    fn mfg_cmd_tx_continuous() {
        let host_cmd = MfgCmd::TxContinuous {
            enable: true,
            continuous_wave_mode: false,
            payload_pattern: 0xAAA,
            cs_mode: false,
            active_subchannel: ActiveSubchannel::Both,
            tx_rate: 8,
        }
        .into_host_command_buffer(CardType::IW610)
        .expect("failed to convert into buffer");

        let mfg_header = validate_host_cmd_header(&host_cmd, 48);
        let body = validate_mfg_cmd_header(mfg_header, 0x1009, 0x0001);

        // TxCont.enable_tx: 1
        assert_eq!(&body[0..4], &[0x01, 0x00, 0x00, 0x00]);
        // TxCont.cw_mode: 0
        assert_eq!(&body[4..8], &[0x00, 0x00, 0x00, 0x00]);
        // TxCont.payload_pattern: 2730 (0x0AAA)
        assert_eq!(&body[8..12], &[0xAA, 0x0A, 0x00, 0x00]);
        // TxCont.cs_mode: 0
        assert_eq!(&body[12..16], &[0x00, 0x00, 0x00, 0x00]);
        // TxCont.act_sub_ch: 3
        assert_eq!(&body[16..20], &[0x03, 0x00, 0x00, 0x00]);
        // TxCont.tx_rate: 8
        assert_eq!(&body[20..24], &[0x08, 0x00, 0x00, 0x00]);
    }

    // echo "tx_frame=1 0x8 0xAAA 0x256 0 20 0 0 0 0 0 0 0 -1 -1 -1 -1 -1 -1 -1 -1 05:43:3f:c4:51" >> /proc/mwlan/adapter0/config
    // 00: 74 00 01 00 89 00 70 00 24 00 00 00 21 10 00 00
    // 10: 01 00 00 00 00 00 00 00 01 00 00 00 08 00 00 00
    // 20: aa 0a 00 00 56 02 00 00 05 43 3f c4 51 ff 00 00
    // 30: 14 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    // 40: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    // 50: 00 00 00 00 ff ff ff ff ff ff ff ff ff ff ff ff
    // 60: ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff
    // 70: ff ff ff ff 00 00 00 00 00 00 00 00 00 00 00 00
    #[test]
    fn mfg_cmd_tx_frame() {
        let host_cmd = MfgCmd::TxFrame {
            enable: true,
            tx_rate: 8,
            payload_pattern: 2730,
            payload_length: 598,
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
            num_pkt: -1i32,
            max_pkt_ext: -1i32,
            beam_change: -1i32,
            dcm: -1i32,
            doppler: -1i32,
            midamble_period: -1i32,
            q_num: -1i32,
            bssid: [0x05, 0x43, 0x3F, 0xC4, 0x51, 0xff],
        }
        .into_host_command_buffer(CardType::IW610)
        .expect("failed to convert into buffer");

        let mfg_header = validate_host_cmd_header(&host_cmd, 112);
        let body = validate_mfg_cmd_header(mfg_header, 0x1021, 0x0001);

        // TxFrame2.enable: 1
        assert_eq!(&body[0..4], &[0x01, 0x00, 0x00, 0x00]);
        // TxFrame2.data_rate: 8
        assert_eq!(&body[4..8], &[0x08, 0x00, 0x00, 0x00]);
        // TxFrame2.frame_pattern: 2730
        assert_eq!(&body[8..12], &[0xAA, 0x0A, 0x00, 0x00]);
        // TxFrame2.frame_length: 598
        assert_eq!(&body[12..16], &[0x56, 0x02, 0x00, 0x00]);
        // TxFrame2.bssid: [0x05, 0x43, 0x3F, 0xC4, 0x51, 0xff]
        assert_eq!(&body[16..22], &[0x05, 0x43, 0x3F, 0xC4, 0x51, 0xff]);
        // TxFrame2.adjust_burst_sifs: 0 (u16)
        assert_eq!(&body[22..24], &[0x00, 0x00]);
        // TxFrame2.burst_sifs_in_us: 20 (0x14)
        assert_eq!(&body[24..28], &[0x14, 0x00, 0x00, 0x00]);
        // TxFrame2.short_preamble: 0
        assert_eq!(&body[28..32], &[0x00, 0x00, 0x00, 0x00]);
        // TxFrame2.act_sub_ch: 0
        assert_eq!(&body[32..36], &[0x00, 0x00, 0x00, 0x00]);
        // TxFrame2.short_gi: 0
        assert_eq!(&body[36..40], &[0x00, 0x00, 0x00, 0x00]);
        // TxFrame2.adv_coding: 0
        assert_eq!(&body[40..44], &[0x00, 0x00, 0x00, 0x00]);
        // TxFrame2.tx_bf: 0
        assert_eq!(&body[44..48], &[0x00, 0x00, 0x00, 0x00]);
        // TxFrame2.gf_mode: 0
        assert_eq!(&body[48..52], &[0x00, 0x00, 0x00, 0x00]);
        // TxFrame2.stbc: 0
        assert_eq!(&body[52..56], &[0x00, 0x00, 0x00, 0x00]);
        // TxFrame2 padding/reserved: 0
        assert_eq!(&body[56..60], &[0x00, 0x00, 0x00, 0x00]);
        // TxFrame2.signal_bw: -1 (0xFFFFFFFF)
        assert_eq!(&body[60..64], &[0xff, 0xff, 0xff, 0xff]);
        // TxFrame2.num_pkt: -1
        assert_eq!(&body[64..68], &[0xff, 0xff, 0xff, 0xff]);
        // TxFrame2.max_pe: -1
        assert_eq!(&body[68..72], &[0xff, 0xff, 0xff, 0xff]);
        // TxFrame2.beam_change: -1
        assert_eq!(&body[72..76], &[0xff, 0xff, 0xff, 0xff]);
        // TxFrame2.dcm: -1
        assert_eq!(&body[76..80], &[0xff, 0xff, 0xff, 0xff]);
        // TxFrame2.doppler: -1
        assert_eq!(&body[80..84], &[0xff, 0xff, 0xff, 0xff]);
        // TxFrame2.mid_p: -1
        assert_eq!(&body[84..88], &[0xff, 0xff, 0xff, 0xff]);
        // TxFrame2.q_num: -1
        assert_eq!(&body[88..92], &[0xff, 0xff, 0xff, 0xff]);
    }

    // echo "he_tb_tx=1 1 5 400 9" >> /proc/mwlan/adapter0/config
    // 00: 22 00 01 00 89 00 1e 00 25 00 00 00 0a 11 00 00
    // 10: 01 00 00 00 00 00 00 00 01 00 01 00 05 00 90 01
    // 20: 09 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    #[test]
    fn mfg_cmd_he_tb_tx() {
        let host_cmd = MfgCmd::HeTbTx {
            enable: true,
            qnum: 1,
            aid: 5,
            axq0_mu_timer: 400,
            tx_pwr: 9,
        }
        .into_host_command_buffer(CardType::IW610)
        .expect("failed to convert into buffer");

        let mfg_header = validate_host_cmd_header(&host_cmd, 30);
        let body = validate_mfg_cmd_header(mfg_header, 0x110A, 0x0001);

        // HeTbTx.enable: 1
        assert_eq!(&body[0..2], &[0x01, 0x00]);
        // HeTbTx.qnum: 1
        assert_eq!(&body[2..4], &[0x01, 0x00]);
        // HeTbTx.aid: 5
        assert_eq!(&body[4..6], &[0x05, 0x00]);
        // HeTbTx.axq_mu_timer: 400
        assert_eq!(&body[6..8], &[0x90, 0x01]);
        // HeTbTx.tx_power: 9
        assert_eq!(&body[8..10], &[0x09, 0x00]);
    }

    #[test]
    fn mfg_cmd_otp_mac_addr() {
        let host_cmd = MfgCmd::OtpMacAddr {
            write: false,
            mac_addr: [0x00, 0x11, 0x22, 0x33, 0x44, 0x55],
        }
        .into_host_command_buffer(CardType::IW610)
        .expect("failed to convert into buffer");

        let mfg_header = validate_host_cmd_header(&host_cmd, 26);
        let body = validate_mfg_cmd_header(mfg_header, 0x108C, 0x0000);

        // OtpMacAddrRdWr.mac_addr
        assert_eq!(body, &[0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
    }

    #[test]
    fn mfg_cmd_otp_cal_data() {
        let cal_data = vec![0xAA; 100]; // 100 bytes of test data
        let host_cmd = MfgCmd::OtpCalData {
            write: true,
            cal_data,
        }
        .into_host_command_buffer(CardType::IW610)
        .expect("failed to convert into buffer");

        let mfg_header = validate_host_cmd_header(&host_cmd, 1428);
        let body = validate_mfg_cmd_header(mfg_header, 0x121A, 0x0001);

        // OtpCalDataRdWr.cal_data_status: 0
        assert_eq!(&body[0..4], &[0x00, 0x00, 0x00, 0x00]);
        // OtpCalDataRdWr.cal_data_len: 100
        assert_eq!(&body[4..8], &[0x64, 0x00, 0x00, 0x00]);
        // First few bytes of cal_data
        assert_eq!(&body[8..18], &[0xAA; 10]);
    }
}
