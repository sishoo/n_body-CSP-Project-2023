use ggez::{Context, GameResult};
use ggez::event;
use ggez::glam::*;
use ggez::graphics::{self, Color, Vertex, InstanceArray};
use rand::Rng;
use std::time::{Duration, Instant};

mod quad_tree;
use quad_tree::{Node, Rect};

const NUM_BODIES: usize = 200;

struct MainState {
    planets: Vec<Planet>,
    quad_tree: Node,
}

#[derive(Copy, Clone, Debug)]
pub struct Planet {
    pub mass: f32,
    pub pos: Vec2,
    pub velocity: Vec2,
}

impl MainState {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut planets = Vec::new();
        let mut quad_tree = Node::new(Rect::new(Vec2::new(0.0, 0.0), 1000.0, 800.0));
        for _ in 0..NUM_BODIES {
            let planet = Planet {
                //mass: rng.gen_range(1.0..1_000_000_000.0),
                mass: 10000000.0,
                pos: Vec2::new(rng.gen_range(0.0..1_000.0), rng.gen_range(0.0..800.0)),
                velocity: Vec2::new(0.0, 0.0)
            };
            planets.push(planet.clone());
            quad_tree.insert(planet);
        }
        MainState {
            planets: planets,
            quad_tree: quad_tree,
        }
    }
}


impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let now = Instant::now();
        for planet in &mut self.planets {
            self.quad_tree.update_pos(planet);
        }
        println!("{:?}", now.elapsed());
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult  {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::from([0.0, 0.0, 0.0, 0.0])
        );
        let planet_mesh = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            3.0,
            0.1,
            Color::WHITE,
        )?;
        for planet in &self.planets {
            canvas.draw(&planet_mesh, planet.pos);
        }
        canvas.finish(ctx)?;
        Ok(())
    }
}




fn main() -> GameResult {
    let context_builder = ggez::ContextBuilder::new("orbit", "Mac")
        .window_setup(ggez::conf::WindowSetup::default().title("N body simulator"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(1000.0, 800.0));
    let (ctx, event_loop) =  context_builder.build()?;
    let state = MainState::new();
    event::run(ctx, event_loop, state);
    Ok(())
}
