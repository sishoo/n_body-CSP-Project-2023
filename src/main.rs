// as of march 18 2023
use ggez;
use ggez::{Context, GameResult};
use ggez::event;
use ggez::glam::*;
use ggez::graphics::{self, Color, Vertex, InstanceArray};
use rand::Rng;
use std::time::{Duration, Instant};

mod quad_tree;
use quad_tree::{Node, Rect, Point};

const NUM_BODIES: u32 = 1_000;

struct MainState {
    planets: Vec<Planet>,
    quad_tree: Node
}

impl MainState {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut planets = Vec::new();
        let mut quad_tree = Node::new(Rect::new(Point::new(0.0, 0.0),
                                                1000.0, 800.0));
        for _ in 0..NUM_BODIES {
            let planet = Planet {
                mass: rng.gen_range(1.0..1000.0),
                pos: Vec2::new(rng.gen_range(0.0..1000.0),
                               rng.gen_range(0.0..800.0)),
                velocity: Vec2::new(rng.gen_range(0.0..10.0),
                                    rng.gen_range(0.0..10.0))
            };
            planets.push(planet.clone());
            quad_tree.insert(Point::new(planet.pos.x, planet.pos.y),
                             planet.mass);
        }
        MainState {
            planets: planets,
            quad_tree: quad_tree
        }
    }
}

#[derive(Copy, Clone)]
struct Planet {
    mass: f32,  // Kilogram
    pos: Vec2,
    velocity: Vec2
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        for planet in self.planets.iter_mut() {

        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult  {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::from([0.0, 0.0, 0.0, 0.0]) // background
        );

        let planet_mesh = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            3.0,
            0.1,
            Color::WHITE,
        )?;

        /*
                AVERAGE:
                    - 400 bodies, for loop = 333.426 microseconds
                    - 1_000 bodies, for loop = 825.956 microseconds
                    - 10_000 bodies, for loop = 9.767 milliseconds
                    - 20_000 bodies, for loop = 16.01317 milliseconds.
                    - 30_000 bodies, for loop = 21.030 milliseconds
                    - 400 bodies, batch render =

         */
        let now = Instant::now();
        for planet in &self.planets {
            canvas.draw(&planet_mesh, planet.pos); // use batch render instead
        }
        println!("The time to draw: {:?} seconds", now.elapsed());


        canvas.finish(ctx)?;
        Ok(())
    }

}

fn main() -> GameResult {
    let context_builder = ggez::ContextBuilder::new("orbit", "Mac")
        .window_setup(ggez::conf::WindowSetup::default().title("N body
simulator"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(1000.0,
                                                                  800.0));
    let (ctx, event_loop) =  context_builder.build()?;
    let state = MainState::new();
    event::run(ctx, event_loop, state);
    Ok(())
}
