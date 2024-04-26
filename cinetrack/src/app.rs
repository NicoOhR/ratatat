use serde::ser::{Serialize, SerializeStruct, Serializer};

use chrono::NaiveDate;
use std::fmt;

pub enum CurrentScreen {
    Main,
    MoviePage,
    Exiting,
}
pub struct MovieInfo {
    pub date_watched: NaiveDate,
    pub rating: Rating,
}

pub enum AddingMovie {
    Title,
    DateWatched,
    Rating,
}
#[derive(Clone)]
pub struct Rating {
    pub score: u8,
    pub total: u8, //score out of, user will change later
}

impl Rating {
    fn new() -> Rating {
        let mut rating = Rating { score: 0, total: 5 };
        rating
    }
    fn set_score(&mut self, argscore: u8) {
        self.score = argscore;
    }
    pub fn increment(&mut self) {
        self.score += 1;
    }
    pub fn decrement(&mut self) {
        self.score -= 1;
    }
}

pub struct App {
    pub movie_name_input: String,
    pub date_watched_input: String,
    pub rating_input: Rating,
    pub entries: std::collections::HashMap<String, MovieInfo>, //should probably make movie info struct
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<AddingMovie>,
}

impl Serialize for Rating {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Rating", 2)?;
        state.serialize_field("score", &self.score)?;
        state.serialize_field("total", &self.total)?;
        state.end()
    }
}

impl fmt::Display for Rating {
    // This function must write the desired format into the given formatter
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Score: {}/{}", self.score, self.total) // Custom format for display
    }
}
impl Serialize for MovieInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("MovieInfo", 2)?;
        state.serialize_field("Date Watched", &self.date_watched)?;
        state.serialize_field("Rating", &self.rating)?;
        state.end()
    }
}
impl fmt::Display for MovieInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Date: {} Rating: {}", self.date_watched, self.rating)
    }
}

impl App {
    pub fn new() -> App {
        App {
            movie_name_input: String::new(),
            date_watched_input: String::new(),
            rating_input: Rating::new(),
            entries: std::collections::HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }

    pub fn save_new_movie(&mut self) {
        let mut movie_rating = Rating::new();
        movie_rating.set_score(self.rating_input.score.clone());

        let new_movie_info = MovieInfo {
            date_watched: NaiveDate::parse_from_str(&self.date_watched_input.clone(), "%Y-%m-%d")
                .unwrap(), //need error handling
            rating: movie_rating,
        };

        self.entries
            .insert(self.movie_name_input.clone(), new_movie_info);

        self.movie_name_input = String::new();
        self.date_watched_input = String::new();
        self.rating_input = Rating::new();

        self.currently_editing = None;
    }

    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                AddingMovie::Title => self.currently_editing = Some(AddingMovie::Title),
                AddingMovie::Rating => self.currently_editing = Some(AddingMovie::Rating),
                AddingMovie::DateWatched => self.currently_editing = Some(AddingMovie::DateWatched),
            };
        } else {
            self.currently_editing = Some(AddingMovie::Title);
        }
    }

    pub fn print_json(&self) -> std::io::Result<()> {
        let movie_list = serde_json::to_string(&self.entries)?;
        println!("{}", movie_list);
        Ok(())
    }
}
