use raylib::prelude::*;
use rand::Rng;
use std::time::Instant;

const WIDTH: i32 = 450;
const HEIGHT: i32 = 450;
const CELL_SIZE: i32 = 25;

#[derive(Clone, Copy)]
struct Vector2 {
    x: i32,
    y: i32,
}

struct Grid {
    rows: i32,
    cols: i32,
}

impl Grid {
    fn new(rows: i32, cols: i32) -> Grid {
        Grid { rows, cols }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        for x in (0..=WIDTH).step_by(CELL_SIZE as usize) {
            d.draw_line(x, 0, x, HEIGHT, Color::LIGHTGRAY);
        }
        for y in (0..=HEIGHT).step_by(CELL_SIZE as usize) {
            d.draw_line(0, y, WIDTH, y, Color::LIGHTGRAY);
        }
    }
}

struct Food {
    position: Vector2,
    color: Color,
}

impl Food {
    fn new(x: i32, y: i32, color: Color) -> Food {
        Food {
            position: Vector2 { x, y },
            color,
        }
    }

    fn get_position(&self) -> Vector2 {
        Vector2 {
            x: self.position.x * CELL_SIZE,
            y: self.position.y * CELL_SIZE,
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(
            self.position.x * CELL_SIZE,
            self.position.y * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE,
            self.color,
        );
    }
}

struct Snake {
    body: Vec<Vector2>,
    color: Color,
    grow_color: Color,
}

impl Snake {
    fn new(x: i32, y: i32, color: Color, grow_color: Color) -> Snake {
        Snake {
            body: vec![Vector2 { x, y }],
            color,
            grow_color,
        }
    }

    fn grow(&mut self) {
        let last_segment = *self.body.last().unwrap();
        self.body.push(Vector2 {
            x: last_segment.x,
            y: last_segment.y,
        });
    }

    fn check_collision(&self) -> bool {
        let head = self.body[0];
        for segment in &self.body[1..] {
            if head.x == segment.x && head.y == segment.y {
                return true;
            }
        }
        false
    }

    fn update(&mut self, move_dir: Vector2) {
        let mut prev_head = self.body[0];
        self.body[0].x += move_dir.x;
        self.body[0].y += move_dir.y;

        for segment in &mut self.body[1..] {
            let current = *segment;
            segment.x = prev_head.x;
            segment.y = prev_head.y;
            prev_head = current;
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        for (i, segment) in self.body.iter().enumerate() {
            let color = if i == 0 { self.color } else { self.grow_color };
            d.draw_rectangle(
                segment.x * CELL_SIZE,
                segment.y * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE,
                color,
            );
        }
    }
}

struct Game {
    grid: Grid,
    snake: Snake,
    food: Food,
    move_dir: Vector2,
    direction: String,
    game_speed: f32,
    last_update: Instant,
}

impl Game {
    fn new() -> Game {
        let grid = Grid::new((WIDTH / CELL_SIZE) as i32, (HEIGHT / CELL_SIZE) as i32);
        let snake = Snake::new(5, 5, Color::BLUE, Color::SKYBLUE);
        let food = Food::new(10, 15, Color::RED);
        Game {
            grid,
            snake,
            food,
            move_dir: Vector2 { x: 0, y: 0 },
            direction: "none".to_string(),
            game_speed: 10.0,
            last_update: Instant::now(),
        }
    }

    fn handle_keydown(&mut self, keycode: KeyboardKey) {
        match keycode {
            KeyboardKey::KEY_UP => {
                if self.direction != "down" {
                    self.move_dir = Vector2 { x: 0, y: -1 };
                    self.direction = "up".to_string();
                }
            }
            KeyboardKey::KEY_DOWN => {
                if self.direction != "up" {
                    self.move_dir = Vector2 { x: 0, y: 1 };
                    self.direction = "down".to_string();
                }
            }
            KeyboardKey::KEY_LEFT => {
                if self.direction != "right" {
                    self.move_dir = Vector2 { x: -1, y: 0 };
                    self.direction = "left".to_string();
                }
            }
            KeyboardKey::KEY_RIGHT => {
                if self.direction != "left" {
                    self.move_dir = Vector2 { x: 1, y: 0 };
                    self.direction = "right".to_string();
                }
            }
            _ => {}
        }
    }

    fn eat_food(&mut self) {
        self.snake.grow();
        self.respawn_food();
    }

    fn respawn_food(&mut self) {
        let max_x = (WIDTH / CELL_SIZE) as i32;
        let max_y = (HEIGHT / CELL_SIZE) as i32;
        self.food.position.x = rand::thread_rng().gen_range(0..max_x);
        self.food.position.y = rand::thread_rng().gen_range(0..max_y);
    }

    fn update(&mut self) {
        if self.last_update.elapsed().as_secs_f32() < 1.0 / self.game_speed {
            return;
        }

        self.snake.update(self.move_dir);

        let snake_head = self.snake.body[0];

        if snake_head.x >= self.grid.cols
            || snake_head.x < 0
            || snake_head.y < 0
            || snake_head.y >= self.grid.rows
            || self.snake.check_collision()
        {
            self.reset_snake();
        }

        let food_position = self.food.get_position();
        if snake_head.x * CELL_SIZE == food_position.x && snake_head.y * CELL_SIZE == food_position.y {
            self.eat_food();
        }

        self.last_update = Instant::now();
    }

    fn reset_snake(&mut self) {
        self.snake = Snake::new(5, 5, Color::BLUE, Color::SKYBLUE);
        self.move_dir = Vector2 { x: 0, y: 0 };
    }

    fn draw_hud(&self, d: &mut RaylibDrawHandle) {
        let snake_head = self.snake.body[0];
        d.draw_text(
            &format!("SNAKE: {},{}", snake_head.x, snake_head.y),
            140,
            25,
            14,
            Color::BLUE,
        );
        d.draw_text(
            &format!("LENGTH: {}", self.snake.body.len()),
            240,
            25,
            14,
            Color::BLUE,
        );
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Snake Game")
        .build();

    rl.set_target_fps(60);

    let mut game = Game::new();

    while !rl.window_should_close() {
        if let Some(key) = rl.get_key_pressed() {
            game.handle_keydown(key);
        }

        game.update();

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        game.grid.draw(&mut d);
        game.snake.draw(&mut d);
        game.food.draw(&mut d);
        game.draw_hud(&mut d);
    }
}
