use macroquad::prelude::*;

use crate::input;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    pub fn init_direction() -> Self {
        return Self::Right;
    }

    pub fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    pub fn apply_to(&self, x: i32, y: i32) -> (i32, i32) {
        match self {
            Self::Up => (x, y - 1),
            Self::Down => (x, y + 1),
            Self::Left => (x - 1, y),
            Self::Right => (x + 1, y),
        }
    }

    pub fn get_direction(direction: Self) -> Self
    {
        if input::is_key_pressed_up()
        {
            if direction.opposite() != Self::Up
            {
                return Self::Up;
            }
        }
        else if input::is_key_pressed_down()
        {
            if direction.opposite() != Self::Down
            {
                return Self::Down;
            }
        }
        else if input::is_key_pressed_left()
        {
            if direction.opposite() != Self::Left
            {
                return Self::Left;
            }
        }
        else if input::is_key_pressed_right()
        {
            if direction.opposite() != Self::Right
            {
                return Self::Right;
            }
        }

        return direction;
    }
}