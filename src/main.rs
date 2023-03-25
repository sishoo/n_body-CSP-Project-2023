use ggez::{Context, GameResult};
use ggez::event;
use ggez::glam::*;
use ggez::graphics::{self, Color, Vertex, InstanceArray};
use rand::Rng;
use std::time::{Duration, Instant};
use std::mem;

mod quad_tree;
use quad_tree::{Node, Rect};

const NUM_BODIES: usize = 10000;
const SINGLE_BODY_DISTANCE_LIMIT: f32 = 0.5;
const G: f32 = 6.67430e11;

struct MainState {
    planets: Vec<Planet>,
    quad_tree: Node
}

#[derive(Copy, Clone, Debug)]
pub struct Planet {
    pub mass: f32,
    pub pos: Vec2,
}

impl MainState {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut planets = Vec::new();
        let mut quad_tree = Node::new(Rect::new(Vec2::new(0.0, 0.0), 1000.0, 800.0));
        for _ in 0..NUM_BODIES {
            let planet = Planet {
                mass: rng.gen_range(1.0..1000.0),
                pos: Vec2::new(rng.gen_range(0.0..1000.0), rng.gen_range(0.0..800.0)),
            };
            planets.push(planet.clone());
            quad_tree.insert(planet);
        }
        println!("{:?}", quad_tree);
        MainState {
            planets: planets,
            quad_tree: quad_tree
        }
    }

    fn update_pos(&mut self, planet: Planet) -> Vec2 {
        let mut x_net_force = 0.0;
        let mut y_net_force = 0.0;

        // checks if is node is an external node, meaning if it is a point or not
        if self.quad_tree.children.is_none() && !self.quad_tree.bounds.bounds_contains(&planet.pos) {
            let x_difference = self.quad_tree.contents[0].pos.x - planet.pos.x;
            let y_difference = self.quad_tree.contents[0].pos.y - planet.pos.y;
            let total_distance = x_difference * x_difference + y_difference * y_difference;
            let net_force = (G * planet.mass * self.quad_tree.contents[0].mass) / total_distance;
            x_net_force += net_force * (x_difference / total_distance.sqrt());
            y_net_force += net_force * (y_difference / total_distance.sqrt());
        }

        Vec2::new(10.0, 10.0)
    }
}


impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let now = Instant::now();
        let mut buffer = Vec::with_capacity(NUM_BODIES);
        std::mem::swap(&mut buffer, &mut self.planets);
        for mut planet in buffer {
            planet.pos += self.update_pos(planet);
            self.planets.push(planet);
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


