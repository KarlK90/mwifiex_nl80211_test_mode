// SPDX-FileCopyrightText: 2026 The mwifiex_nl80211_test_mode authors
// SPDX-License-Identifier: GPL-2.0-only

use std::fmt::Debug;

use bitfield_struct::bitfield;
use zerocopy::{FromBytes, Immutable, IntoBytes};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes};

pub trait MwifiexMfgCmd: Copy + Debug + IntoBytes + Immutable + Default + FromBytes {
    fn into_host_cmd(self, code: u32, write: bool) -> HostCmd<MfgCmdHeader<Self>>
    where
        Self: Sized,
    {
        HostCmd::from(MfgCmdHeader::new(code, write, self))
    }
}

pub const HOST_CMD_CMD_MFG_COMMAND: u16 = 0x0089;

/** C struct: HostCmd_DS_COMMAND */
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, IntoBytes, Immutable, FromBytes)]
pub struct HostCmd<T> {
    pub command: u16,
    pub size: u16,
    pub seq_num: u16,
    pub result: u16,
    pub body: T,
}

impl<T: Default> Default for HostCmd<T> {
    fn default() -> Self {
        Self {
            command: HOST_CMD_CMD_MFG_COMMAND,
            size: std::mem::size_of::<HostCmd<T>>() as u16,
            seq_num: Default::default(), // Set by mwifiex driver
            result: Default::default(),  // Set by mwifiex driver
            body: Default::default(),
        }
    }
}

impl<T> HostCmd<T>
where
    T: IntoBytes + Immutable,
{
    pub fn to_vec(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl<T: MwifiexMfgCmd + Default> From<MfgCmdHeader<T>> for HostCmd<MfgCmdHeader<T>> {
    fn from(header: MfgCmdHeader<T>) -> Self {
        Self {
            body: header,
            size: std::mem::size_of::<Self>() as u16,
            ..Default::default()
        }
    }
}

#[repr(C, packed)]
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, IntoBytes, Immutable, FromBytes,
)]
pub struct MfgCmdHeader<T> {
    /** MFG command code */
    pub mfg_cmd: u32,
    /** Action */
    pub action: u16,
    /** Device ID - appears to be unused */
    pub device_id: u16,
    /** MFG Error code - filled by firmware response */
    pub error: u32,
    pub body: T,
}

/** General purpose action : Get */
const HOST_CMD_ACT_GEN_GET: u16 = 0;
/** General purpose action : Set */
const HOST_CMD_ACT_GEN_SET: u16 = 1;

impl<T: MwifiexMfgCmd> MfgCmdHeader<T> {
    pub fn new(code: u32, write: bool, body: T) -> Self {
        Self {
            mfg_cmd: code,
            action: if write {
                HOST_CMD_ACT_GEN_SET
            } else {
                HOST_CMD_ACT_GEN_GET
            },
            device_id: Default::default(), // appears to be unused
            error: Default::default(),     // filled by firmware response
            body,
        }
    }
}

/** MFG CMD generic cfg */
// C struct: mfg_cmd_generic_cfg
#[repr(C, packed)]
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, IntoBytes, Immutable, FromBytes,
)]
pub struct GenericCfg {
    /** value 1 */
    pub data1: u32,
    /** value 2 */
    pub data2: u32,
    /** value 3 */
    pub data3: u32,
}
impl MwifiexMfgCmd for GenericCfg {}

/** MFG CMD Tx Frame 2 */
// C struct: mfg_cmd_tx_frame2
#[repr(C, packed)]
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, IntoBytes, Immutable, FromBytes,
)]
pub struct TxFrame2 {
    /** enable */
    pub enable: u32,
    /** data_rate */
    pub data_rate: u32,
    /** frame pattern */
    pub frame_pattern: u32,
    /** frame length */
    pub frame_length: u32,
    /** BSSID */
    pub bssid: [u8; MAC_ADDR_LENGTH],
    /** Adjust burst sifs */
    pub adjust_burst_sifs: u16,
    /** Burst sifs in us*/
    pub burst_sifs_in_us: u32,
    /** short preamble */
    pub short_preamble: u32,
    /** active sub channel */
    pub act_sub_ch: u32,
    /** short GI */
    pub short_gi: u32,
    /** Adv coding */
    pub adv_coding: u32,
    /** Tx beamforming */
    pub tx_bf: u32,
    /** HT Greenfield Mode*/
    pub gf_mode: u32,
    /** STBC */
    pub stbc: u32,
    /** power id */
    pub rsvd: [u32; 1],
    /**signal bw*/
    pub signal_bw: u32,
    /** NumPkt */
    pub num_pkt: u32,
    /** MaxPE */
    pub max_pe: u32,
    /** BeamChange */
    pub beam_change: u32,
    /** Dcm */
    pub dcm: u32,
    /** Doppler */
    pub doppler: u32,
    /** MidP */
    pub mid_p: u32,
    /** QNum */
    pub q_num: u32,
}
impl MwifiexMfgCmd for TxFrame2 {}

