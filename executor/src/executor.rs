use crate::pose::Pose;
use crate::state::State;
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
                'B' => self.state.toggle_backward(),
                'F' => self.state.toggle_fast(),
                _ => {
                    let actions = self.state.assemble_single_action(cmd);
                    for action in actions {
                        action.execute_action(&mut self.pose);
                    }
                }
            }
        }
    }

    pub fn query(&self) -> Pose {
        self.pose
    }
}
