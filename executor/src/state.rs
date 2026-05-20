use std::default;

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
}
