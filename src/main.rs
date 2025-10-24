#[macro_use]
extern crate rocket;

use std::{sync::Mutex, time::Instant};

use rocket::State;

#[derive(Clone, Copy)]
struct Session {
    token: u16,
}

struct TakenTokens {
    token_list: Mutex<Vec<Session>>,
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

#[post("/start")]
fn start_session(token: u16) -> Instant {
   folet PartialOr
}

fn is_token_taken(token: u16, taken_tokens:&State<TakenTokens>) -> bool {
    for item in &*taken_tokens.token_list.lock().unwrap() {
        if token == item.get_token() {
            return true;
        }
    }
    false
}

#[get("/create")]
fn create_session(taken_tokens: &State<TakenTokens>) -> String {
    let mut session;
    loop {
        session = Session::new(rand::random_range(1000..=9999));
    if !is_token_taken(session.get_token(), taken_tokens) {
        break;
    }}
    taken_tokens
        .token_list
        .lock()
        .unwrap()
        .push(session.clone());
    session.token.to_string()
}
