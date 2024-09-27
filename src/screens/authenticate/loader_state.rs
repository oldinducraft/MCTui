use throbber_widgets_tui::ThrobberState;

#[derive(Default)]
pub struct LoaderState {
    pub(super) throbber_state: ThrobberState,
}

impl LoaderState {
    pub fn on_tick(&mut self) { self.throbber_state.calc_next(); }
}
