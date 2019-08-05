use super::handler::{Request, Response};
use super::board::{Position, Cell, Color};

#[derive(Debug, PartialEq ,Eq)]
pub struct ParsingError { message: String, token: String}

impl ParsingError {
    fn invalid_number(role: &'static str, token: &str) -> ParsingError {
        ParsingError{
            message: format!("Could not be parsed into a number ({})", role),
            token: token.to_string()
        }
    }

    fn missing(role: &'static str, token: &str) -> ParsingError {
        ParsingError{
            message: format!("Could not find {}", role),
            token: token.to_string()
        }
    }

    fn unrecognized_request_type(token: &str) -> ParsingError {
        ParsingError{
            message: format!("Could not recognize request type"),
            token: token.to_string()
        }
    }
}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ParsingError! Could not parse \"{}\". {}.", self.token, self.message)
    }
}

impl Position {
    fn parse(source: &str) -> Result<Position, ParsingError> {
        let mut split = source.split(",");
        let x_str = split.next().expect("Impossible event: First next() of source.split() returned None (Position::parse)");
        let x = x_str.parse::<usize>().or_else(|_| Err(ParsingError::invalid_number("Position::x", x_str)))?;
        let y_str = split.next().ok_or(ParsingError::missing("Position::y", source))?;
        let y = y_str.parse::<usize>().or_else(|_| Err(ParsingError::invalid_number("Position::y", y_str)))?;
        Ok(Position{x: x, y: y})
    }
}

impl Request {
    pub fn parse(source: &str) -> Result<Request, ParsingError> {
        let mut split = source.split(";");
        let request_type: &str = split.next().expect("Impossible event: First next() of source.split() returned None (Request::parse)");
        match request_type {
            "Start" => Ok(Request::Start),
            "DoTurn" => {
                let details = split.next().ok_or(ParsingError::missing("Request::DoTurn::Position", source))?;
                Ok(Request::DoTurn(Position::parse(details)?))
            },
            "Cancel" => Ok(Request::Cancel),
            _ => Err(ParsingError::unrecognized_request_type(request_type))
        }
    }
}

impl Position {
    fn stringify(&self) -> String {
        format!("{}.{}", self.x, self.y)
    }
}

impl Cell {
    fn stringify(&self) -> &'static str {
        match &self.0 {
            Some(color) => match color {
                Color::White => "255.255.255",
                Color::Black => "0.0.0"
            },
            None => "64.64.64"
        }
    }
}

impl Response {
    pub fn stringify(&self) -> String {
        match self {
            Response::Update(details) => {
                let mut details_raw = details.iter()
                                         .map(
                                             |(pos, cell)| {
                                                 format!("{}:{}", pos.stringify(), cell.stringify())
                                             })
                                         .fold(String::from(""), |acc, val| {
                                             acc + &val + ","
                                         });
                if details_raw.len() > 0 { details_raw.pop(); } // Remove last comma if needed
                format!("Update;{}", details_raw)
            },
            Response::Error(details) => {
                format!("Error;{}", details)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ParsingError;
    use super::super::handler::{Request, Response};
    use super::super::board::{Position, Cell, Color};
    use std::collections::HashMap;

    #[test]
    fn request_test_empty() {
        assert_eq!(Request::parse(""), Err(ParsingError::unrecognized_request_type("")));
    }

    #[test]
    fn request_test_non_existent_type() {
        assert_eq!(Request::parse("NonExistent"), Err(ParsingError::unrecognized_request_type("NonExistent")));
        assert_eq!(Request::parse("StartIt"), Err(ParsingError::unrecognized_request_type("StartIt")));
    }

    #[test]
    fn request_test_start_and_cancel() {
        assert_eq!(Request::parse("Start"), Ok(Request::Start));
        assert_eq!(Request::parse("Cancel"), Ok(Request::Cancel));
    }

    #[test]
    fn request_test_do_turn() {
        assert_eq!(Request::parse("DoTurn;2,4"), Ok(Request::DoTurn(Position{x: 2, y: 4})));
    }

    #[test]
    fn request_test_do_turn_no_position() {
        assert_eq!(Request::parse("DoTurn"), Err(ParsingError::missing("Request::DoTurn::Position", "DoTurn")));
    }

    #[test]
    fn request_test_do_turn_position_no_y() {
        assert_eq!(Request::parse("DoTurn;2"), Err(ParsingError::missing("Position::y", "2")));
    }
    
    #[test]
    fn request_test_do_turn_position_empty_x() {
        assert_eq!(Request::parse("DoTurn;,8"), Err(ParsingError::invalid_number("Position::x", "")));
    }

    #[test]
    fn request_test_do_turn_position_empty_y() {
        assert_eq!(Request::parse("DoTurn;2,"), Err(ParsingError::invalid_number("Position::y", "")));
    }
    
    #[test]
    fn request_test_do_turn_position_wrong_delimiter() {
        assert_eq!(Request::parse("DoTurn;2.4"), Err(ParsingError::invalid_number("Position::x", "2.4")));
    }

    #[test]
    fn response_test_update_no_pairs() {
        let details: HashMap<Position, Cell> = HashMap::new();
        assert_eq!(Response::Update(details).stringify(), "Update;")
    }
    #[test]
    fn response_test_update_one_pair() {
        let mut details: HashMap<Position, Cell> = HashMap::new();
        details.insert(Position{x: 4, y: 5}, Cell(Some(Color::White)));
        assert_eq!(Response::Update(details).stringify(), "Update;4.5:255.255.255")
    }
    #[test]
    fn response_test_update_two_pairs() {
        let mut details: HashMap<Position, Cell> = HashMap::new();
        details.insert(Position{x: 8, y: 5}, Cell(None));
        details.insert(Position{x: 4, y: 5}, Cell(Some(Color::White)));
        let actual = Response::Update(details).stringify();
        let expected1 = "Update;4.5:255.255.255,8.5:64.64.64";
        let expected2 = "Update;8.5:64.64.64,4.5:255.255.255";
        assert!(actual == expected1 || actual == expected2, "{} isn't equeal to {} nor {}", actual, expected1, expected2);
    }
    #[test]
    fn response_test_error() {
        assert_eq!(Response::Error("Custom error message".to_string()).stringify(), "Error;Custom error message");
    }
}