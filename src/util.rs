// SPDX-FileCopyrightText: 2026 The mwifiex_nl80211_test_mode authors
// SPDX-License-Identifier: GPL-2.0-only

use std::{cmp::max, error::Error};

use crate::{command::MfgCmd, ffi::MAC_ADDR_LENGTH};

pub fn parse_mac_addr(input: &str) -> Result<[u8; MAC_ADDR_LENGTH], Box<dyn Error>> {
    let bytes = input
        .split(':')
        .map(|segment| u8::from_str_radix(segment, 16))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| format!("Invalid MAC address '{input}': {err}"))?;

    if bytes.len() != MAC_ADDR_LENGTH {
        return Err(
            format!("Invalid MAC address '{input}': expected {MAC_ADDR_LENGTH} bytes").into(),
        );
    }

    bytes
        .try_into()
        .map_err(|_| "failed to construct MAC address".to_string().into())
}

pub fn format_request_response(request: &MfgCmd, response: &MfgCmd) -> String {
    let request = format!("{request:#?}");
    let response = format!("{response:#?}");

    let longest_line = request
        .lines()
        .zip(response.lines())
        .fold(0, |longest_line, (req, resp)| {
            max(max(req.len(), resp.len()), longest_line)
        });

    let lines = max(request.lines().count(), response.lines().count());

    request.lines().zip(response.lines()).enumerate().fold(
        String::new(),
        |mut result, (idx, (req, resp))| {
            result.push_str(&format!(
                "{:width$}  {}  {:width$}\n",
                req,
                if idx == lines / 2 { "=>" } else { "  " },
                resp,
                width = longest_line,
            ));

            result
        },
    )
}
