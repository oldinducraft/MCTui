#[derive(Debug, Default, Clone)]
pub struct GameArgs {
    pub player: Player,
    pub quick_play_multiplayer: String,
}

#[derive(Debug, Default, Clone)]
pub struct Player {
    pub user_type:    String,
    pub username:     String,
    pub uuid:         String,
    pub access_token: String,
}