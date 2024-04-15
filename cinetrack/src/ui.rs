use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{AddingMovie, App, CurrentScreen};

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

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Track Your Movies!",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    f.render_widget(title, chunks[0]);

    let mut list_items = Vec::<ListItem>::new();

    for movie in app.entries.keys() {
        list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{: <25} : {}", movie, app.entries.get(movie).unwrap()),
            Style::default().fg(Color::Yellow),
        ))));
    }

    let list = List::new(list_items);

    f.render_widget(list, chunks[1]);

    let current_navigation_text = vec![
        match app.current_screen {
            CurrentScreen::Main => Span::styled("Normal", Style::default().fg(Color::Green)),
            CurrentScreen::MoviePage => {
                Span::styled("Adding Movie", Style::default().fg(Color::Yellow))
            }
            CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::LightRed)),
        }
        .to_owned(),
        Span::styled(" | ", Style::default().fg(Color::White)),
        {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    AddingMovie::Title => {
                        Span::styled("Editing Title", Style::default().fg(Color::Green))
                    }
                    AddingMovie::Rating => {
                        Span::styled("Editing Rating", Style::default().fg(Color::LightGreen))
                    }
                    AddingMovie::DateWatched => {
                        Span::styled("Editing Date", Style::default().fg(Color::Green))
                    }
                }
            } else {
                Span::styled("Not Editing Anything", Style::default().fg(Color::White))
            }
        },
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled("(q) to quit", Style::default().fg(Color::Red)),
            CurrentScreen::Exiting => Span::styled("cancel", Style::default().fg(Color::Red)),
            CurrentScreen::MoviePage => Span::styled("quit", Style::default().fg(Color::Red)),
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    f.render_widget(mode_footer, footer_chunks[0]);
    f.render_widget(key_notes_footer, footer_chunks[1]);

    if let Some(editing) = &app.currently_editing {
        let popup_block = Block::default()
            .title("Enter a new movie")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));
        let area = centered_rect(60, 25, f.size());
        f.render_widget(popup_block, area);

        let popup_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(30),
                Constraint::Percentage(20),
            ])
            .split(area);

        let mut title_block = Block::default().title("Name").borders(Borders::ALL);
        let mut date_block = Block::default().title("Date").borders(Borders::ALL);
        let mut rating_block = Block::default().title("Rating").borders(Borders::ALL);

        let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

        match editing {
            AddingMovie::Title => title_block = title_block.style(active_style),
            AddingMovie::DateWatched => date_block = date_block.style(active_style),
            AddingMovie::Rating => rating_block = rating_block.style(active_style),
        };

        let title_text = Paragraph::new(app.movie_name_input.clone()).block(title_block);
        f.render_widget(title_text, popup_chunks[0]);

        let date_text = Paragraph::new(app.date_watched_input.clone()).block(date_block);
        f.render_widget(date_text, popup_chunks[1]);

        let rating_text = Paragraph::new(app.rating_input.clone()).block(rating_block);
        f.render_widget(rating_text, popup_chunks[2]);

        if let CurrentScreen::Exiting = app.current_screen {
            f.render_widget(Clear, f.size()); //this clears the entire screen and anything already drawn
            let popup_block = Block::default()
                .title("Y/N")
                .borders(Borders::NONE)
                .style(Style::default().bg(Color::DarkGray));

            let exit_text = Text::styled(
                "Would you like to save? (y/n)",
                Style::default().fg(Color::Red),
            );
            // the `trim: false` will stop the text from being cut off when over the edge of the block
            let exit_paragraph = Paragraph::new(exit_text)
                .block(popup_block)
                .wrap(Wrap { trim: false });

            let area = centered_rect(60, 25, f.size());
            f.render_widget(exit_paragraph, area);
        }
    }
}

