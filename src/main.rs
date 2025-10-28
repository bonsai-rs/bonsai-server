#[macro_use]
extern crate rocket;

use chrono::Utc;
use core::time;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket::{State, serde::json::Json};
use std::{sync::Mutex, time::Instant};

#[derive(Clone)]
struct Session<'a> {
    token: u16,
    start_time: Option<String>,
    users: Vec<String>,
    terminating_user: Option<TerminatingUser<'a>>,
}

struct TakenTokens<'a> {
    token_list: Mutex<Vec<Session<'a>>>,
}

#[derive(Deserialize)]
struct StartSession<'a> {
    time: &'a str,
}

#[derive(Deserialize)]
struct UserData<'a> {
    username: &'a str,
}

#[derive(Serialize, Clone, Copy)]
struct TerminatingUser<'a> {
    user: &'a str,
    time: &'a str,
    token: u16,
}

impl<'a> TerminatingUser<'a> {
    pub fn get_user(&self) -> String {
        self.user.clone().to_owned()
    }
    pub fn get_token(&self) -> u16 {
        self.token
    }
    pub fn get_time() -> String {
        Utc::now().to_rfc3339()
    }
    pub fn set_user(&mut self, user: &'a str) {
        self.user = user;
    }
    pub fn set_token(&mut self, token: u16) {
        self.token = token;
    }
    pub fn set_time(&mut self, time: &'a str) {
        self.time = time;
    }
}

impl<'a> Session<'a> {
    pub fn new(token: u16) -> Self {
        Self {
            token,
            start_time: None,
            users: Vec::new(),
            terminating_user: None,
        }
    }

    pub fn get_session(token: u16, taken_tokens: &State<TakenTokens>) -> Session<'a> {
        let mut needed_session = taken_tokens.token_list.lock().unwrap();
        needed_session
            .iter()
            .find(|t| t.token == token)
            .unwrap()
            .to_owned()
    }

    pub fn get_token(&self) -> u16 {
        self.token
    }

    pub fn get_users_list(&self) -> Vec<String> {
        self.users.clone()
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(TakenTokens {
            token_list: Mutex::new(Vec::new()),
        })
        .mount(
            "/session",
            routes![
                create_session,
                start_session,
                join_session,
                terminate_session,
                terminate_notify,
                get_users_list
            ],
        )
}

fn is_token_taken(token: u16, taken_tokens: &State<TakenTokens>) -> bool {
    for item in &*taken_tokens.token_list.lock().unwrap() {
        if token == item.get_token() {
            return true;
        }
    }
    false
}

#[get("/userlist/<token>")]
fn get_users_list(token: u16, taken_tokens: &State<TakenTokens>) -> String {
    let my_session = Session::get_session(token, taken_tokens);
    serde_json::to_string(&my_session.get_users_list()).unwrap()
}

#[get("/create")]
fn create_session(taken_tokens: &State<TakenTokens>) -> String {
    let mut session;
    loop {
        session = Session::new(rand::random_range(1000..=9999));
        if !is_token_taken(session.get_token(), taken_tokens) {
            break;
        }
    }
    taken_tokens
        .token_list
        .lock()
        .unwrap()
        .push(session.clone());
    session.token.to_string()
}

#[post("/start/<token>", format = "application/json", data = "<start_time>")]
fn start_session(taken_tokens: &State<TakenTokens>, token: u16, start_time: Json<StartSession>) {
    let mut needed_session = Session::get_session(token, taken_tokens);
    needed_session.start_time = Some(start_time.time.to_string());
}

#[post("/join/<token>", format = "application/json", data = "<user>")]
fn join_session(token: u16, user: Json<UserData>, taken_tokens: &State<TakenTokens>) {
    let mut needed_session = Session::get_session(token, taken_tokens);
    needed_session.users.push(user.username.to_string());
}

#[post("/terminate/<token>", format = "application/json", data = "<user>")]
fn terminate_session(token: u16, user: String, taken_tokens: &State<TakenTokens>) {
    let needed_session = Session::get_session(token, taken_tokens);
    if !needed_session.terminating_user.is_some() {
        return;
    }
    needed_session.terminating_user.unwrap().set_user(&user);
    needed_session.terminating_user.unwrap().set_token(token);
    needed_session
        .terminating_user
        .unwrap()
        .set_time(&Utc::now().to_rfc3339());
}

#[get("/end/<token>")]
fn terminate_notify(token: u16, taken_tokens: &State<TakenTokens>) -> String {
    let my_session = Session::get_session(token, taken_tokens);
    if my_session.terminating_user.is_some() {
        return "true".to_string();
    }
    "false".to_string()
}
