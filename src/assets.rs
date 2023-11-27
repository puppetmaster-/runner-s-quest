use std::time::Duration;
use comfy::kira::tween::{Easing, Tween};
use comfy::{EngineContext, load_sound_from_bytes, quad_in_out, StaticSoundSettings};
use comfy::kira::StartTime;
use comfy::kira::tween::Easing::InOutPowf;

pub fn load_music(){
    load_sound_from_bytes(
        "game_music",
        include_bytes!("../assets/music/MUSIC.ogg"),
        StaticSoundSettings::new().loop_region(..)
            .volume(0.2)
            .fade_in_tween(Some(Tween{
                start_time: StartTime::Immediate,
                duration: Duration::from_secs(5),
                easing: InOutPowf(10.0),
            }))
    );
}

pub fn load_sound(){
    load_sound_from_bytes(
        "key_collected",
        include_bytes!("../assets/sound/key_collected.ogg"),
        StaticSoundSettings::default(),
    );
    load_sound_from_bytes(
        "ladder_collected",
        include_bytes!("../assets/sound/ladder_collected.ogg"),
        StaticSoundSettings::default(),
    );
    load_sound_from_bytes(
        "welcome",
        include_bytes!("../assets/sound/welcome.ogg"),
        StaticSoundSettings::default(),
    );
    load_sound_from_bytes(
        "door_open",
        include_bytes!("../assets/sound/door_open.ogg"),
        StaticSoundSettings::default(),
    );
    load_sound_from_bytes(
        "hello",
        include_bytes!("../assets/sound/hello.ogg"),
        StaticSoundSettings::default(),
    );
}

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
        "player_hang",
        include_bytes!("../assets/player/player hanging 48x48.png"),
    );
    c.load_texture_from_bytes(
        "sprites",
        include_bytes!("../assets/sprite.png"),
    );
    c.load_texture_from_bytes(
        "door",
        include_bytes!("../assets/door_16x32.png"),
    );
    c.load_texture_from_bytes(
        "particle",
        include_bytes!("../assets/sprite_5.png"),
    );
}