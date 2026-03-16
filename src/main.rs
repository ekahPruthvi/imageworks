pub mod frame;

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let menu_items = vec!["", "Grayscale", "Compress"];
    let mut selected_index = 1;
    let mut frame_width_percent: u16 = 10;

    loop {
        terminal.draw(|f| {

            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(frame_width_percent), Constraint::Percentage(100-frame_width_percent)
                ])
                .split(f.area());

            let items: Vec<ListItem> = menu_items
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    if i == selected_index {
                        ListItem::new(format!(" ⯌ {}", item)) 
                    } else {
                        ListItem::new(format!("   {}", item))
                    }
                })
                .collect();

            let menu = List::new(items)
                .block(Block::default().title(" ImageWorks ").borders(Borders::ALL ^ Borders::RIGHT));
            
            f.render_widget(menu, chunks[0]);

            let preview = Paragraph::new("\n Image Preview will appear here...")
                .block(Block::default().title(" Preview ").borders(Borders::ALL));
            
            f.render_widget(preview, chunks[1]);

        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Up => {
                    if selected_index > 1 { selected_index -= 1; }
                }
                KeyCode::Down => {
                    if selected_index < menu_items.len() - 1 { selected_index += 1; }
                }
                KeyCode::Enter => {
                    if selected_index == 4 { break; } 
                }
                KeyCode::Char('=') | KeyCode::Char('+') => {
                    if frame_width_percent < 30 {
                        frame_width_percent += 5;
                    }
                }
                KeyCode::Char('-') | KeyCode::Char('_') => {
                    if frame_width_percent > 10 {
                        frame_width_percent -= 5;
                    }
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}