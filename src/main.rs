use ggez::{Context, GameResult};
use ggez::event;
use ggez::glam::*;
use ggez::graphics::{self, Color, Vertex, InstanceArray};
use rand::Rng;
use std::time::{Duration, Instant};
use std::io::stdin;


mod quad_tree;
use quad_tree::{Node, Rect};

const NUM_BODIES: usize = 100;

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

impl Planet {
    fn new(mass: f32, pos: Vec2, velocity: Vec2) -> Self {
        Planet {
            mass: mass,
            pos: pos,
            velocity: velocity
        }
    }

    fn collides_with(&self, planet1: &Planet, planet2: &Planet) -> bool {
        true
    }
}

impl MainState {
    pub fn new(num_bodies: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut planets: Vec<Planet> = Vec::new();
        let mut quad_tree = Node::new(Rect::new(Vec2::new(0.0, 0.0), 1000.0, 800.0));
        for _ in 0..num_bodies {
            let planet = Planet::new(
                rng.gen_range(1.0..1_000_000_000.0),
                Vec2::new(rng.gen_range(0.0..1_000.0), rng.gen_range(0.0..800.0)),
                Vec2::new(0.0, 0.0),
            );
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
        self.planets.iter_mut().for_each(|planet| self.quad_tree.update_pos(planet));
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

fn get_user_input() -> usize {
    let mut num_bodies = String::new();
    println!("How many bodies do you want to simulate?: ");
    let buffer = stdin().read_line(&mut num_bodies).unwrap();
    if let Ok(num_bodies) = num_bodies.trim().parse::<usize>() {
        return num_bodies;
    }
    num_bodies.clear();
    get_user_input()
}

fn main() -> GameResult {
    let context_builder = ggez::ContextBuilder::new("orbit", "Mac")
        .window_setup(ggez::conf::WindowSetup::default().title("N body simulator"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(1000.0, 800.0));
    let (ctx, event_loop) =  context_builder.build()?;
    // let state = MainState::new(get_user_input());
    let state = MainState::new(NUM_BODIES);
    event::run(ctx, event_loop, state);
    Ok(())
}
