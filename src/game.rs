use piston_window::*;
use piston_window::types::Color;

use rand::{thread_rng, Rng};

use crate::snake::{SnakePart, MoveDirection};
use crate::draw::{draw_block, draw_rectangle};

const APPLE_COLOR: Color = [0.80, 0.00, 0.00, 1.00];
const BORDER_COLOR: Color = [0.10, 0.10, 0.10, 1.00];
const BACKGROUND_COLOR: Color = [0.0, 0.0, 0.0, 1.00];

const MOVING_PERIOD: f64 = 0.2;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    snake: SnakePart,
    food_exists: bool,
    food_position: (i32, i32),
    width: i32,
    height: i32,
    game_over: bool,
    waiting_time: f64,
    ate: bool,
}

impl Game{
    pub fn new(width: i32, height: i32) -> Game{
        Game{
            snake: SnakePart::init(),
            food_exists: false,
            food_position: (0, 0),
            width,
            height,
            game_over: false,
            waiting_time: 0.0,
            ate: false,
        }
    }

    pub fn spawn_food(&mut self){
        let mut rng = thread_rng();
        let mut food_x = rng.gen_range(1..self.width-1);
        let mut food_y = rng.gen_range(1..self.height-1);
        
        while self.snake.is_inside(food_x, food_y){
            food_x = rng.gen_range(1..self.width-1);
            food_y = rng.gen_range(1..self.height-1);
        }

        self.food_position = (food_x, food_y);
        self.food_exists = true;
    }

    pub fn user_input(&mut self, key: Key) {
        let dir: Option<MoveDirection> = match key{
            Key::Up => Some(MoveDirection::Up),
            Key::W => Some(MoveDirection::Up),
            Key::Down => Some(MoveDirection::Down),
            Key::S => Some(MoveDirection::Down),
            Key::Left => Some(MoveDirection::Left),
            Key::A => Some(MoveDirection::Left),
            Key::Right => Some(MoveDirection::Right),
            Key::D => Some(MoveDirection::Right),
            _ => None,
        };

        if dir.is_none(){
            return;
        }
        
        if dir.unwrap() == self.snake.direction.opposite(){
            return;
        }
        
        self.snake.direction = dir.unwrap();
    }

    pub fn draw(&self, con: &Context, g: &mut G2d){
        draw_rectangle(BACKGROUND_COLOR, 0, 0, self.width, self.height, con, g);

        self.snake.draw(con, g);

        if self.food_exists{
            draw_block(APPLE_COLOR, self.food_position.0, self.food_position.1, con, g);
        }
        draw_rectangle(BORDER_COLOR, 0,             0,              self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0,             self.height-1,  self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0,             0,       1,                  self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width-1,  0,       1,                  self.height, con, g);
        
        if self.game_over{
            draw_rectangle([0.0, 0.0, 0.0, 1.0], 0, 0, self.width, self.height, con, g);
        }
    }

    pub fn update(&mut self, delta_time: f64, dir: Option<MoveDirection>){
        self.waiting_time += delta_time;
        if self.game_over{
            self.waiting_time += 0.01;
            if self.waiting_time > RESTART_TIME{
                self.restart();
            }
        }else{
            if self.waiting_time > MOVING_PERIOD{
                self.update_snake(dir);
                self.waiting_time = 0.0;
            }

            if !self.food_exists{
                self.spawn_food();
            }
        }
    }

    pub fn restart(&mut self){
        self.snake = SnakePart::init();
        self.food_exists = false;
        self.game_over = false;
        self.waiting_time = 0.0;
        self.food_position = (0, 0);
        self.spawn_food();
        self.width;
        self.height;
    }

    pub fn update_snake(&mut self, dir: Option<MoveDirection>){
        let (head_x, head_y) = self.snake.position();

        self.snake.move_forward(dir);

        if self.ate{
            self.snake.add_part(head_x, head_y);
            self.ate = false;
        }

        let (head_x, head_y) = self.snake.position();

        if head_x < 1 || head_x > self.width-2 || head_y < 1 || head_y > self.height-2{
            self.game_over = true;
        }

        if head_x == self.food_position.0 && head_y == self.food_position.1{
            self.ate = true;
            self.food_exists = false;
        }

        if self.snake.overlap_tail(head_x, head_y){
            self.game_over = true;
        }
    }
}
