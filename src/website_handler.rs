use super::http::{Method, Request, Response, StatusCode};
use super::server::Handler;
use std::collections::HashMap;
use std::fs;

pub struct WebsiteHandler {
  public_path: String,
}

impl WebsiteHandler {
  pub fn new(public_path: String) -> Self {
    Self { public_path }
  }

  fn read_file(&self, file_path: &str) -> Option<String> {
    let path = format!("{}/{}", self.public_path, file_path);
    match fs::canonicalize(path) {
      Ok(path) => {
        if path.starts_with(&self.public_path) {
          fs::read_to_string(path).ok()
        } else {
          println!("Directory Traversal Attack Attempted: {}", file_path);
          None
        }
      }
      Err(_) => None,
    }
  }
}

impl Handler for WebsiteHandler {
  fn handle_request(&mut self, request: &Request) -> Response {
    match request.method() {
      Method::GET => match request.path() {
        "/" => Response::new(
          StatusCode::Ok,
          self.read_file("index.html"),
          Some(HashMap::from([("Content-Type", "text/html")])),
        ),
        "/json" => {
          let json_body = "{\"message\":\"Hello\"}".to_owned();
          Response::new(
            StatusCode::Ok,
            Some(json_body),
            Some(HashMap::from([("Content-Type", "application/json")])),
          )
        }
        path => match self.read_file(path) {
          Some(contents) => Response::new(StatusCode::Ok, Some(contents), None),
          None => Response::new(StatusCode::NotFound, None, None),
        },
      },
      _ => Response::new(StatusCode::NotFound, None, None),
    }
  }
}
