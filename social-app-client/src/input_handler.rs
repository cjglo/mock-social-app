use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use reqwest::blocking::Client;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::mem;

pub struct InputHandler {
    input: String,
    username: Option<String>,
    server_url: String,
    post_log: Vec<String>,
}

impl InputHandler {
    pub fn new(server_url: String) -> InputHandler {
        InputHandler {
            input: String::new(),
            username: None,
            server_url,
            post_log: Vec::new(),
        }
    }

    pub fn handle_text(&mut self, key: KeyEvent) {
        let mut to_send: Option<String> = None;

        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char(c) => {
                    if c.is_alphanumeric() {
                        self.input.push(c);
                    }
                }
                KeyCode::Backspace => {
                    if self.input.len() > 0 {
                        self.input.pop();
                    }
                }
                KeyCode::Enter if self.username.is_some() => {
                    let text = mem::replace(&mut self.input, String::new());
                    to_send = Some(text);
                }
                KeyCode::Enter if self.username.is_none() && self.input.len() > 0 => {
                    self.username = Some(mem::replace(&mut self.input, String::new()));
                }
                _ => {}
            }
        }

        if let Some(content) = to_send {
            Self::send_request_to_server(self, content.as_str());
        }
    }

    fn send_request_to_server(&mut self, content: &str) {
        let mut map = HashMap::new();
        map.insert("user", self.username.as_ref().unwrap().as_str());
        map.insert("content", content);
        let client = Client::new();
        let resp = client
            .post(&self.server_url)
            .json(&map)
            .send()
            .expect("Couldn't send to server!")
            .text();

        if resp.is_ok() {
            self.post_log
                .push(self.username.as_ref().unwrap().to_string() + ": " + content);
        }
    }
}

impl Display for InputHandler {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(ref name) = self.username {
            let mut display = String::new();
            for each in self.post_log.iter() {
                display += each;
                display.push('\n');
            }
            write!(f, "{display}{name}: {}", self.input)
        } else {
            write!(f, "{}", self.input)
        }
    }
}
