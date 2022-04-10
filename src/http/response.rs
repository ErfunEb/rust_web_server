use super::StatusCode;
use std::collections::HashMap;
use std::io::{Result as IoResult, Write};

#[derive(Debug)]
pub struct Response<'a> {
  status_code: StatusCode,
  body: Option<String>,
  headers: Option<HashMap<&'a str, &'a str>>,
}

impl<'a> Response<'a> {
  pub fn new(
    status_code: StatusCode,
    body: Option<String>,
    headers: Option<HashMap<&'a str, &'a str>>,
  ) -> Self {
    Self {
      status_code,
      body,
      headers,
    }
  }

  pub fn send(&self, stream: &mut dyn Write) -> IoResult<()> {
    let body = match &self.body {
      Some(b) => b,
      None => "",
    };
    let mut header_string = String::new();

    if let Some(headers) = &self.headers {
      for (key, value) in headers.iter() {
        header_string.push_str(&format!("{}: {}\r\n", key, value));
      }
    };

    write!(
      stream,
      "HTTP/1.1 {} {}\r\n{}\r\n\r\n{}",
      self.status_code,
      self.status_code.reason_phrase(),
      header_string,
      body
    )
  }
}
