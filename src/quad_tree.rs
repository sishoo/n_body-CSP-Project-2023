const MAX_POINTS: usize = 4;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    x: f32,
    y: f32
}

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    nw: Point,
    width: f32,
    height: f32
}

#[derive(Debug)]
pub struct Node {
    contents: Vec<Point>,
    children: Option<Box<[Option<Box<Node>>; 4]>>,
    bounds: Rect,
    total_mass: f32
}

impl Node {
    pub fn new(bounds: Rect) -> Self {
        Node {
            contents: Vec::new(),
            children: None,
            bounds: bounds,
            total_mass: 0.0
        }
    }

    pub fn insert(&mut self, point: Point, mass: f32) {
        self.total_mass += mass;

        if self.children.is_none() {    // if there is children
            self.contents.push(point);  // it adds the point
            if self.contents.len() > MAX_POINTS {   // if over the limit
                self.subdivide(mass);   // it subdivides
            }
        } else {    // if it does have a point in it
            for child in self.children.as_mut().unwrap().iter_mut() {
// for each child
                if
                child.as_ref().unwrap().bounds.bounds_contains(&point) { // if the
                    child contains the point
                    child.as_mut().unwrap().insert(point, mass); //
                    push it to the childs contents
                    break; // get the f outa dodge
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
                                Point::new(origin.x + width / 2.0,
                                           origin.y / 2.0), width / 2.0, height / 2.0)
                        )
                    )
                ),
                Some(
                    Box::new(
                        Node::new(
                            Rect::new(Point::new(origin.x, origin.y +
                                height / 2.0), width / 2.0, height / 2.0)
                        )
                    )
                ),
                Some(
                    Box::new(
                        Node::new(
                            Rect::new(
                                Point::new(origin.x + width / 2.0,
                                           origin.y + height / 2.0), width / 2.0, height / 2.0)
                        )
                    )
                )
            ]
            )
        );

        for point in &self.contents {
            for child in self.children.as_mut().unwrap().iter_mut() {
                if child.as_ref().unwrap().bounds.bounds_contains(&point) {
                    child.as_mut().unwrap().insert(*point, mass);
                    break;
                }
            }
        }
    }
}



impl Rect {
    pub fn new(nw: Point, width: f32, height: f32) -> Self {
        Rect {
            nw: nw,
            width: width,
            height: height
        }
    }

    fn bounds_contains(&self, point: &Point) -> bool {
        self.nw.x < point.x && point.x < self.nw.x + self.width &&
            self.nw.y < point.y && point.y < self.nw.y + self.height
    }
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point {
            x: x,
            y: y
        }
    }
}