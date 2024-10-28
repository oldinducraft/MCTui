#[derive(Debug, Default, Clone)]
pub struct GameArgs {
    pub player:                 Player,
    /// IP address of a server
    pub quick_play_multiplayer: String,
}

#[derive(Debug, Default, Clone)]
pub struct Player {
    /// To use authlib set to "mojang"
    pub user_type:    String,
    pub username:     String,
    pub uuid:         String,
    pub access_token: String,
}
