use serde::{Deserialize, Serialize};
use rand::Rng;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GameState {
    pub snake: Vec<Position>,
    pub food: Position,
    pub direction: Direction,
    pub score: i32,
    pub game_over: bool,
    pub width: i32,
    pub height: i32,
    pub cell_size: i32,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl GameState {
    pub fn new(width: i32, height: i32, cell_size: i32) -> Self {
        let mut game = GameState {
            snake: vec![Position { x: 5, y: 5 }],
            food: Position { x: 10, y: 10 },
            direction: Direction::Right,
            score: 0,
            game_over: false,
            width,
            height,
            cell_size,
        };
        game.generate_food();
        game
    }

    pub fn update(&mut self) {
        if self.game_over {
            return;
        }

        let head = self.snake[0];
        let new_head = match self.direction {
            Direction::Up => Position { x: head.x, y: head.y - 1 },
            Direction::Down => Position { x: head.x, y: head.y + 1 },
            Direction::Left => Position { x: head.x - 1, y: head.y },
            Direction::Right => Position { x: head.x + 1, y: head.y },
        };

        // 检查碰撞
        if self.check_collision(&new_head) {
            self.game_over = true;
            return;
        }

        self.snake.insert(0, new_head);

        // 检查是否吃到食物
        if new_head.x == self.food.x && new_head.y == self.food.y {
            self.score += 10;
            self.generate_food();
        } else {
            self.snake.pop();
        }
    }

    fn check_collision(&self, pos: &Position) -> bool {
        // 检查是否撞墙
        if pos.x < 0 || pos.x >= self.width / self.cell_size ||
           pos.y < 0 || pos.y >= self.height / self.cell_size {
            return true;
        }

        // 检查是否撞到自己
        self.snake.iter().any(|p| p.x == pos.x && p.y == pos.y)
    }

    fn generate_food(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            let new_food = Position {
                x: rng.gen_range(0..(self.width / self.cell_size)),
                y: rng.gen_range(0..(self.height / self.cell_size)),
            };
            if !self.snake.iter().any(|p| p.x == new_food.x && p.y == new_food.y) {
                self.food = new_food;
                break;
            }
        }
    }

    pub fn change_direction(&mut self, new_direction: Direction) {
        let invalid_change = match (self.direction, new_direction) {
            (Direction::Up, Direction::Down) => true,
            (Direction::Down, Direction::Up) => true,
            (Direction::Left, Direction::Right) => true,
            (Direction::Right, Direction::Left) => true,
            _ => false,
        };

        if !invalid_change {
            self.direction = new_direction;
        }
    }
}