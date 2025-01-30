use std::time::{SystemTime, UNIX_EPOCH};

use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{self, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::App;

pub fn render(frame: &mut Frame) {
    let text = Text::raw("Work in progress!");
    frame.render_widget(text, frame.area());
}

pub fn draw(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(frame.area());

    draw_header(frame, app, chunks[0]);

    let constraints = vec![Constraint::Percentage(50), Constraint::Percentage(50)];
    let chunks = Layout::horizontal(constraints).split(chunks[1]);
    let chunks =
        Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).split(chunks[0]);
    draw_project_list(frame, app, chunks[0]);
    draw_text(frame, chunks[1]);
}
/*

    let timer_buttons: Vec<Paragraph<'_>> = app
        .timer_buttons
        .items
        .iter()
        .map(|b| {
            let selected = if b.selected { true } else { false };
            let content = format!(
                "{}{}{}",
                if selected { "[" } else { "" },
                b.text,
                if selected { "]" } else { "" }
            );
            Paragraph::new(text::Line::from(Span::styled(
                content,
                Style::default().fg(Color::Green),
            )))
        })
        .collect();

    let timer_chunks = Layout::horizontal(
        app.timer_buttons
            .items
            .iter()
            .map(|_| Constraint::Length(10))
            .collect::<Vec<_>>(),
    )
    .split(chunks[0]);

    let mut i = 0;
    timer_buttons.iter().for_each(|b| {

        frame.render_widget(b, timer_chunks[i]);
        i += 1;
    });

*/

fn draw_header(frame: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default().borders(Borders::ALL);
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
            .map(|_| Constraint::Length(10))
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

pub fn draw_project_list(frame: &mut Frame, app: &mut App, area: Rect) {
    let constraints = vec![Constraint::Percentage(50), Constraint::Percentage(50)];
    let chunks = Layout::horizontal(constraints).split(area);

    let projects: Vec<ListItem> = app
        .projects
        .items
        .iter()
        .map(|project| ListItem::new(vec![text::Line::from(Span::raw(project.name.clone()))]))
        .collect();

    let projects = List::new(projects)
        .block(Block::bordered().title("Projects"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    frame.render_stateful_widget(projects, chunks[0], &mut app.projects.state);
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
