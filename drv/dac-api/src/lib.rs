// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! API crate for Hash server.

#![no_std]

use derive_idol_err::IdolError;
use userlib::{sys_send, FromPrimitive};


/// Errors that can be produced from the hash server API.
///
/// This enumeration doesn't include errors that result from configuration
/// issues, like sending host flash messages to some other task.
#[derive(
    Copy, Clone, Debug, FromPrimitive, Eq, PartialEq, IdolError, counters::Count,
)]
pub enum DacError {
    NotInitialized = 1,

    #[idol(server_death)]
    ServerRestarted,
}

include!(concat!(env!("OUT_DIR"), "/client_stub.rs"));
