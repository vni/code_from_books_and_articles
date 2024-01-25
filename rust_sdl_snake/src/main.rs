use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use std::time::Duration;

const GRID_X_SIZE: u32 = 40;
const GRID_Y_SIZE: u32 = 30;
const DOT_SIZE_IN_PXS: u32 = 20;

#[derive(PartialEq)]
pub enum GameState {
    Playing,
    Paused,
    Lost,
}
// yes, currently u can't win. :-))
pub enum PlayerDirection {
    Up,
    Down,
    Right,
    Left,
}
#[derive(Copy, Clone, PartialEq)]
pub struct Point(pub i32, pub i32);

impl std::ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

pub struct GameContext {
    pub snake_body: Vec<Point>,
    pub snake_size: usize,
    pub direction: PlayerDirection,
    pub apple: Point,
    pub state: GameState,
}

impl GameContext {
    pub fn new() -> GameContext {
        GameContext {
            snake_body: vec![Point(3, 1), Point(2, 1), Point(1, 1)],
            snake_size: 3,
            direction: PlayerDirection::Right,
            apple: Point(3, 3),
            state: GameState::Paused,
        }
    }

    pub fn next_tick(&mut self) {
        if GameState::Paused == self.state {
            return;
        }

        if GameState::Lost == self.state {
            return;
        }

        let head_position = self.snake_body.first().unwrap();
        let next_head_position = match self.direction {
            PlayerDirection::Up => *head_position + Point(0, -1),
            PlayerDirection::Down => *head_position + Point(0, 1),
            PlayerDirection::Right => *head_position + Point(1, 0),
            PlayerDirection::Left => *head_position + Point(-1, 0),
        };

        if self.snake_body.contains(&next_head_position) {
            self.state = GameState::Lost;
            println!("You crashed yourself and lose!");
            println!("Your score is {}", self.snake_size);
            return;
        }

        if next_head_position.0 >= GRID_X_SIZE as i32
            || next_head_position.0 < 0
            || next_head_position.1 >= GRID_Y_SIZE as i32
            || next_head_position.1 < 0
        {
            self.state = GameState::Lost;
            println!("You crashed the wall and lose!");
            println!("Your score is {}", self.snake_size);
            return;
        }

        if next_head_position == self.apple {
            self.snake_size += 1;
            self.apple = self.generate_new_apple_position();
        } else {
            self.snake_body.pop();
        }

        self.snake_body.reverse();
        self.snake_body.push(next_head_position);
        self.snake_body.reverse();
    }

    pub fn move_up(&mut self) {
        self.direction = PlayerDirection::Up;
    }

    pub fn move_down(&mut self) {
        self.direction = PlayerDirection::Down;
    }

    pub fn move_right(&mut self) {
        self.direction = PlayerDirection::Right;
    }

    pub fn move_left(&mut self) {
        self.direction = PlayerDirection::Left;
    }

    pub fn toggle_pause(&mut self) {
        self.state = match self.state {
            GameState::Playing => GameState::Paused,
            GameState::Paused => GameState::Playing,
            _ => unreachable!(),
        }
    }

    fn generate_new_apple_position(&self) -> Point {
        loop {
            let x = rand::random::<u32>() % GRID_X_SIZE;
            let y = rand::random::<u32>() % GRID_Y_SIZE;

            let a = Point(x as i32, y as i32);
            if !self.snake_body.contains(&a) {
                return a;
            }
        }
    }
}

impl Default for GameContext {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Renderer {
    canvas: WindowCanvas,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Renderer { canvas })
    }

    pub fn draw(&mut self, context: &GameContext) -> Result<(), String> {
        /*
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
        */

        self.draw_background(context); // ???
        self.draw_snake(context)?;
        self.draw_apple(context)?;
        self.canvas.present();

        Ok(())
    }

    fn draw_dot(&mut self, point: &Point) -> Result<(), String> {
        let Point(x, y) = point;
        self.canvas.fill_rect(Rect::new(
            x * DOT_SIZE_IN_PXS as i32,
            y * DOT_SIZE_IN_PXS as i32,
            DOT_SIZE_IN_PXS,
            DOT_SIZE_IN_PXS,
        ))?;

        Ok(())
    }

    fn draw_background(&mut self, context: &GameContext) {
        let color = match context.state {
            GameState::Playing => Color::RGB(0, 0, 0),
            GameState::Paused => Color::RGB(30, 30, 30),
            GameState::Lost => Color::RGB(50, 0, 0),
        };

        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    fn draw_snake(&mut self, context: &GameContext) -> Result<(), String> {
        self.canvas.set_draw_color(Color::GREEN);
        for point in &context.snake_body {
            self.draw_dot(point)?;
        }

        Ok(())
    }

    fn draw_apple(&mut self, context: &GameContext) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RED);
        self.draw_dot(&context.apple)?;
        Ok(())
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "Simple Stupid and Lonely Snake Game",
            GRID_X_SIZE * DOT_SIZE_IN_PXS,
            GRID_Y_SIZE * DOT_SIZE_IN_PXS,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut renderer = Renderer::new(window)?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut context = GameContext::new();
    let mut frame_counter = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::W | Keycode::Up => context.move_up(),
                    Keycode::A | Keycode::Left => context.move_left(),
                    Keycode::S | Keycode::Down => context.move_down(),
                    Keycode::D | Keycode::Right => context.move_right(),
                    Keycode::Space => context.toggle_pause(),
                    _ => {}
                },
                _ => {}
            }
        }

        if context.state == GameState::Lost {
            break;
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000u32 / 30));
        frame_counter += 1;
        if frame_counter % 1000 == 0 {
            context.next_tick();
            frame_counter = 0;
        }
        renderer.draw(&context)?;
    }

    Ok(())
}
