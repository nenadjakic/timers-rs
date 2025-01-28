use ratatui::{text::Text, Frame};

pub fn render(frame: &mut Frame) {
    let text = Text::raw("Work in progress!");
    frame.render_widget(text, frame.area());
}
