use crate::cli::{Action, Event};

impl Action {
    pub fn is_debounced(&self) -> &bool {
        return &self.debounce;
    }
    pub fn cancels_debounce(&self) -> bool {
        return &self.event == &Event::Hoverlost;
    }
    pub fn without_debounce(mut self) -> Self {
        self.debounce = false;
        return self;
    }
}
