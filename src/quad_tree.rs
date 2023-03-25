use ggez::glam::Vec2;
use crate::Planet;

const MAX_POINTS: usize = 1;
const APPROXIMATION_DISTANCE_LIMIT: f32 = 0.5;
const G: f32 = 6.67430e-11;

/*
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
 */

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    nw: Vec2,
    pub width: f32,
    height: f32
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

        if self.children.is_none() && !self.bounds.bounds_contains(&planet.pos) {
            let taxicab_distance = planet.pos - self.contents[0].pos;
            let distance = (taxicab_distance.x * taxicab_distance.x + taxicab_distance.y * taxicab_distance.y).sqrt();
            let net_force = G * planet.mass * self.contents[0].mass / (distance * distance);
            let force_components = Vec2::new(net_force * (taxicab_distance.x / distance), net_force * (taxicab_distance.y / distance));
            let acceleration_components = force_components / planet.mass;
            planet.velocity += acceleration_components;
            planet.pos += planet.velocity;
        } else if self.bounds.width / distance < APPROXIMATION_DISTANCE_LIMIT {
            let taxicab_distance = planet.pos - self.center_of_mass;
            let distance = (taxicab_distance.x * taxicab_distance.x + taxicab_distance.y * taxicab_distance.y).sqrt();
            let net_force = G * planet.mass * self.total_mass / (distance * distance);
            let force_components = Vec2::new(net_force * (taxicab_distance.x / distance), net_force * (taxicab_distance.y / distance));
            let acceleration_components = force_components / planet.mass;
            planet.velocity += acceleration_components;
            planet.pos += planet.velocity;
        } else {
            for child in self.children.as_mut().unwrap().iter_mut() {
                child.as_mut().unwrap().update_pos(planet);
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
        self.center_of_mass = body.mass * body.pos + self.center_of_mass / self.total_mass;

        /*
        if there is no children
        it adds the point
        if over the limit
        it subdivides
        if it does have a point in it
        for each child it sees if the child contains the point
        pushes point to child
        breaks out because we already found the node that contains it
         */

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
                Some( //q1
                      Box::new(
                          Node::new(
                              Rect::new(origin, width / 2.0, height / 2.0)
                          )
                      )
                ),
                Some(  //q2
                       Box::new(
                           Node::new(
                               Rect::new(
                                   Vec2::new(origin.x + width / 2.0, origin.y), width / 2.0, height / 2.0)
                           )
                       )
                ),
                Some(  //q3
                       Box::new(
                           Node::new(
                               Rect::new(
                                   Vec2::new(origin.x, origin.y + height / 2.0), width / 2.0, height / 2.0)
                           )
                       )
                ),
                Some(  //q4
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

        /*
        for every point in the contents, we go through each child
        and see if the point fits in the childs bounds
        if it does, we push the point to the childs contents
         */
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
        self.nw.x < point.x && point.x < self.nw.x + self.width &&
            self.nw.y < point.y && point.y < self.nw.y + self.height
    }
}



