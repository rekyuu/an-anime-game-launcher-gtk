use std::process::{Child, Command};
use crate::lib;

#[derive(Clone)]
pub struct DiscordParams {
    pub id: Option<u64>,
    pub details: Option<String>,
    pub state: Option<String>,
    pub icons: Option<DiscordIcons>,
    pub time: Option<DiscordTime>
}

#[derive(Clone)]
pub struct DiscordIcons {
    pub small: Option<String>,
    pub large: Option<String>
}

#[derive(Clone)]
pub struct DiscordTime {
    pub start: Option<u64>,
    pub end: Option<u64>
}

#[derive(Clone)]
pub struct DiscordRpc {
    params: Option<DiscordParams>,
    process: Option<Child>
}

impl Default for DiscordRpc {
    fn default() -> Self {
        Self {
            params: None,
            process: None
        }
    }
}

impl DiscordRpc {
    pub fn update_rpc(&mut self, params: DiscordParams) {
        self.stop();

        let cmd = Command::new("./public/discord-rpc/discord-rpc")
            .arg("-a").arg(params.id);

        if let Some(details) = &params.details {
            cmd.arg("-d").arg(details);
        }

        if let Some(state) = &params.state {
            cmd.arg("-s").arg(state);
        }

        if let Some(icons) = &params.icons {
            if let Some(large_icon) = &icons.large {
                cmd.arg("-li").arg(large_icon);
            }

            if let Some(small_icon) = &icons.small {
                cmd.arg("-si").arg(small_icon);
            }
        }

        if let Some(time) = &params.time {
            if let Some(start) = &time.start {
                cmd.arg("-st").arg(start);
            }

            if let Some(end) = &time.end {
                cmd.arg("-et").arg(end);
            }
        }

        self.params = Some(params.clone());
        self.process = Some(cmd.spawn()?);
    }

    pub fn stop(&mut self) {
        if self.process != None {
            self.process.kill()?
        }
    }
}