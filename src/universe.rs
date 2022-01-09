use std::error::Error;
use tui::{
    widgets::canvas::{
        Points,
        Painter,
        Shape,
    },
    style::Color,
};

pub struct Universe {
    world: Vec<Vec<bool>>,
    size: (u32, u32),
}

impl Universe {
    pub fn size(&self) -> (u32, u32) {
        self.size
    }

    pub fn new(x_size: u32, y_size: u32) -> Universe {
        let mut y: Vec<Vec<bool>> = Vec::new();
        let mut x: Vec<bool> = Vec::new();
        x.resize(x_size as usize, false);
        y.resize(y_size as usize, x);

        Universe {
            world: y,
            size:  (x_size, y_size),
        }
    }

    pub fn add(&mut self, x: u32, y: u32) -> Result<(), Box<dyn Error>> {
        if self.size.0 < x || self.size.1 < y {
            return Err(format!("Point ({}, {}) is out of bounds of the universe.", x, y).into());
        }
        self.world[y as usize][x as usize] = true;
        Ok(())
    }

    pub fn update(&mut self) {
        // create new world for next iteration
        let mut new_world = self.world.clone();
        // somewhere in these loops something let's the program crash without panicing
        for y in 0..self.size.1 as i64 {
            for x in 0..self.size.0 as i64 {
                let mut neighbors = 0u8;

                for y_off in y-1..=y+1 {
                    for x_off in x-1..=x+1 {
                        if 0 <= x_off && x_off < self.size.0 as i64 &&
                           0 <= y_off && y_off < self.size.1 as i64 &&
                           !(y_off == y && x_off == x) {
                            if self.world[y_off as usize][x_off as usize] {
                                neighbors += 1;
                            }
                        }
                    }
                }
                if !self.world[y as usize][x as usize] {
                    if neighbors == 3 { new_world[y as usize][x as usize] = true; }
                } else {
                    match neighbors {
                        // fewer than 2 live neighbors => die (underpopulation)
                        0 | 1 => {
                            new_world[y as usize][x as usize] = false;
                            //eprintln!("({}, {}) dies of underpopulation ({})", x, y, neighbors);
                        },
                        // 2 or 3 live neighbors => stay alive
                        2 | 3 => {},//eprintln!("({}, {}) lives ({})", x, y, neighbors)},
                        // more than 3 live neighbors => die (overpopulation)
                        _ => {
                            new_world[y as usize][x as usize] = false;
                            //eprintln!("({}, {}) dies of overpopulation ({})", x, y, neighbors);
                        },
                    }
                }
            }
        }
        self.world = new_world;
    }

    pub fn from_string(string: String) -> Universe {
        let string_world: Vec<&str> = string.split("\n").collect();
        let x_len = string_world[0].len();
        let y_len = string_world.len();

        for line in &string_world {
            if line.as_bytes().len() != x_len {
                panic!("The given String can not be processed to a world. (All lines has to have the same length)");
            } 
        } 
 
        let mut world: Vec<Vec<bool>> = vec![];
 
        for y in 0..y_len {
            world.push(vec![]);
            let byte_row = string_world[y].as_bytes();
            for x in byte_row {
                world[y].push(*x == b'1');
            }
        }

        Universe { world, size: (x_len as u32, y_len as u32) }
    }

    pub fn to_string(&self) -> String {
        let mut string: String = String::new();
        for y in &self.world {
            for x in y {
                if *x { string.push('1'); }
                else { string.push('0'); }
            }
            string.push('\n');
        }
        string
    }
}

impl Shape for Universe {
    fn draw(&self, painter: &mut Painter) {
        let mut v: Vec<(f64, f64)> = vec![];
        for y in 0..self.size.1 as usize {
            for x in 0..self.size.0 as usize {
                if self.world[y][x] { {
                            v.push((x as f64, (self.size.1 - y as u32) as f64));
                    }
                }
            }
        }
        Points {
            coords: &v,
            color: Color::White,
        }.draw(painter);
    }
}