// /* MFG CMD Tx Continuous */
// C struct: mfg_cmd_tx_cont
#[repr(C, packed)]
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, IntoBytes, Immutable, FromBytes,
)]
pub struct TxCont {
    /** enable Tx */
    pub enable_tx: u32,
    /** Continuous Wave mode */
    pub cw_mode: u32,
    /** payload pattern */
    pub payload_pattern: u32,
    /** CS Mode */
    pub cs_mode: u32,
    /** active sub channel */
    pub act_sub_ch: u32,
    /** Tx rate */
    pub tx_rate: u32,
    /** power id */
    pub rsvd: u32,
}
impl MwifiexMfgCmd for TxCont {}

// C struct: mfg_Cmd_HE_TBTx_t
#[repr(C, packed)]
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, IntoBytes, Immutable, FromBytes,
)]
pub struct HeTbTx {
    /** Enable Tx */
    pub enable: u16,
    /** Q num */
    pub qnum: u16,
    /** AID */
    pub aid: u16,
    /** AXQ Mu Timer */
    pub axq_mu_timer: u16,
    /** Tx Power */
    pub tx_power: i16,
}
impl MwifiexMfgCmd for HeTbTx {}
#[bitfield(u64)]
#[derive(PartialEq, Eq, PartialOrd, Ord, IntoBytes, Immutable, FromBytes)]
pub struct IeeetypesHeTrigComInfo {
    #[bits(4)]
    pub trigger_type: u64,
    #[bits(12)]
    pub ul_len: u64,
    #[bits(1)]
    pub more_tf: u64,
    #[bits(1)]
    pub cs_required: u64,
    #[bits(2)]
    pub ul_bw: u64,
    #[bits(2)]
    pub ltf_type: u64,
    #[bits(1)]
    pub ltf_mode: u64,
    #[bits(3)]
    pub ltf_symbol: u64,
    #[bits(1)]
    pub ul_stbc: u64,
    #[bits(1)]
    pub ldpc_ess: u64,
    #[bits(6)]
    pub ap_tx_pwr: u64,
    #[bits(2)]
    pub pre_fec_pad_fct: u64,
    #[bits(1)]
    pub pe_disambig: u64,
    #[bits(16)]
    pub spatial_reuse: u64,
    #[bits(1)]
    pub doppler: u64,
    #[bits(9)]
    pub he_sig2: u64,
    #[bits(1)]
    pub reserved: u64,
}
impl MwifiexMfgCmd for IeeetypesHeTrigComInfo {}

#[bitfield(u32)]
#[derive(PartialEq, Eq, PartialOrd, Ord, IntoBytes, Immutable, FromBytes)]
pub struct IeeetypesHeTrigUserInfoBits {
    #[bits(12)]
    pub aid12: u32,
    #[bits(1)]
    pub ru_alloc_reg: u32,
    #[bits(7)]
    pub ru_alloc: u32,
    #[bits(1)]
    pub ul_coding_type: u32,
    #[bits(4)]
    pub ul_mcs: u32,
    #[bits(1)]
    pub ul_dcm: u32,
    #[bits(6)]
    pub ss_alloc: u32,
}
impl MwifiexMfgCmd for IeeetypesHeTrigUserInfoBits {}

#[bitfield(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, IntoBytes, Immutable, FromBytes)]
pub struct IeeetypesHeTrigUserInfoRssi {
    #[bits(7)]
    pub ul_target_rssi: u8,
    #[bits(1)]
    pub reserved: u8,
}
impl MwifiexMfgCmd for IeeetypesHeTrigUserInfoRssi {}

