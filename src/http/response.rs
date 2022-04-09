pub enum StatusCode {

}

pub struct Response {
  status: StatusCode,
  body: Option<String>,
}

impl Response {
  pub fn new(status: StatusCode, body: Option<String>) -> Self {
    Self { status, body }
  }
}
