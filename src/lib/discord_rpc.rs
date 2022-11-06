use std::process::{Child, Command};

#[derive(Clone)]
pub struct DiscordParams {
    pub id: u64,
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

        let id = params.id.to_string();
        let mut args = vec!("-a", id.trim());
        let start_string: String;
        let end_string: String;

        if let Some(details) = &params.details {
            args.extend(["-d", details]);
        }

        if let Some(state) = &params.state {
            args.extend(["-s", state]);
        }

        if let Some(icons) = &params.icons {
            if let Some(large_icon) = &icons.large {
                args.extend(["-li", large_icon]);
            }

            if let Some(small_icon) = &icons.small {
                args.extend(["-si", small_icon]);
            }
        }

        if let Some(time) = &params.time {

            if let Some(start) = &time.start {
                start_string = start.to_string();
                args.extend(["-st", start_string.trim()]);
            }

            if let Some(end) = &time.end {
                end_string = end.to_string();
                args.extend(["-et", end_string.trim()]);
            }
        }

        let cmd = Command::new("./public/discord-rpc/discord-rpc")
            .args(args)
            .spawn()
            .unwrap();

        self.params = Some(params.clone());
        self.process = Some(cmd);
    }

    pub fn stop(&mut self) {
        if !&self.process.is_none() {
            let _ = &self.process.as_mut().unwrap().kill().unwrap();
        }

        ()
    }
}