use crate::action::Action;

#[derive(Default, Debug, Copy, Clone, PartialEq)]

pub(crate) struct State {
    pub(crate) is_backward: bool,
    pub(crate) is_fast: bool,
}
impl State {
    pub(crate) fn toggle_backward(&mut self) {
        self.is_backward = !self.is_backward;
    }
    pub(crate) fn toggle_fast(&mut self) {
        self.is_fast = !self.is_fast;
    }
    pub(crate) fn assemble_single_action(&self, cmd: char) -> Vec<Action> {
        let mut actions = Vec::new();
        if self.is_fast {
            actions.push(self.assemble_move_action());
        }
        match cmd {
            'M' => actions.push(self.assemble_move_action()),
            'L' | 'R' => actions.push(self.assemble_turn_action(cmd)),
            _ => (),
        }
        actions
    }
    pub(crate) fn assemble_multiple_actions(&self, cmd: &str) -> Vec<Action> {
        let mut actions = Vec::new();
        for c in cmd.chars() {
            actions.extend(self.assemble_single_action(c));
        }
        actions
    }
    fn assemble_move_action(&self) -> Action {
        if self.is_backward {
            Action::Step(-1)
        } else {
            Action::Step(1)
        }
    }
    fn assemble_turn_action(&self, direction: char) -> Action {
        if self.is_backward {
            Action::TurnDirection(if direction == 'L' { 'R' } else { 'L' })
        } else {
            Action::TurnDirection(direction)
        }
    }
}
