mod input;

use ::rand::Rng;
use macroquad::prelude::*;
use std::collections::VecDeque;

const GRID_W: i32 = 30;
const GRID_H: i32 = 20;
const CELL_SIZE: f32 = 24.0;
const MOVE_DELAY: f32 = 0.12;

#[macroquad::main("Snake (macroquad)")]
async fn main()
{
    let window_w = GRID_W as f32 * CELL_SIZE;
    let window_h = GRID_H as f32 * CELL_SIZE;

    let mut rng = ::rand::rng();
    let mut snake: VecDeque<(i32, i32)> = VecDeque::new();
    let start_x = GRID_W / 2;
    let start_y = GRID_H / 2;
    snake.push_back((start_x, start_y));
    snake.push_back((start_x - 1, start_y));
    let mut direction = input::get_start_direction();
    let mut move_timer = 0.0;
    let mut food = spawn_food(&snake, &mut rng);
    let mut score: i32 = 0;
    let mut game_over = false;

    loop
    {
        clear_background(Color::from_rgba(30, 30, 30, 255));

        let screen_center_x = screen_width() / 2.0;
        let screen_center_y = screen_height() / 2.0;
        let field_x = screen_center_x - window_w / 2.0;
        let field_y = screen_center_y - window_h / 2.0;

        direction = input::get_direction(direction);

        if game_over && input::is_key_restart_down()
        {
            snake.clear();
            snake.push_back((start_x, start_y));
            snake.push_back((start_x - 1, start_y));
            direction = input::get_start_direction();
            move_timer = 0.0;
            food = spawn_food(&snake, &mut rng);
            score = 0;
            game_over = false;
        }

        if input::is_key_pause_down()
        {
            loop
            {
                next_frame().await;

                if input::is_key_pause_down()
                {
                    break
                }
            }
        }

        if game_over == false
        {
            let delta_time = get_frame_time();

            move_timer += delta_time;

            if move_timer > MOVE_DELAY
            {
                move_timer = 0.0;

                let (head_x, head_y) = snake.front().copied().unwrap();

                let new_head = input::get_head_position(direction, head_x, head_y);

                if new_head.0 < 0 || new_head.0 >= GRID_W || new_head.1 < 0 || new_head.1 >= GRID_H
                {
                    game_over = true;
                }
                else
                {
                    if snake.iter().any(|&seg| seg == new_head)
                    {
                        game_over = true;
                    }
                    else
                    {
                        snake.push_front(new_head);
                    }

                    if new_head == food
                    {
                        score += 1;

                        food = spawn_food(&snake, &mut rng);
                    }
                    else
                    {
                        snake.pop_back();
                    }
                }
            }
        }

        draw_rectangle_lines(field_x, field_y, window_w, window_h, 2.0, Color::from_rgba(200, 200, 200, 255));

        draw_cell(field_x, field_y, food.0, food.1, Color::from_rgba(200, 40, 40, 255));

        let mut i = 0;

        for &(x, y) in snake.iter()
        {
            let color = if i == 0
            {
                Color::from_rgba(120, 220, 120, 255)
            }
            else
            {
                Color::from_rgba(40, 180, 40, 255)
            };

            draw_cell(field_x, field_y, x, y, color);

            i += 1;
        }

        let hud_x = 10.0;
        let hud_y = 10.0;

        draw_text(&format!("Score: {}", score), hud_x, hud_y + 20.0, 28.0, WHITE);

        draw_text("Arrows / WASD - move, P - pause, R - restart (after game over)", hud_x, hud_y + 46.0, 18.0, WHITE);

        if game_over
        {
            let t = "Game Over! Press R to restart";
            let tw = measure_text(t, None, 36, 1.0);
            let x = screen_width() / 2.0 - tw.width / 2.0;
            let y = screen_height() / 2.0;

            draw_text(t, x, y, 36.0, Color::from_rgba(240, 80, 80, 255));
        }

        next_frame().await;
    }
}

fn draw_cell(field_x: f32, field_y: f32, gx: i32, gy: i32, color: Color)
{
    let x = field_x + gx as f32 * CELL_SIZE;
    let y = field_y + gy as f32 * CELL_SIZE;

    draw_rectangle(x + 1.0, y + 1.0, CELL_SIZE - 2.0, CELL_SIZE - 2.0, color);
}

fn spawn_food(snake: &VecDeque<(i32, i32)>, rng: &mut ::rand::rngs::ThreadRng) -> (i32, i32)
{
    loop
    {
        let fx = rng.random_range(0..GRID_W);
        let fy = rng.random_range(0..GRID_H);

        if snake.iter().any(|&(x, y)| x == fx && y == fy) == false
        {
            return (fx, fy);
        }
    }
}