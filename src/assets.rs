use std::time::Duration;
use comfy::kira::tween::{Tween};
use comfy::kira::StartTime;
use comfy::kira::tween::Easing::InOutPowf;
use comfy::{EngineContext, load_sound_from_bytes, random_range, roundtrip, Sprite, StaticSoundSettings, Transform, vec2, WHITE, world};
use comfy::Vec2;
use comfy::commands;
use crate::state::GameState;

pub fn load_music(){
    load_sound_from_bytes(
        "game_music",
        include_bytes!("../assets/music/MUSIC.ogg"),
        StaticSoundSettings::new().loop_region(..)
            .volume(0.1)
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
        include_bytes!("../assets/sound/present_collected.ogg"),
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
    load_sound_from_bytes(
        "congrats",
        include_bytes!("../assets/sound/congraz.ogg"),
        StaticSoundSettings::default(),
    );
    load_sound_from_bytes(
        "thank_you",
        include_bytes!("../assets/sound/thank_you.ogg"),
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
        "thank_you",
        include_bytes!("../assets/thank_you.png"),
    );
    c.load_texture_from_bytes(
        "bg_sky",
        include_bytes!("../assets/background_color_sky.png"),
    );
    c.load_texture_from_bytes(
        "created",
        include_bytes!("../assets/created.png"),
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

pub fn spawn_logo(pos: Vec2) {
    commands().spawn((
        Transform::position(pos),
        get_tween(),
        Sprite::new("game_logo".to_string(), vec2(128.0 * 2.0, 48.0 * 2.0), 100, WHITE),
    ));
}

pub fn get_tween() -> comfy::Tween {
    comfy::Tween::new(-0.1, 0.1, random_range(5.0, 8.0), 0.0, roundtrip)
}

pub fn update(_state: &mut GameState, c: &mut EngineContext) {
    let dt = c.delta;

    for (_, (_, transform, tween)) in
    world().query::<(&Sprite, &mut Transform, &mut comfy::Tween)>().iter()
    {
        tween.update(dt);
        if tween.is_finished() {
            *tween = get_tween();
        }
        transform.position += vec2(0.0, tween.value())
    }
}