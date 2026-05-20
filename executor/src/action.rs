use crate::pose;

pub(crate) enum Action {
    Step(i32),
    TurnDirection(char),
}

impl Action {
    pub(crate) fn execute_action(&self, pose: &mut pose::Pose) {
        match self {
            Action::Step(offset) => {
                pose.forward(*offset);
            }
            Action::TurnDirection(direction) => {
                pose.turn_direction(*direction);
            }
        }
    }
}
