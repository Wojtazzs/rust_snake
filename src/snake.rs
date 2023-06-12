use piston_window::{Context, G2d};
use piston_window::types::Color;
use crate::draw;

const SNAKE_COLOR: Color = [0.10, 0.10, 0.750, 1.00];

#[derive(Clone, Debug)]
pub enum Part{
    Head,
    Body,
}

#[derive(Clone, Debug)]
pub struct SnakePart{
    pub bodypart: Part,
    pub next: Option<Box<SnakePart>>,
    pub direction: MoveDirection,
    position: Block,
}

impl SnakePart{
    pub fn init() -> SnakePart{
        let head = SnakePart{
            bodypart: Part::Head,
            next: None,
            direction: MoveDirection::Right,
            position: Block{x: 11, y: 11},
        };
        return head;
    }

    // Concatenates a new part to the end of the snake
    fn concat_new(&mut self, new_part: SnakePart){
        let mut current = self;
        while current.next.is_some(){
            current = current.next.as_mut().unwrap();
        }

        current.next = Some(Box::new(new_part));
    }

    // Public function that creates and adds a new part to the snake
    pub fn add_part(&mut self, pos_x: i32, pos_y: i32){
        let new_part = SnakePart{
            bodypart: Part::Body,
            next: None,
            direction: self.direction.clone(),
            position: Block{x: pos_x, y: pos_y},
        };

        self.concat_new(new_part);
    }

    pub fn draw(&self, con: &Context, g: &mut G2d){
        draw::draw_block(SNAKE_COLOR, self.position.x, self.position.y, con, g);

        if self.next.is_some(){
            let next = self.next.as_ref().unwrap();
            next.draw(con, g);
        }
    }

    pub fn head_position(&self) -> Block{
        return self.position.clone();
    }

    pub fn move_forward(&mut self, dir: Option<MoveDirection>){
        match dir{
            Some(d) => self.direction = d,
            None => (),
        }
        let mut current = self;
        let mut prev_pos = current.head_position();

        current.position = match current.direction{
            MoveDirection::Up => Block{x: current.position.x, y: current.position.y - 1},
            MoveDirection::Down => Block{x: current.position.x, y: current.position.y + 1},
            MoveDirection::Left => Block{x: current.position.x - 1, y: current.position.y},
            MoveDirection::Right => Block{x: current.position.x + 1, y: current.position.y},
        };

        while current.next.is_some(){
            let mut next_part = current.next.as_mut().unwrap().as_mut();
            let next_pos = next_part.head_position();
            next_part.position = prev_pos;
            prev_pos = next_pos;
            current = next_part;

        }
    }

    pub fn overlap_tail(&self, x: i32, y: i32) -> bool{
        let mut current = self;
        while current.next.is_some(){
            let next = current.next.as_ref().unwrap();
            if next.position.x == x && next.position.y == y{
                return true;
            }
            current = next;
        }
        return false;
    }

    pub fn position(&self) -> (i32, i32){
        return (self.position.clone().x, self.position.clone().y);
    }

    pub fn is_inside(&self, x: i32, y: i32) -> bool{

        let mut current = self;
        while current.next.is_some(){
            if current.position.x == x && current.position.y == y{
                return true;
            }
            current = current.next.as_ref().unwrap();
        }
        return false;

    }
}


#[derive(Clone, Debug, PartialEq, Copy)]
pub enum MoveDirection{
    Up,
    Down,
    Left,
    Right,
}


#[derive(Clone, Debug)]
pub struct Block{
    x: i32,
    y: i32,
}


impl MoveDirection{
    pub fn opposite(&self) -> MoveDirection{
        match *self{
            MoveDirection::Up => MoveDirection::Down,
            MoveDirection::Down => MoveDirection::Up,
            MoveDirection::Left => MoveDirection::Right,
            MoveDirection::Right => MoveDirection::Left,
        }
    }
}

