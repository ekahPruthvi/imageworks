pub mod frame;

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout,Rect},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
    style::{Color, Style}
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;

fn help(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let menu_items = vec!["", "Grayscale", "Compress"];
    let mut selected_index = 1;
    let mut frame_width_percent: u16 = 10;
    let mut show_qu :bool = false;

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

            if show_qu {
                let area = help(60, 20, f.area()); // 60% wide, 20% tall
                
                // Clear the background so the image doesn't bleed through the menu
                f.render_widget(ratatui::widgets::Clear, area); 

                let help_text = " 󰌌  CONTROLS \n\n \
                                [+] Increase Menu  [-] Decrease Menu \n \
                                [Up/Down] Navigate [Enter] Select \n \
                                [?] Close Help     [q] Quit";

                let help_menu = Paragraph::new(help_text)
                    .block(Block::default()
                        .title(" Quick Help ")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Yellow)))
                    .alignment(ratatui::layout::Alignment::Center);

                f.render_widget(help_menu, area);
            }

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
                KeyCode::Char('?') | KeyCode::Char('h') => {
                    if !show_qu {
                        show_qu = true;
                    } else { show_qu = false; }
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}