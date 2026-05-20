use std::default;

#[derive(Default, Debug, Copy, Clone, PartialEq)]

pub(crate) struct State {
    pub(crate) is_backward: bool,
}
impl State {
    pub(crate) fn toggle_backward(&mut self) {
        self.is_backward = !self.is_backward;
    }
}
