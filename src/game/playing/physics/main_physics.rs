use std::cell::Cell;

use macroquad::prelude::*;

pub fn physics_handle(
    player: &mut Rect,
    velocity_y: &Cell<f32>,
    on_ground: &mut bool,
    rotation: &mut f32,
    world_offset: &mut f32,
    movement_speed: f32
) {
    player.y += velocity_y.get();
    // *velocity_y += gravity;

    // if is_mouse_button_down(MouseButton::Left) && *on_ground {
    //     *velocity_y -= jump_force;
    //     *on_ground = false;
    // }

    if player.y > screen_height() / 1.15 - 20.0 {
        player.y = screen_height() / 1.15 - 20.0;
        velocity_y.set(0.0);
        *on_ground = true;
        *rotation = 0.0
    } else if player.y < screen_height() / 1.15 - 21.0 {
        *rotation += 0.1
    }

    *world_offset += movement_speed
}