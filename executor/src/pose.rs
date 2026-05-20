#[derive(Debug, Copy, Clone, PartialEq)]

pub struct Pose {
    pub x: i32,
    pub y: i32,
    pub heading: char,
}

impl Pose {
    pub fn new(x: i32, y: i32, heading: char) -> Self {
        Pose { x, y, heading }
    }
    pub fn step(&mut self, offset: i32) {
        match self.heading {
            'E' => self.x += offset,
            'S' => self.y -= offset,
            'W' => self.x -= offset,
            'N' => self.y += offset,
            _ => (),
        }
    }
    pub fn turn_direction(&mut self, direction: char) {
        match direction {
            'L' => match self.heading {
                'E' => self.heading = 'N',
                'S' => self.heading = 'E',
                'W' => self.heading = 'S',
                'N' => self.heading = 'W',
                _ => (),
            },
            'R' => match self.heading {
                'E' => self.heading = 'S',
                'S' => self.heading = 'W',
                'W' => self.heading = 'N',
                'N' => self.heading = 'E',
                _ => (),
            },
            _ => (),
        }
    }
}

impl Default for Pose {
    fn default() -> Self {
        Pose {
            x: 0,
            y: 0,
            heading: 'N',
        }
    }
}
