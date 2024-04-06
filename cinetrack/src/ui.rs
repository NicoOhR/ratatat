
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, CurrentScreen, AddingMovie};



/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

pub fn ui(f: &mut Frame, app: &App){
    let chunks = Layout::default().direction(Direction::Vertical).constraints([
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(3),
    ]).split(f.size());

    let title_block = Block::default().borders(Borders::ALL).style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Track Your Movies!", 
        Style::default().fg(Color::Green),
    )).block(title_block);

    f.render_widget(title, chunks[0]);

    let mut list_items = Vec::<ListItem>::new();

    for movie in app.entries.keys(){
        list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{: <25} : {}", movie, app.entries.get(movie).unwrap()),
            Style::default().fg(Color::Yellow),
        ))));
    }

    let list = List::new(list_items);

    f.render_widget(list, chunks[1]);

    let current_navigation_text = vec![
        match app.current_screen{
            CurrentScreen::Main => {
                Span::styled("Normal", Style::default().fg(Color::Green))
            }
            CurrentScreen::MoviePage => {
                Span::styled("Adding Movie", Style::default().fg(Color::Yellow))
            }
            CurrentScreen::Exiting => {
                Span::styled("Exiting", Style::default().fg(Color::LightRed))
            }
        }
        .to_owned(),
        Span::styled(" | ", Style::default().fg(Color::White)),
        {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    AddingMovie::Title => Span::styled(
                        "Editing Title",
                        Style::default().fg(Color::Green),
                    ),
                    AddingMovie::Rating => Span::styled(
                        "Editing Rating",
                        Style::default().fg(Color::LightGreen),
                    ),
                    AddingMovie::DateWatched => Span::styled(
                        "Editing Date",
                        Style::default().fg(Color::Green),
                    ),
                }
            } else {
                Span::styled(
                    "Not Editing Anything",
                    Style::default().fg(Color::White),
                )
            }
        },
    ];

}