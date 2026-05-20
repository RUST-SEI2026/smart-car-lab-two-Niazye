use crate::pose::Pose;
#[derive(Debug, Copy, Clone, PartialEq)]

pub struct Executor {
    pose: Pose,
}

impl Executor {
    pub fn with_pose(pose: Pose) -> Self {
        Executor { pose }
    }

    pub fn execute(&mut self, cmds: &str) {
        for cmd in cmds.chars() {
            match cmd {
                'M' => self.pose.step(),
                'L' | 'R' => self.pose.turn_direction(cmd),
                _ => (),
            }
        }
    }

    pub fn query(&self) -> Pose {
        self.pose
    }
}
