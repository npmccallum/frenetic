// SPDX-License-Identifier: Apache-2.0

#[cfg(not(target_os = "windows"))]
mod sysv;

#[cfg(not(target_os = "windows"))]
pub use sysv::Context;

#[cfg(target_os = "windows")]
mod win;

#[cfg(target_os = "windows")]
pub use win::Context;
