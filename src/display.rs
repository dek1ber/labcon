use std::io::{self, stdout};

use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{
            disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
        },
        ExecutableCommand,
    },
    prelude::*,
    widgets::*,
};


pub fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

pub fn ui(frame: &mut Frame) {
    let main_layout = Layout::new(
        Direction::Vertical, 
        [
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ],
    )
    .split(frame.size());

    frame.render_widget(
        Block::new().borders(Borders::TOP).title("Homelab"), main_layout[0],
    );
    frame.render_widget(
        Block::new().borders(Borders::TOP).title("Status Bar"), main_layout[2],
    );

    let inner_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(50), Constraint::Percentage(50)],
    )
    .split(main_layout[1]);

    frame.render_widget(Block::bordered().title("Left"), inner_layout[0]);
    frame.render_widget(Block::bordered().title("Right"), inner_layout[1]);

    let progress_widget = Gauge::default()
        .block(Block::bordered().title("Progress"))
        .gauge_style(
            Style::default()
                .fg(Color::Red)
                .bg(Color::White)
                .add_modifier(Modifier::ITALIC),
            )
        .percent(20);
    frame.render_widget(progress_widget, inner_layout[0]);
}

pub fn draw(lab_name: String) -> io::Result<()> {

    dbg!(lab_name);

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(ui)?;
        should_quit = handle_events()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