// C struct: mfg_cmd_IEEEtypes_HETrigUserInfo_t
#[repr(C, packed)]
#[derive(
    Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, IntoBytes, Immutable, FromBytes,
)]
pub struct IeeetypesHeTrigUserInfo {
    pub bits: IeeetypesHeTrigUserInfoBits,
    pub rssi: IeeetypesHeTrigUserInfoRssi,
}
impl MwifiexMfgCmd for IeeetypesHeTrigUserInfo {}

#[bitfield(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, IntoBytes, Immutable, FromBytes)]
pub struct IeeetypesBasicHeTrigUserInfo {
    #[bits(2)]
    pub mpdu_mu_sf: u8,
    #[bits(3)]
    pub tid_al: u8,
    #[bits(1)]
    pub ac_pl: u8,
    #[bits(2)]
    pub pref_ac: u8,
}
impl MwifiexMfgCmd for IeeetypesBasicHeTrigUserInfo {}

#[bitfield(u16)]
#[derive(PartialEq, Eq, PartialOrd, Ord, IntoBytes, Immutable, FromBytes)]
pub struct IeeetypesFrameCtrl {
    #[bits(2)]
    pub protocol_version: u16,
    #[bits(2)]
    pub frame_type: u16,
    #[bits(4)]
    pub sub_type: u16,
    #[bits(1)]
    pub to_ds: u16,
    #[bits(1)]
    pub from_ds: u16,
    #[bits(1)]
    pub more_frag: u16,
    #[bits(1)]
    pub retry: u16,
    #[bits(1)]
    pub pwr_mgmt: u16,
    #[bits(1)]
    pub more_data: u16,
    #[bits(1)]
    pub wep: u16,
    #[bits(1)]
    pub order: u16,
}
impl MwifiexMfgCmd for IeeetypesFrameCtrl {}

pub const MAC_ADDR_LENGTH: usize = 6;

// C struct: mfg_Cmd_IEEEtypes_CtlBasicTrigHdr_t
#[repr(C, packed)]
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, IntoBytes, Immutable, FromBytes,
)]
pub struct IeeetypesCtlBasicTrigHdr {
    /** enable Tx*/
    pub enable_tx: u32,
    /** enable Stand Alone HE TB */
    pub standalone_hetb: u32,
    /** Frame Control */
    pub frm_ctl: IeeetypesFrameCtrl,
    /** Duration */
    pub duration: u16,
    /** Destination MAC Address */
    pub dest_addr: [u8; MAC_ADDR_LENGTH],
    /** Source MAC Address */
    pub src_addr: [u8; MAC_ADDR_LENGTH],
    /** Common Info Field **/
    pub trig_common_field: IeeetypesHeTrigComInfo,
    /** User Info Field **/
    pub trig_user_info_field: IeeetypesHeTrigUserInfo,
    /** Trigger Dependent User Info Field **/
    pub basic_trig_user_info: IeeetypesBasicHeTrigUserInfo,
}
impl MwifiexMfgCmd for IeeetypesCtlBasicTrigHdr {}

// C struct: mfg_cmd_otp_mac_addr_rd_wr_t
#[repr(C, packed)]
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, IntoBytes, Immutable, FromBytes,
)]
pub struct OtpMacAddrRdWr {
    /** Destination MAC Address */
    pub mac_addr: [u8; MAC_ADDR_LENGTH],
}
impl MwifiexMfgCmd for OtpMacAddrRdWr {}

pub const CAL_DATA_LEN: usize = 1400;

// C struct: mfg_cmd_otp_cal_data_rd_wr_t
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, IntoBytes, Immutable, FromBytes)]
pub struct OtpCalDataRdWr {
    /** CAL Data write status */
    pub cal_data_status: u32,
    /** CAL Data Length*/
    pub cal_data_len: u32,
    /** Destination MAC Address */
    pub cal_data: [u8; CAL_DATA_LEN],
}
impl MwifiexMfgCmd for OtpCalDataRdWr {}

impl Default for OtpCalDataRdWr {
    fn default() -> Self {
        Self {
            cal_data_status: Default::default(),
            cal_data_len: Default::default(),
            cal_data: [0; CAL_DATA_LEN],
        }
    }
}
