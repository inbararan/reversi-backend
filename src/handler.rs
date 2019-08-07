use super::game;
use super::game::Game;
use super::position::Position;

#[derive(Debug, PartialEq, Eq)]
pub enum Request {
    Start,                                                                  /* Start of the game */
    DoTurn(Position),                                                       /* A move was played */
    Cancel                                                                  /* Cancel last operation */
}

pub enum Response {
    Update(game::ChangeSummary),                                            /* A board update */
    Error(String)                                                           /* Unrecoverable error */
}

pub struct Handler {
    game: Option<Game>       /* "None" indicates no game is currently run - no game was started at all or no game was started since last error */
}

impl Handler {
    pub fn new() -> Handler {
        Handler{ game: None }
    }

    fn summary_of(&mut self, task: impl Fn(&mut Game)->game::Result) -> Result<game::ChangeSummary, String> {
        self.game.as_mut().map_or(Err(String::from("No game is running")), |game| {
            match task(game) {
                Ok(()) => Ok(game.flush_changes()),
                Err(e) => {
                    game.discard_changes();
                    Err(e)
                }
            }
            
        })
    }

    fn handle_parsed(&mut self, request: Request) -> Response {
        let result = match request {
            Request::Start => {
                self.game = Some(Game::new());
                self.summary_of(|game| game.prepare_board())
            },
            Request::DoTurn(position) => self.summary_of(|game| game.do_turn(position)),
            Request::Cancel => self.summary_of(|game| game.cancel())
        };
        
        match result {
            Ok(summary) => Response::Update(summary),
            Err(error) => Response::Error(error)
        }
    }

    pub fn handle_raw(&mut self, request_raw: String) -> String {
        let response = match Request::parse(&request_raw) {
            Ok(request) => self.handle_parsed(request),
            Err(parsing_error) => Response::Error(parsing_error.to_string())
        };
        response.stringify()
    }
}