use ggez::glam::Vec2;
use crate::Planet;

const MAX_POINTS: usize = 1;
const APPROXIMATION_DISTANCE_LIMIT: f32 = 0.5;
const G: f32 = 6.67430e-11;
const DELTA_TIME: f32 = 1.0;

/*
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
 */

#[derive(Debug)]
pub struct Rect {
    pub nw: Vec2,
    pub width: f32,
    pub height: f32
}

#[derive(Debug)]
pub struct Node {
    pub contents: Vec<Planet>,
    pub children: Option<Box<[Option<Box<Node>>; 4]>>,
    pub bounds: Rect,
    pub total_mass: f32,
    pub center_of_mass: Vec2
}

impl Node {
    pub fn update_pos(&mut self, planet: &mut Planet) {
        let taxicab_distance = planet.pos - self.center_of_mass;
        let distance = (taxicab_distance.x * taxicab_distance.x + taxicab_distance.y * taxicab_distance.y).sqrt();
        if self.children.is_none() && !self.bounds.bounds_contains(&planet.pos) && self.contents.len() > 0 {
            let taxicab_distance = planet.pos - self.contents[0].pos;
            let distance = (taxicab_distance.x * taxicab_distance.x + taxicab_distance.y * taxicab_distance.y).sqrt();
            let net_force = G * planet.mass * self.contents[0].mass / (distance * distance);
            let force_components = Vec2::new(net_force * (taxicab_distance.x / distance), net_force * (taxicab_distance.y / distance));
            let acceleration_components = force_components / planet.mass;
            planet.velocity += acceleration_components * DELTA_TIME;
            if (distance - planet.velocity).x <= 3.0 || (distance - planet.velocity).y <= 3.0 {
                let new_velocity = (planet.mass * planet.velocity + self.contents[0].mass * self.contents[0].velocity) / (planet.mass + self.contents[0].mass);
                let new_pos = planet.pos + new_velocity * DELTA_TIME;
                self.remove(planet);
                planet.pos = new_pos;
            } else {
                self.remove(planet);
                planet.pos -= planet.velocity * DELTA_TIME;
            }
            self.insert(*planet);
        } else if self.bounds.width / distance < APPROXIMATION_DISTANCE_LIMIT {
            let taxicab_distance = planet.pos - self.center_of_mass;
            let distance = (taxicab_distance.x * taxicab_distance.x + taxicab_distance.y * taxicab_distance.y).sqrt();
            let net_force = G * planet.mass * self.total_mass / (distance * distance);
            let force_components = Vec2::new(net_force * (taxicab_distance.x / distance), net_force * (taxicab_distance.y / distance));
            let acceleration_components = force_components / planet.mass;
            self.remove(planet);
            planet.velocity += acceleration_components;
            planet.pos -= planet.velocity;
            self.insert(*planet);
        } else {
            if self.children.is_some() {
                for child in self.children.as_mut().unwrap().iter_mut() {
                    child.as_mut().unwrap().update_pos(planet);
                }
            }
        }
    }

    pub fn new(bounds: Rect) -> Self {
        Node {
            contents: Vec::new(),
            children: None,
            bounds: bounds,
            total_mass: 0.0,
            center_of_mass: Vec2::new(0.0, 0.0)
        }
    }

    pub fn insert(&mut self, body: Planet) {
        self.total_mass += body.mass;
        self.center_of_mass = ((body.mass * body.pos) + (self.center_of_mass * (self.total_mass - body.mass))) / self.total_mass;

        if self.children.is_none() {
            self.contents.push(body);
            if self.contents.len() > MAX_POINTS {
                self.subdivide(body.mass);
            }
        } else {
            for child in self.children.as_mut().unwrap().iter_mut() {
                if child.as_ref().unwrap().bounds.bounds_contains(&body.pos) {
                    child.as_mut().unwrap().insert(body);
                    break;
                }
            }
        }
    }

    fn subdivide(&mut self, mass: f32) {
        let origin = self.bounds.nw;
        let width = self.bounds.width;
        let height = self.bounds.height;
        self.children = Some(
            Box::new([
                Some(
                    Box::new(
                        Node::new(
                            Rect::new(origin, width / 2.0, height / 2.0)
                        )
                    )
                ),
                Some(
                    Box::new(
                        Node::new(
                            Rect::new(
                                Vec2::new(origin.x + width / 2.0, origin.y), width / 2.0, height / 2.0)
                        )
                    )
                ),
                Some(
                    Box::new(
                        Node::new(
                            Rect::new(
                                Vec2::new(origin.x, origin.y + height / 2.0), width / 2.0, height / 2.0)
                        )
                    )
                ),
                Some(
                    Box::new(
                        Node::new(
                            Rect::new(
                                Vec2::new(origin.x + width / 2.0, origin.y + height / 2.0), width / 2.0, height / 2.0)
                        )
                    )
                )
            ]
            )
        );
        for planet in &self.contents {
            for child in self.children.as_mut().unwrap().iter_mut() {
                if child.as_ref().unwrap().bounds.bounds_contains(&planet.pos) {
                    child.as_mut().unwrap().insert(*planet);
                    break;
                }
            }
        }
        self.contents.clear();
    }

    fn remove(&mut self, planet: &Planet) {
        // if self.bounds.bounds_contains(&planet.pos) {
        //     let index_of_value = match self.contents.iter().position(|&element| element.pos == planet.pos) {
        //         Some(i) => i,
        //         None => return,
        //     };
        //     self.contents.remove(index_of_value);
        //     self.total_mass -= planet.mass;
        //     self.center_of_mass -= planet.mass * planet.pos;
        if let Some(pos) = self.contents.iter().position(|x| x.pos == planet.pos) {
            self.contents.remove(pos);
            self.total_mass -= planet.mass;
            self.center_of_mass -= planet.mass * planet.pos;
        } else if self.children.is_some() {
            for child in self.children.as_mut().unwrap().iter_mut() {
                if child.as_ref().unwrap().bounds.bounds_contains(&planet.pos) {
                    child.as_mut().unwrap().remove(planet);
                    break;
                }
            }

        }
    }
}



impl Rect {
    pub fn new(nw: Vec2, width: f32, height: f32) -> Self {
        Rect {
            nw: nw,
            width: width,
            height: height
        }
    }

    pub fn bounds_contains(&self, point: &Vec2) -> bool {
        self.nw.x <= point.x && point.x <= self.nw.x + self.width &&
            self.nw.y <= point.y && point.y <= self.nw.y + self.height
    }
}
