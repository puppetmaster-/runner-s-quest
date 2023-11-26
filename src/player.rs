use comfy::*;
use crate::{door, items};

use crate::state::GameState;
use crate::tilemap::tilemap_helper::{get_id, is_air, is_ladder, is_ladder_or, is_missing_ladder, is_not_wall, set_ladder_at};

use crate::tilemap::tilemap_helper::TILEMAP_ORIGIN;

pub struct Player;

pub fn spawns(state: &mut GameState) {
    let player_spawn_pos = state.tilemap.get_all_position_from_id(state.tilemap.get_layer_id("logic"), &16);
    spawn(player_spawn_pos[0] + TILEMAP_ORIGIN + vec2(0.0, -11.0));
}

pub fn spawn(pos: Vec2) {
    commands().spawn((
        Transform::position(pos),
        Player,
        AnimatedSpriteBuilder::new()
            .z_index(10)
            .add_animation("idle", 0.1, true, AnimationSource::Spritesheet {
                name: "player_idle".into(),
                spritesheet: Spritesheet {
                    rows: 1,
                    columns: 10,
                },
            })
            .add_animation("run", 0.1, true, AnimationSource::Spritesheet {
                name: "player_run".into(),
                spritesheet: Spritesheet {
                    rows: 1,
                    columns: 8,
                },
            })
            .add_animation("climb", 0.15, true, AnimationSource::Spritesheet {
                name: "player_climb".into(),
                spritesheet: Spritesheet {
                    rows: 1,
                    columns: 4,
                },
            })
            .add_animation("hang", 0.15, true, AnimationSource::Spritesheet {
                name: "player_hang".into(),
                spritesheet: Spritesheet {
                    rows: 1,
                    columns: 4,
                },
            })
            .add_animation("climb_idle", 0.1, true, AnimationSource::Spritesheet {
                name: "player_climb".into(),
                spritesheet: Spritesheet {
                    rows: 1,
                    columns: 4,
                },
            })
            .add_animation("fall", 0.1, true, AnimationSource::Atlas {
                name: "player_jump".into(),
                offset: ivec2(96, 0),
                step: ivec2(1, 0),
                size: isplat(48),
                frames: 1,
            })
            .size(splat(40.0))
            .blend_mode(BlendMode::Alpha)
            .build(),
    ));
}

