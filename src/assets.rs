use comfy::*;

pub fn load_sprites(c: &mut EngineContext) {
    c.load_texture_from_bytes(
        "tileset",
        include_bytes!("../assets/tileset.png"),
    );
    c.load_texture_from_bytes(
        "game_logo",
        include_bytes!("../assets/game_logo.png"),
    );
    c.load_texture_from_bytes(
        "player_idle",
        include_bytes!("../assets/player/Character Idle 48x48.png"),
    );
    c.load_texture_from_bytes(
        "player_run",
        include_bytes!("../assets/player/run cycle 48x48.png"),
    );
    c.load_texture_from_bytes(
        "player_climb",
        include_bytes!("../assets/player/player climb-back 48x48.png"),
    );
    c.load_texture_from_bytes(
        "player_jump",
        include_bytes!("../assets/player/player jump 48x48.png"),
    );
    c.load_texture_from_bytes(
        "sprites",
        include_bytes!("../assets/sprite.png"),
    );
    c.load_texture_from_bytes(
        "door",
        include_bytes!("../assets/door_16x32.png"),
    );
}