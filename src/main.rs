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

    let menu_items = vec!["Open Image", "Grayscale", "Downsample", "Save", "Quit"];
    let mut selected_index = 0;

    loop {
        terminal.draw(|f| {
            // Divide screen into a Left (Menu) and Right (Preview) section
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
                .split(f.size());

            // 1. Create the Menu List
            let items: Vec<ListItem> = menu_items
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    if i == selected_index {
                        ListItem::new(format!(">> {}", item)) // Highlight selection
                    } else {
                        ListItem::new(format!("   {}", item))
                    }
                })
                .collect();

            let menu = List::new(items)
                .block(Block::default().title(" Image Menu ").borders(Borders::ALL));
            
            f.render_widget(menu, chunks[0]);

            // 2. Create the Preview Area
            let preview = Paragraph::new("Image Preview will appear here...")
                .block(Block::default().title(" Preview ").borders(Borders::ALL));
            
            f.render_widget(preview, chunks[1]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Up => {
                    if selected_index > 0 { selected_index -= 1; }
                }
                KeyCode::Down => {
                    if selected_index < menu_items.len() - 1 { selected_index += 1; }
                }
                KeyCode::Enter => {
                    if selected_index == 4 { break; } // Quit
                    // Add logic here for other menu items!
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}