pub fn handle_input(state: &mut GameState, c: &mut EngineContext) {
    let dt = c.delta;

    for (_, (_, animated_sprite, transform)) in
    world().query::<(&Player, &mut AnimatedSprite, &mut Transform)>().iter()
    {

        let pos = vec2(transform.position.x, transform.position.y - 5.0);

        items::pickup(state,&pos);

        let check_move_left = pos + vec2(-8.0, 0.0);
        let check_move_right = pos + vec2(8.0, 0.0);
        let check_ladder_pos = pos + vec2(0.0, -7.0);
        let check_ladder_pos_down = pos + vec2(0.0, -9.0);
        let check_ladder_pos_head = pos + vec2(0.0, 8.0);
        let pos_floor_1 = pos - vec2(7.0, 8.0);
        let pos_floor_2 = pos + vec2(7.0, -8.0);
        let pos_floor_3 = pos - vec2(7.0, 9.0);
        let pos_floor_4 = pos + vec2(7.0, -9.0);
        let id_middle = get_id(state, pos);
        let id_ladder_up = get_id(state, check_ladder_pos);
        let id_ladder_down = get_id(state, check_ladder_pos_down);
        let _id_ladder_head = get_id(state, check_ladder_pos_head);
        let id_floor_1 = get_id(state, pos_floor_1);
        let id_floor_2 = get_id(state, pos_floor_2);
        let id_floor_3 = get_id(state, pos_floor_3);
        let id_floor_4 = get_id(state, pos_floor_4);
        let id_move_left = get_id(state, check_move_left);
        let id_move_right = get_id(state, check_move_right);

        let mut moved = false;
        let mut climb = false;
        let mut fall = false;
        let speed = 60.0;
        let mut move_dir = Vec2::ZERO;

        if (is_air(id_floor_1) && is_air(id_floor_2)) || (is_air(id_floor_3) && is_air(id_floor_4)) {
            move_dir.y -= 1.0;
            fall = true;
        } else {
            if is_missing_ladder(id_middle) && is_action_up() && state.give_ladder(){
                set_ladder_at(state,pos);
                move_dir.y += 1.0;
                climb = true;
            }
            if is_action_up() && (is_ladder(id_ladder_up) || is_ladder(id_middle)) {
                move_dir.y += 1.0;
                climb = true;
            }
            if is_action_down() && is_ladder(id_ladder_down) {
                move_dir.y -= 1.0;
                climb = true;
            }
            if is_action_left() {
                move_dir.x -= 1.0;
                moved = true;
            }
            if is_action_right() {
                move_dir.x += 1.0;
                moved = true;
            }
        }

        if moved {
            animated_sprite.flip_x = move_dir.x < 0.0;
            let dis = move_dir.normalize_or_zero() * speed * dt;
            let id_left = get_id(state, check_move_left + dis);
            let id_right = get_id(state, check_move_right + dis);
            if move_dir.x < 0.0 && is_not_wall(id_left) || move_dir.x > 0.0 && is_not_wall(id_right) {
                transform.position += dis;
                if !(are_we_climbing(animated_sprite) && is_ladder_or(&[id_middle, id_floor_1, id_floor_2, id_right, id_left])) {
                    animated_sprite.play("run");
                }
            }
        } else if climb {
            transform.position += move_dir.normalize_or_zero() * (speed - 20.0) * dt;
            if animated_sprite.state.animation_name == "climb_idle" {
                animated_sprite.play("climb");
                animated_sprite.state.timer = state.climb_timer;
            } else {
                animated_sprite.play("climb");
            }
        } else if fall {

            let dis = move_dir.normalize_or_zero() * (speed + 20.0) * dt;
            transform.position += dis;
            animated_sprite.play("fall");
        } else {
            if is_ladder_or(&[id_middle, id_move_left, id_move_right]) && are_we_climbing(animated_sprite) {
                if animated_sprite.state.animation_name == "climb" {
                    state.climb_timer = animated_sprite.state.timer;
                    animated_sprite.play("climb_idle");
                }
                animated_sprite.state.timer = state.climb_timer;
            } else {
                // let player stand (pixel) perfect on floor, less problems
                transform.position = vec2(transform.position.x, (transform.position.y as i32) as f32);
                animated_sprite.play("idle");
            }
        }

        door::exit(state,&pos);

        //draw_circle(pos,2.0,WHITE,100);
        //draw_circle(check_move_left, 1.0, GREEN, 100);
        //draw_circle(check_move_right, 1.0, GREEN, 100);

        //draw_circle(pos,1.0,WHITE,100);
        //draw_circle(check_ladder_pos, 1.0,PINK,100);
        //draw_circle(check_ladder_pos_down, 1.0,PINK,100);
        //draw_circle(check_ladder_pos_head, 1.0,PINK,100);
        //draw_rect_outline(pos,splat(16.0),2.0,WHITE,0);
        //println!("Player pos {}", transform.position);
    }
}

pub fn is_action_down() -> bool {
    is_key_down(KeyCode::S) || is_key_down(KeyCode::Down)
}

pub fn is_action_up() -> bool {
    is_key_down(KeyCode::W) || is_key_down(KeyCode::Up)
}

pub fn is_action_left() -> bool {
    is_key_down(KeyCode::A) || is_key_down(KeyCode::Left)
}

pub fn is_action_right() -> bool {
    is_key_down(KeyCode::D) || is_key_down(KeyCode::Right)
}

pub fn are_we_climbing(animated_sprite: &AnimatedSprite) -> bool {
    animated_sprite.state.animation_name == "climb_idle" || animated_sprite.state.animation_name == "climb"
}