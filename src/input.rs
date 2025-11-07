use macroquad::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction { Up, Down, Left, Right }

fn opposite(a: Direction, b: Direction) -> bool
{
    matches!((a, b),
        (Direction::Up, Direction::Down) |
        (Direction::Down, Direction::Up) |
        (Direction::Left, Direction::Right) |
        (Direction::Right, Direction::Left))
}

pub fn get_direction(direction: Direction) -> Direction
{
    if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W)
    {
        if opposite(direction, Direction::Up) == false
        {
            return Direction::Up;
        }
    }
    else if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S)
    {
        if opposite(direction, Direction::Down) == false
        {
            return Direction::Down;
        }
    }
    else if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A)
    {
        if opposite(direction, Direction::Left) == false
        {
            return Direction::Left;
        }
    }
    else if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D)
    {
        if opposite(direction, Direction::Right) == false
        {
            return Direction::Right;
        }
    }

    return direction;
}

pub fn get_head_position(direction: Direction, head_x: i32, head_y: i32) -> (i32, i32)
{
    let new_head = match direction 
    {
        Direction::Up => (head_x, head_y - 1),
        Direction::Down => (head_x, head_y + 1),
        Direction::Left => (head_x - 1, head_y),
        Direction::Right => (head_x + 1, head_y),
    };
    
    return new_head;
}

pub fn get_start_direction() -> Direction
{
    return Direction::Right;
}

pub fn is_key_pause_down() -> bool
{
    return is_key_pressed(KeyCode::P);
}

pub fn is_key_restart_down() -> bool
{
    return is_key_pressed(KeyCode::R);
}