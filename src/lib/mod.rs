pub mod consts;
pub mod config;
pub mod game;
pub mod dxvk;
pub mod wine;
pub mod launcher;
pub mod prettify_bytes;
pub mod fps_unlocker;
pub mod discord_rpc;

use std::process::{Command, Stdio};

/// Check if specified binary is available
/// 
/// ```
/// use crate::lib;
/// 
/// assert!(lib::is_available("bash"));
/// ```
#[allow(unused_must_use)]
pub fn is_available(binary: &str) -> bool {
    match Command::new(binary).stdout(Stdio::null()).stderr(Stdio::null()).spawn() {
        Ok(mut child) => {
            child.kill();

            true
        },
        Err(_) => false
    }
}
