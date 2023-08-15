use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet},
    ops::{Deref, Index},
    rc::Rc,
    time::Instant,
};

use image::{DynamicImage, GenericImage, GenericImageView, Pixel, Rgba};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Node {
    path_cost: i32,
    point: (u32, u32),
    parent: Option<Rc<Node>>,
    distance_from_start: i32,
    distance_to_end: i32,
}

struct MazeGraph {
    maze_img: DynamicImage,
    frontier: BinaryHeap<Node>,
    visited: HashSet<(u32, u32)>,
    start: (u32, u32),
    end: (u32, u32),
    path: Vec<(u32, u32)>,
}

impl MazeGraph {
    pub fn new(file_path: &str) -> Self {
        let img = image::open(file_path).unwrap();
        let start = Self::find_start(&img).unwrap();
        println!("{:?}", start);
        let end = Self::find_end(&img).unwrap();
        println!("{:?}", end);

        MazeGraph {
            maze_img: img,
            frontier: BinaryHeap::new(),
            visited: HashSet::new(),
            start: start,
            end: end,
            path: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        let start_node = Node {
            point: self.start,
            parent: None,
            distance_from_start: 0,
            distance_to_end: (Self::distance(self.start, self.end) * 1000.0) as i32,
            path_cost: (Self::distance(self.start, self.end) * 1000.0) as i32,
        };

        self.frontier.push(start_node);

        while !self.frontier.is_empty() {
            let node = self.get_next_node();
            let point = node.point;
            // let pixel = Rgba::from_channels(255, 100, 0, 255);
            // self.maze_img.put_pixel(point.0, point.1, pixel);
            if node.point == self.end {
                self.retrace_path(node);
                break;
            }
            if self.visited.contains(&point) {
                continue;
            }
            self.visit_node(node);
            self.visited.insert(point);
        }
        for point in self.path.clone() {
            let pixel = Rgba::from_channels(0, 255, 0, 255);
            self.maze_img.put_pixel(point.0, point.1, pixel);
        }

        self.maze_img.save("test.jpg").unwrap();
    }

    fn get_next_node(&mut self) -> Node {
        // let mut current_max = f64::MAX;
        // let mut node_index = 0;
        // for i in 0..self.frontier.len() {
        //     let path_cost = self.frontier[i].distance_from_start + self.frontier[i].distance_to_end;
        //     if path_cost < current_max {
        //         current_max = path_cost;
        //         node_index = i;
        //     }
        // }
        // self.frontier.remove(node_index)

        // let mut current_max = i32::MAX;
        // let mut current_match = (0u32, 0u32);
        // for (key, value) in self.frontier.iter() {
        //     let path_cost = value.path_cost;
        //     if path_cost < current_max {
        //         current_max = path_cost;
        //         current_match = key.clone();
        //     }
        // }
        // self.frontier.remove(&current_match).unwrap()
        self.frontier.pop().unwrap()
    }

    fn visit_node(&mut self, node: Node) {
        let x = node.point.0 as i32;
        let y = node.point.1 as i32;

        for i in -1..=1 {
            for j in -1..=1 {
                let node = node.clone();
                if i == j {
                    continue;
                }
                let x_prime = (x + i) as u32;
                if x_prime >= self.maze_img.dimensions().0 {
                    continue;
                }
                let y_prime = (y + j) as u32;
                if y_prime >= self.maze_img.dimensions().1 {
                    continue;
                }
                let pixel = self.maze_img.get_pixel(x_prime, y_prime);
                if Self::is_wall(&pixel) {
                    continue;
                }
                let point = (x_prime, y_prime);
                let distance_from_start = node.distance_from_start
                    + (((i.pow(2) + j.pow(2)) as f64).sqrt() * 1000.0) as i32;
                let distance_to_end = (Self::distance(point, self.end) * 1000.0) as i32;
                let path_cost = distance_from_start + distance_to_end;
                let new_node = Node {
                    point: point,
                    parent: Some(Rc::new(node)),
                    distance_from_start: distance_from_start,
                    distance_to_end: distance_to_end,
                    path_cost: (distance_from_start + distance_to_end),
                };
                self.frontier.push(new_node);
            }
        }
    }

    fn retrace_path(&mut self, node: Node) {
        let mut current_node = node;
        while let Some(node_rc) = current_node.parent {
            self.path.push(current_node.point);
            current_node = (*node_rc.deref()).clone();
        }
    }

    fn find_start(img: &DynamicImage) -> Option<(u32, u32)> {
        let dimensions = img.dimensions();
        for x in 0..dimensions.0 {
            for y in 0..dimensions.1 {
                let pixel = img.get_pixel(x, y);
                if Self::is_start(&pixel) {
                    return Some((x, y));
                }
            }
        }
        None
    }

    fn find_end(img: &DynamicImage) -> Option<(u32, u32)> {
        let dimensions = img.dimensions();
        for x in 0..dimensions.0 {
            for y in 0..dimensions.1 {
                let pixel = img.get_pixel(x, y);
                if Self::is_end(&pixel) {
                    return Some((x, y));
                }
            }
        }
        None
    }

    fn distance(from: (u32, u32), to: (u32, u32)) -> f64 {
        let dx = (to.0 as i32 - from.0 as i32) as f64;
        let dy = (to.1 as i32 - from.1 as i32) as f64;
        (dx * dx + dy * dy).sqrt()
    }

    fn is_start(pixel: &Rgba<u8>) -> bool {
        let r = pixel.index(0);
        let g = pixel.index(1);
        let b = pixel.index(2);
        if *r < 100 && *g > 150 && *b < 100 {
            return true;
        }
        false
    }

    fn is_end(pixel: &Rgba<u8>) -> bool {
        let r = pixel.index(0);
        let g = pixel.index(1);
        let b = pixel.index(2);
        if *r > 150 && *g < 100 && *b < 100 {
            return true;
        }
        false
    }

    fn is_wall(pixel: &Rgba<u8>) -> bool {
        let r = pixel.index(0);
        let g = pixel.index(1);
        let b = pixel.index(2);
        if *r < 100 && *g < 100 && *b < 100 {
            return true;
        }
        false
    }
}

fn main() {
    let mut maze = MazeGraph::new("input_3.jpg");
    maze.run();
}
