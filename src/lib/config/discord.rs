use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordState {
    pub details: String,
    pub state: String,
    pub icon: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordStates {
    pub in_launcher: DiscordState,
    pub in_game: DiscordState
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Discord {
    pub enabled: bool,
    pub timer: bool,
    pub states: DiscordStates
}

impl Default for DiscordState {
    fn default() -> Self {
        Self {
            details: "".to_string(),
            state: "".to_string(),
            icon: "".to_string()
        }
    }
}

impl Default for DiscordStates {
    fn default() -> Self {
        Self {
            in_launcher: DiscordState {
                details: "Preparing to launch".to_string(),
                state: "".to_string(),
                icon: "launcher".to_string()
            },
            in_game: DiscordState {
                details: "Exploring the landscape".to_string(),
                state: "of Teyvat".to_string(),
                icon: "game".to_string()
            }
        }
    }
}

impl Default for Discord {
    fn default() -> Self {
        Self {
            enabled: false,
            timer: true,
            states: DiscordStates::default()
        }
    }
}

impl From<&JsonValue> for DiscordState {
    fn from(value: &JsonValue) -> Self {
        let default = Self::default();

        Self {
            details: match value.get("details") {
                Some(value) => value.as_str().unwrap_or(default.details.as_str()).to_string(),
                None => default.details
            },
            state: match value.get("state") {
                Some(value) => value.as_str().unwrap_or(default.state.as_str()).to_string(),
                None => default.state
            },
            icon: match value.get("icon") {
                Some(value) => value.as_str().unwrap_or(default.icon.as_str()).to_string(),
                None => default.icon
            }
        }
    }
}

impl From<&JsonValue> for DiscordStates {
    fn from(value: &JsonValue) -> Self {
        let default = Self::default();

        Self {
            in_launcher: match value.get("in_launcher") {
                Some(value) => DiscordState::from(value),
                None => default.in_launcher
            },
            in_game: match value.get("in_game") {
                Some(value) => DiscordState::from(value),
                None => default.in_game
            }
        }
    }
}

impl From<&JsonValue> for Discord {
    fn from(value: &JsonValue) -> Self {
        let default = Self::default();

        Self {
            enabled: match value.get("enabled") {
                Some(value) => value.as_bool().unwrap_or(default.enabled),
                None => default.enabled
            },
            timer: match value.get("timer") {
                Some(value) => value.as_bool().unwrap_or(default.timer),
                None => default.timer
            },
            states: match value.get("states") {
                Some(value) => DiscordStates::from(value),
                None => default.states
            }
        }
    }
}

impl Discord {

}