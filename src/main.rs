extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use rand::Rng;

use std::collections::LinkedList;
use std::iter::FromIterator;
use std::vec;

#[derive(Clone, PartialEq)]
enum Direction
{
    Right, Left, Up, Down
}

struct Game 
{
    score: i32,
    size: u32,
    gl: GlGraphics,
    snake: Snake,
    food: Food,
}

impl Game 
{
    fn render(&mut self, arg: &RenderArgs) 
    {
        let black: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        self.gl.draw(arg.viewport(), |_c, gl| 
        {
            graphics::clear(black, gl);
        });

        self.snake.render(&mut self.gl, arg);
        self.food.render(&mut self.gl, arg);
    }

    fn update(&mut self)
    {
        self.snake.update();

        if self.snake.is_collide(self.food.pos_x, self.food.pos_y)
        {
            self.score += 1;

            let mut new_fragment = LinkedList::new();
            new_fragment.push_back((0, -1 as i32));

            self.snake.body.append(&mut new_fragment);

            loop
            {
                self.food.pos_x = self.food.teleport(self.size as i32);
                self.food.pos_y = self.food.teleport(self.size as i32);

                if !self.snake.is_collide(self.food.pos_x, self.food.pos_y)
                {
                    break;
                }
            }
        }
    }

    fn pressed(&mut self, btn: &Button)
    {
        let last_direction = self.snake.dir.clone();

        self.snake.dir = match btn 
        {
            &Button::Keyboard(Key::W) //Go up
                if last_direction != Direction::Down => Direction::Up,  
            &Button::Keyboard(Key::S) //Go down
                if last_direction != Direction::Up => Direction::Down,  
            &Button::Keyboard(Key::A) //Go left
                if last_direction != Direction::Right => Direction::Left,  
            &Button::Keyboard(Key::D) //Go right
                if last_direction != Direction::Left => Direction::Right,  
            _ => last_direction
        }
    }
}

struct Snake
{
    body: LinkedList<(i32, i32)>,
    head_position_x: i32,
    head_position_y: i32, 
    dir: Direction,
}

impl Snake
{
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs)
    {
        let green: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self.body
            .iter()
            .map(|&(x, y)|
            {
                graphics::rectangle::square(
                    (x * 20) as f64,
                    (y * 20) as f64,
                    20_f64
                )
            }).collect();

        gl.draw(args.viewport(), |c, gl|
        {
            let transform = c.transform;
            squares.into_iter()
                .for_each(|square|  graphics::rectangle(green, square, transform, gl));
        });
    }

    fn update(&mut self)
    {
        let mut new_head = (*self.body.front().expect("Snake has no body")).clone();

        match self.dir
        {
            Direction::Left => 
            {
                new_head.0 -= 1;
                self.head_position_x -= 1;
            },
            Direction::Right => 
            {
                new_head.0 += 1;
                self.head_position_x += 1;
            },
            Direction::Up => 
            {
                new_head.1 -= 1;
                self.head_position_y -= 1; 
            },
            Direction::Down =>
            {
                new_head.1 += 1;
                self.head_position_y += 1;
            }
        }

        if self.is_collide(new_head.0, new_head.1)
        {
            return;
        }

        self.body.push_front(new_head);   
        self.body.pop_back().unwrap();
    }

    fn is_collide(&self, x: i32, y: i32) -> bool
    {
        self.body.iter().any(|p| x == p.0 && y == p.1)
    }
}

struct Food
{
    pos_x: i32,
    pos_y: i32,
}

impl Food
{
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs)
    {
        let blue: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let square = graphics::rectangle::square(
            (self.pos_x * 20) as f64,
            (self.pos_y * 20) as f64,
            20_f64
        );

        gl.draw(args.viewport(), |c, gl|{
            let transform = c.transform;

            graphics::rectangle(blue, square, transform, gl);
        });
    }

    fn teleport(&self, size: i32) -> i32
    {
        return rand::thread_rng().gen_range(0, size / 20);
    }
}

fn main() 
{
    let opengl = OpenGL::V4_5;
    let score: i32 = 0;
    let size: u32 = 200;

    let mut window: GlutinWindow = WindowSettings::new("Snake Game", [size, size])
    .exit_on_esc(false)
    .resizable(false)
    .vsync(true)
    .build()
    .unwrap();

    let mut game = Game 
    {
        score: score,
        size: size,
        gl: GlGraphics::new(opengl),
        snake: Snake 
        {
            body: LinkedList::from_iter((vec![(0,0), (0,1)]).into_iter()),
            head_position_x: 0,
            head_position_y: 0,
            dir: Direction::Right
        },
        food: Food { pos_x: (0), pos_y: (0) }
    };

    let mut events = Events::new(EventSettings::new().ups(4));
    while let Some(e) = events.next(&mut window) 
    { 
        if let Some(args) = e.render_args() 
        {
            game.render(&args);
        }

        if let Some(_u) = e.update_args()
        {
            game.update();
        }

        if let Some(k) = e.button_args()
        {
            if k.state == ButtonState::Press
            {
                game.pressed(&k.button);
            }
        }
    }
    println!("Congratulations, your score was: {}", game.score);
}
