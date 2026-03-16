// SPDX-FileCopyrightText: 2026 The mwifiex_nl80211_test_mode authors
// SPDX-License-Identifier: GPL-2.0-only

use std::error::Error;
use std::iter::once;

use colored::*;
use neli::router::synchronous::{NlRouter, NlRouterReceiverHandle};
use neli::types::GenlBuffer;
use neli::{
    attr::Attribute,
    consts::{
        nl::{GenlId, NlmF},
        socket::NlFamily,
    },
    genl::{AttrTypeBuilder, NlattrBuilder},
    genl::{Genlmsghdr, GenlmsghdrBuilder, NoUserHeader},
    nl::NlPayload,
    utils::Groups,
};
use nix::net::if_::if_nametoindex;

use crate::command::{CardType, MfgCmd};

pub trait MwifiexNetlinkInterface {
    fn send_mfg_cmd(&self, cmd: &MfgCmd) -> Result<MfgCmd, Box<dyn Error>>;
}

#[neli::neli_enum(serialized_type = "u8")]
pub enum Nl80211Command {
    GetInterface = 5,
    Testmode = 45,
}
impl neli::consts::genl::Cmd for Nl80211Command {}

#[neli::neli_enum(serialized_type = "u16")]
pub enum Nl80211Attribute {
    Wiphy = 1,
    IfIndex = 3,
    Testdata = 69,
    Wdev = 153,
}
impl neli::consts::genl::NlAttrType for Nl80211Attribute {}

#[neli::neli_enum(serialized_type = "u16")]
pub enum Nl80211MwifiexTmAttribute {
    Cmd = 1,
    Data = 2,
}
impl neli::consts::genl::NlAttrType for Nl80211MwifiexTmAttribute {}

#[repr(u32)]
pub enum Nl80211MwifiexTmCommand {
    HostCmd = 0,
}

pub struct MwifiexNetlinkHandle {
    sock: NlRouter,
    family_id: u16,
    wiphy: u32,
    ifindex: u32,
    wdev: u64,
    card_type: CardType,
}

impl MwifiexNetlinkHandle {
    pub fn from_interface(interface: &str, card_type: CardType) -> Result<Self, Box<dyn Error>> {
        let ifindex = if_nametoindex(interface).map_err(|err| {
            format!("Interface {interface} is unknown, is mwifiex driver running?: {err}")
        })?;

        // Initialize NlRouter and resolve 'nl80211' genl family
        let (sock, _) = NlRouter::connect(
            NlFamily::Generic, /* family */
            Some(0),           /* pid */
            Groups::empty(),   /* groups */
        )?;
        let family_id = sock.resolve_genl_family("nl80211")?;

        let attrs = once(
            NlattrBuilder::default()
                .nla_type(
                    AttrTypeBuilder::default()
                        .nla_type(Nl80211Attribute::IfIndex)
                        .build()?,
                )
                .nla_payload(ifindex)
                .build()?,
        )
        .collect::<GenlBuffer<_, _>>();

        let recv: NlRouterReceiverHandle<GenlId, Genlmsghdr<Nl80211Command, Nl80211Attribute>> =
            sock.send(
                family_id,
                NlmF::ACK,
                NlPayload::Payload(
                    GenlmsghdrBuilder::<Nl80211Command, Nl80211Attribute, NoUserHeader>::default()
                        .cmd(Nl80211Command::GetInterface)
                        .attrs(attrs)
                        .version(1)
                        .build()?,
                ),
            )?;

        let mut wiphy = None;
        let mut wdev = None;

        for msg in recv {
            let msg = msg?;

            let payload: &Genlmsghdr<Nl80211Command, Nl80211Attribute> =
                if let NlPayload::Payload(p) = msg.nl_payload() {
                    p
                } else {
                    continue;
                };

            let attr_handle = payload.attrs().get_attr_handle();
            for attr in attr_handle.iter() {
                match attr.nla_type().nla_type() {
                    Nl80211Attribute::Wiphy => {
                        wiphy.replace(attr.get_payload_as::<u32>()?);
                    }
                    Nl80211Attribute::Wdev => {
                        wdev.replace(attr.get_payload_as::<u64>()?);
                    }
                    _ => (),
                }
            }
        }

        if let (Some(wiphy), Some(wdev)) = (wiphy, wdev) {
            Ok(Self {
                sock,
                family_id,
                wiphy,
                wdev,
                ifindex,
                card_type,
            })
        } else {
            Err(format!(
                "Couldn't determine wiphy and wdev for interface {interface} with ifindex {ifindex}"
            )
            .into())
        }
    }

