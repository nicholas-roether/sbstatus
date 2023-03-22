use mpris::{DBusError, Metadata, PlaybackStatus, Player, PlayerFinder};

#[derive(Debug, PartialEq, Eq)]
pub struct RunningPlayerData {
    pub title: String,
    pub artist: Option<String>,
    pub playing: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PlayerData {
    Stopped,
    Running(RunningPlayerData),
}

pub struct PlayerDataCollector {
    player_finder: PlayerFinder,
    active_player: Option<Player>,
    player_data: Option<PlayerData>,
}

impl PlayerDataCollector {
    pub fn new() -> Result<Self, DBusError> {
        Ok(Self {
            player_finder: PlayerFinder::new()?,
            active_player: None,
            player_data: None,
        })
    }

    fn refresh(&mut self) {
        self.refresh_active_player();
        self.refresh_player_data();
    }

    pub fn poll_player_data(&mut self) -> &'_ Option<PlayerData> {
        self.refresh();
        &self.player_data
    }

    fn refresh_active_player(&mut self) {
        if let Some(player) = &self.active_player {
            if !player.is_running() {
                self.active_player = None;
            }
        }
        if self.active_player.is_none() {
            if let Ok(new_active) = self.player_finder.find_active() {
                self.active_player = Some(new_active);
            }
        }
    }

    fn fetch_new_player_data(player: &Player) -> Option<PlayerData> {
        let Ok(meta) = player.get_metadata() else {
            return None;
        };

        let Ok(status) = player.get_playback_status() else {
            return None;
        };

        if status == PlaybackStatus::Stopped {
            return Some(PlayerData::Stopped);
        }

        Some(PlayerData::Running(RunningPlayerData {
            title: meta.title().unwrap_or("<Unknown Title>").to_string(),
            artist: meta.artists().map(|strs| strs.join(", ")),
            playing: status == PlaybackStatus::Playing,
        }))
    }

    fn refresh_player_data(&mut self) {
        self.player_data = if let Some(player) = &self.active_player {
            Self::fetch_new_player_data(player)
        } else {
            None
        }
    }
}
