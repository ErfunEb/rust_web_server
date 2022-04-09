use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
  GET,
  POST,
  PUT,
  PATCH,
  HEAD,
  CONNECT,
  OPTIONS,
  TRACE,
  DELETE,
}

impl FromStr for Method {
  type Err = MethodError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "GET" => Ok(Self::GET),
      "POST" => Ok(Self::POST),
      "PUT" => Ok(Self::PUT),
      "PATCH" => Ok(Self::PATCH),
      "HEAD" => Ok(Self::HEAD),
      "CONNECT" => Ok(Self::CONNECT),
      "OPTIONS" => Ok(Self::OPTIONS),
      "TRACE" => Ok(Self::TRACE),
      "DELETE" => Ok(Self::DELETE),
      _ => Err(MethodError)
    }
  }
}

pub struct MethodError; 
