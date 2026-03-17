pub mod frame;

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout,Rect},
    widgets::{Block, Borders, List, ListItem, Paragraph, Row, Table},
    Terminal,
    style::{Color, Style, Modifier}
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;

use crate::frame::image_to_matrix;

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
    let mut open_qu :bool = false;
    let mut inputpp :String = "".to_string();

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
                        ListItem::new(format!(" ⯌ {}", item)).style(Style::default().add_modifier(Modifier::BOLD)) 
                    } else {
                        ListItem::new(format!("   {}", item))
                    }
                })
                .collect();

            let menu = List::new(items)
                .block(Block::default().title(" ImageWorks ").borders(Borders::ALL ^ Borders::RIGHT));
            
            f.render_widget(menu, chunks[0]);

            let preview = Paragraph::new("\n ░▀▀▀░░░\n ▒▒▒▒▒▒▒ empty working directory \n ▒▒▒▒▒▒▒")
                .block(Block::default().title(" Preview ").borders(Borders::ALL));
            
            f.render_widget(preview, chunks[1]);

            if show_qu {
                let area = help(60, 20, f.area());  
                // f.render_widget(ratatui::widgets::Clear, area); 

                let rows = vec![
                    Row::new(vec![" + / - ", " Increase / Decrease Menu width "]).style(Style::default().bg(Color::Rgb(30, 30, 30))),
                    Row::new(vec![" h / ? ", " This help menu toggle "]),
                    Row::new(vec![" o ", " Open new image using path "]).style(Style::default().bg(Color::Rgb(30, 30, 30))), 
                    Row::new(vec![" ↑ / ↓ ", " Navigate Menu "]),
                    Row::new(vec![" Return ", " Select Menu option "]).style(Style::default().bg(Color::Rgb(30, 30, 30))),
                    Row::new(vec![" → / ← ", " Navigate images from the working folder "]),
                    Row::new(vec![" s ", " Save image as edited "]).style(Style::default().bg(Color::Rgb(30, 30, 30))),
                    Row::new(vec![" q ", " Quit ImageWorks "]),
                    Row::new(vec![" PageUp ", " Credits to me ( - The Dev) "]).style(Style::default().bg(Color::Rgb(30, 30, 30)))
                ];

                let tablele = Table::new(rows, 
                        [
                            Constraint::Min(5),
                            Constraint::Min(20)
                        ]
                    )
                    .header(
                        Row::new(vec![" Key ", " Action "])
                            .style(Style::default().add_modifier(Modifier::ITALIC).add_modifier(Modifier::BOLD))
                            .bottom_margin(1)
                            .top_margin(1)
                    )
                    .block(Block::default()
                        .title(" Welp? ")
                        .borders(Borders::all())
                        .border_style(Style::default().fg(Color::Green))
                    )
                    .column_spacing(1);

                f.render_widget(tablele, area);
            }

            if open_qu {
                let area = help(90, 10, f.area());

                f.render_widget(ratatui::widgets::Clear, area);

                let inputdisp = Paragraph::new(inputpp.as_str())
                .block(Block::default().title(" Start Typing - [Enter] to select - [Esc] to 'esacpe' ").borders(Borders::ALL).style(Style::default().bg(Color::LightYellow).fg(Color::Black).add_modifier(Modifier::BOLD)));

                f.render_widget(inputdisp, area);
            }
            

        })?;

        if !open_qu && let Event::Key(key) = event::read()? {
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
                    show_qu = if !show_qu { true } else { false };
                }
                KeyCode::Char('o') => {
                    open_qu = if !open_qu { true } else { false };
                }

                _ => {}
            }
        } else if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => {
                    open_qu = false;
                    inputpp.clear();
                }
                KeyCode::Char(c) => {
                    inputpp.push(c);
                }
                KeyCode::Backspace => {
                    inputpp.pop();
                }
                KeyCode::Enter => {
                    let matrix = image_to_matrix(inputpp.to_string());
                    open_qu = false;
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}