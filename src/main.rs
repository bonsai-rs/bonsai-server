#[macro_use]
extern crate rocket;

use std::{sync::Mutex, time::Instant};

use rocket::State;

struct Session {
    token: u16,
}

struct TakenTokens {
    token_list: Mutex<Vec<u16>>,
}

impl Session {
    pub fn new(token: u16) -> Self {
        Self { token }
    }

    pub fn get_token(&self) -> u16 {
        self.token
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(TakenTokens {
            token_list: Mutex::new(Vec::new()),
        })
        .mount("/session", routes![create_session])
}

#[get("/create")]
fn create_session(taken_tokens: &State<TakenTokens>) -> String {
    let session = Session::new(rand::random_range(1000..=9999));
    taken_tokens
        .token_list
        .lock()
        .unwrap()
        .push(session.get_token());
    session.token.to_string()
}