    fn send_mfg_cmd_raw(&self, cmd: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let attrs = vec![
            NlattrBuilder::default()
                .nla_type(
                    AttrTypeBuilder::default()
                        .nla_type(Nl80211Attribute::IfIndex)
                        .build()?,
                )
                .nla_payload(self.ifindex)
                .build()?,
            NlattrBuilder::default()
                .nla_type(
                    AttrTypeBuilder::default()
                        .nla_type(Nl80211Attribute::Wdev)
                        .build()?,
                )
                .nla_payload(self.wdev)
                .build()?,
            NlattrBuilder::default()
                .nla_type(
                    AttrTypeBuilder::default()
                        .nla_type(Nl80211Attribute::Wiphy)
                        .build()?,
                )
                .nla_payload(self.wiphy)
                .build()?,
            NlattrBuilder::default()
                .nla_type(
                    AttrTypeBuilder::default()
                        .nla_type(Nl80211Attribute::Testdata)
                        .build()?,
                )
                .nla_payload(Vec::<u8>::new())
                .build()?
                .nest(
                    &NlattrBuilder::default()
                        .nla_type(
                            AttrTypeBuilder::default()
                                .nla_type(Nl80211MwifiexTmAttribute::Cmd)
                                .build()?,
                        )
                        .nla_payload(Nl80211MwifiexTmCommand::HostCmd as u32)
                        .build()?,
                )?
                .nest(
                    &NlattrBuilder::default()
                        .nla_type(
                            AttrTypeBuilder::default()
                                .nla_type(Nl80211MwifiexTmAttribute::Data)
                                .build()?,
                        )
                        .nla_payload(cmd)
                        .build()?,
                )?,
        ]
        .into_iter()
        .collect();

        let recv: NlRouterReceiverHandle<GenlId, Genlmsghdr<Nl80211Command, Nl80211Attribute>> =
            self.sock.send(
                self.family_id,
                NlmF::ACK,
                NlPayload::Payload(
                    GenlmsghdrBuilder::<Nl80211Command, Nl80211Attribute, NoUserHeader>::default()
                        .cmd(Nl80211Command::Testmode)
                        .attrs(attrs)
                        .version(1)
                        .build()?,
                ),
            )?;

        for msg in recv {
            let Ok(msg) =
                msg.inspect_err(|err| println!("{}", format!("Netlink Error: {err}").red()))
            else {
                continue;
            };

            let payload: &Genlmsghdr<Nl80211Command, Nl80211Attribute> =
                if let NlPayload::Payload(p) = msg.nl_payload() {
                    p
                } else {
                    continue;
                };

            if let Some(attr) = payload
                .attrs()
                .get_attr_handle()
                .iter()
                .find(|attr| attr.nla_type().nla_type() == &Nl80211Attribute::Testdata)
                && let Some(attr) = attr
                    .get_attr_handle::<Nl80211MwifiexTmAttribute>()?
                    .iter()
                    .find(|attr| attr.nla_type().nla_type() == &Nl80211MwifiexTmAttribute::Data)
            {
                return Ok(Vec::from(attr.payload().as_ref()));
            }
        }

        Err("No response for message!".into())
    }
}

impl MwifiexNetlinkInterface for MwifiexNetlinkHandle {
    fn send_mfg_cmd(&self, cmd: &MfgCmd) -> Result<MfgCmd, Box<dyn Error>> {
        let response = self.send_mfg_cmd_raw(&cmd.into_host_command_buffer(self.card_type)?)?;

        cmd.from_host_command_response(&response)
    }
}

pub struct MwifiexDryRunHandle {
    pub card_type: CardType,
}

impl MwifiexNetlinkInterface for MwifiexDryRunHandle {
    fn send_mfg_cmd(&self, cmd: &MfgCmd) -> Result<MfgCmd, Box<dyn std::error::Error>> {
        let buffer = cmd.into_host_command_buffer(self.card_type)?;
        cmd.from_host_command_response(&buffer)
    }
}
