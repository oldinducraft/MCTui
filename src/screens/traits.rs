use crossterm::event::KeyEvent;
use ratatui::Frame;

pub trait ScreenTrait: RenderableScreen + ScreenEvents {}

pub trait RenderableScreen {
    fn render(&mut self, frame: &mut Frame);
}

pub trait ScreenEvents {
    fn on_key_pressed(&mut self, event: KeyEvent) { let _ = event; }

    fn on_tick(&mut self) {}

    fn on_screen_changed(&mut self) {}

    fn on_exit(&mut self) {}
}
