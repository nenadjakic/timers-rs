use std::time::{SystemTime, UNIX_EPOCH};

use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{self, Span, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, PROJECT_INPUT_PANEL_INDEX, PROJECT_LIST_PANEL_INDEX, TIMER_BUTTONS_PANEL_INDEX, TIMER_LIST_PANEL_INDEX};

pub fn render(frame: &mut Frame) {
    let text = Text::raw("Work in progress!");
    frame.render_widget(text, frame.area());
}

pub fn draw(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(25),
    ])
    .split(frame.area());

    draw_header(frame, app, chunks[0], TIMER_BUTTONS_PANEL_INDEX);
    draw_content(frame, app, chunks[1]);
    draw_text(frame, chunks[2]);
}

fn draw_header(frame: &mut Frame, app: &mut App, area: Rect, panel_index: usize) {
    let (border_color, border_type) = get_border_styles(app.selected_panel_index == panel_index);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(border_type)
        .border_style(Style::default().fg(border_color));
    frame.render_widget(block, area);

    let chunks = Layout::vertical([Constraint::Length(2), Constraint::Min(2)]).split(area);
    let chunks = Layout::horizontal([
        Constraint::Length(1),
        Constraint::Fill(2),
        Constraint::Fill(1),
        Constraint::Length(20),
        Constraint::Length(1),
    ])
    .split(chunks[1]);

    let timer_chunks = Layout::horizontal(
        app.timer_buttons
            .items
            .iter()
            .map(|_| Constraint::Length(15))
            .collect::<Vec<_>>(),
    )
    .split(chunks[2]);

    let selected_button_index = app.timer_buttons.state.selected().unwrap_or(9999);
    app.timer_buttons
        .items
        .iter()
        .enumerate()
        .for_each(|(i, b)| {
            let selected = i == selected_button_index;
            let content = format!(
                "{}{}{}",
                if selected { "[ " } else { "  " },
                b.text,
                if selected { " ]" } else { "  " }
            );
            let paragraph = Paragraph::new(text::Line::from(Span::styled(
                content,
                Style::default().fg(Color::Green),
            )))
            .alignment(Alignment::Right);
            frame.render_widget(paragraph, timer_chunks[i]);
        });
    let selected_project_name = app
        .projects
        .selected()
        .map(|project| project.name.clone())
        .unwrap_or("No project selected".to_owned());

    let content_project = Paragraph::new(Text::from(Span::styled(
        selected_project_name,
        Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Green),
    )));

    let time_text = get_elapsed_time_since_midnight();

    let content_time = Paragraph::new(Text::from(Span::styled(
        time_text.to_string(),
        Style::default().add_modifier(Modifier::BOLD).fg(Color::Red),
    )))
    .alignment(Alignment::Right);

    frame.render_widget(content_project, chunks[1]);
    frame.render_widget(content_time, chunks[3]);
}

fn draw_content(frame: &mut Frame, app: &mut App, area: Rect) {
    let constraints = vec![Constraint::Percentage(50), Constraint::Percentage(50)];
    let chunks = Layout::horizontal(constraints).split(area);
    draw_timer_list(frame, app, chunks[1], TIMER_LIST_PANEL_INDEX);

    let chunks =
        Layout::vertical(vec![Constraint::Fill(1), Constraint::Length(10)]).split(chunks[0]);

    draw_project_list(frame, app, chunks[0], PROJECT_LIST_PANEL_INDEX);
    draw_project_input(frame, app, chunks[1], PROJECT_INPUT_PANEL_INDEX);
}

fn draw_project_list(frame: &mut Frame, app: &mut App, area: Rect, panel_index: usize) {
    let (border_color, border_type) = get_border_styles(app.selected_panel_index == panel_index);

    let projects: Vec<ListItem> = app
        .projects
        .items
        .iter()
        .map(|project| ListItem::new(vec![text::Line::from(Span::raw(project.name.clone()))]))
        .collect();

    let projects = List::new(projects)
        .block(
            Block::bordered()
                .title("Projects")
                .border_type(border_type)
                .border_style(Style::default().fg(border_color)),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    frame.render_stateful_widget(projects, area, &mut app.projects.state);
}

fn draw_timer_list(frame: &mut Frame, app: &mut App, area: Rect, panel_index: usize) {
    let (border_color, border_type) = get_border_styles(app.selected_panel_index == panel_index);

    let block = Block::default()
        .title("TIMER LIST")
        .borders(Borders::ALL)
        .border_type(border_type)
        .border_style(Style::default().fg(border_color));

    frame.render_widget(block, area);
}
fn draw_project_input(frame: &mut Frame, app: &mut App, area: Rect, panel_index: usize) {
    let (border_color, border_type) = get_border_styles(app.selected_panel_index == panel_index);

    let block = Block::default()
        .title("INPUT_PROJECT")
        .borders(Borders::ALL)
        .border_type(border_type)
        .border_style(Style::default().fg(border_color));

    frame.render_widget(block, area);
}

fn draw_text(frame: &mut Frame, area: Rect) {
    let text = vec![
        text::Line::from("TimerRS is a simple application for managing projects and tracking time spent on them. It allows users to:"),
        text::Line::from(""),
        text::Line::from(Span::styled("✅ Create and view projects", Style::default().fg(Color::Red))),
        text::Line::from(Span::styled("✅ Start and stop timers for specific project", Style::default().fg(Color::Green))),
        text::Line::from(Span::styled("✅ Maintain a list of favorite projects", Style::default().fg(Color::Green))),
        text::Line::from(
            "One more thing is that it should display unicode characters: 10€"
        ),
    ];
    let block = Block::bordered().title(Span::styled(
        "Footer",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    frame.render_widget(paragraph, area);
}

fn get_elapsed_time_since_midnight() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let midnight = now - (now % 86400);
    let elapsed = now - midnight;
    let hours = elapsed / 3600;
    let minutes = (elapsed % 3600) / 60;
    let seconds = elapsed % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

fn get_border_styles(selected: bool) -> (Color, BorderType) {
    let mut border_color = Color::White;
    let mut border_type = BorderType::Plain;
    if selected {
        border_color = Color::Magenta;
        border_type = BorderType::Thick;
    }
    (border_color, border_type)
}
