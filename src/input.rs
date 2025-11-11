use macroquad::prelude::*;

pub fn is_key_pressed_up() -> bool
{
    return is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W);
}

pub fn is_key_pressed_down() -> bool
{
    return is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S);
}

pub fn is_key_pressed_left() -> bool
{
    return is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A);
}

pub fn is_key_pressed_right() -> bool
{
    return is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D);
}

pub fn is_key_pressed_pause() -> bool
{
    return is_key_pressed(KeyCode::P);
}

pub fn is_key_pressed_restart() -> bool
{
    return is_key_pressed(KeyCode::R);
}