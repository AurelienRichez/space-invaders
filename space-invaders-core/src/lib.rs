/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate intel_8080_emu;

mod machine;

pub use machine::*;

pub const INVADERS_ROM: &'static [u8] = include_bytes!(env!("ROM_PATH"));