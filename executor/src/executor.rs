use crate::state::State;
use crate::pose::Pose;
#[derive(Debug, Copy, Clone, PartialEq)]

pub struct Executor {
    pose: Pose,
    state: State,
}

impl Executor {
    pub fn with_pose(pose: Pose) -> Self {
        Executor {
            pose,
            state: State::default(),
        }
    }

    pub fn execute(&mut self, cmds: &str) {
        for cmd in cmds.chars() {
            if self.state.is_fast && (cmd == 'M' || cmd == 'L' || cmd == 'R') {
                if self.state.is_backward {
                    self.pose.step(-1);
                } else {
                    self.pose.step(1);
                }
            }
            match cmd {
                'M' => {
                    if self.state.is_backward {
                        self.pose.step(-1);
                    } else {
                        self.pose.step(1);
                    }
                }
                'L' => {
                    if self.state.is_backward {
                        self.pose.turn_direction('R');
                    } else {
                        self.pose.turn_direction('L');
                    }
                }
                'R' => {
                    if self.state.is_backward {
                        self.pose.turn_direction('L');
                    } else {
                        self.pose.turn_direction('R');
                    }
                }
                'B' => self.state.toggle_backward(),
                'F' => self.state.toggle_fast(),
                _ => (),
            }
        }
    }

    pub fn query(&self) -> Pose {
        self.pose
    }
}
