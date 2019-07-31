use super::board::{Position, Cell};

#[derive(Debug, PartialEq, Eq)]
pub enum Request {
    Start,                                                                  /* Start of the game */
    Move(Position),                                                         /* A move was played */
    Cancel                                                                  /* Cancel last operation */
}

pub enum Response {
    Update(std::collections::HashMap<Position, Cell>),                      /* A board update */
    Error(String)                                                           /* Unrecoverable error */
}

pub fn handle_parsed(request: Request) -> Response {
    Response::Error(String::from("Unimplemented"))
}

pub fn handle_raw(request_raw: String) -> String {
    let response = match Request::parse(&request_raw) {
        Ok(request) => handle_parsed(request),
        Err(parsing_error) => Response::Error(parsing_error.to_string())
    };
    response.stringify()
}