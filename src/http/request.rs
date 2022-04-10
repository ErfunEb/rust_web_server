use super::method::{Method, MethodError};
use super::Header;
use super::QueryString;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

#[derive(Debug)]
pub struct Request<'buff> {
  path: &'buff str,
  query_string: Option<QueryString<'buff>>,
  headers: Header<'buff>,
  method: Method,
}

impl<'buff> Request<'buff> {
  pub fn path(&self) -> &str {
    &self.path
  }

  pub fn method(&self) -> &Method {
    &self.method
  }

  pub fn query_string(&self) -> Option<&QueryString> {
    self.query_string.as_ref()
  }

  pub fn headers(&self) -> &Header {
    &self.headers
  }
}

impl<'buff> TryFrom<&'buff [u8]> for Request<'buff> {
  type Error = ParseError;

  fn try_from(buffer: &'buff [u8]) -> Result<Request<'buff>, Self::Error> {
    let request = str::from_utf8(buffer)?;
    let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
    let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
    let (protocol, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
    let (headers, _) = get_headers(request).ok_or(ParseError::InvalidRequest)?;

    if protocol != "HTTP/1.1" {
      return Err(ParseError::InvalidProtocol);
    }

    let method: Method = method.parse()?;
    let mut query_string = None;

    if let Some(i) = path.find('?') {
      query_string = Some(QueryString::from(&path[i + 1..]));
      path = &path[..i];
    }

    Ok(Self {
      path,
      query_string,
      headers,
      method,
    })
  }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
  for (i, c) in request.chars().enumerate() {
    if c == ' ' || c == '\r' {
      return Some((&request[..i], &request[i + 1..]));
    }
  }

  None
}

fn get_headers(request: &str) -> Option<(Header, &str)> {
  let index = match request.find("\r\n\r\n") {
    Some(n) => n,
    _ => return None,
  };

  let headers: &str = &request[1..index]; // removing the first one because it's \n
  let rest_of_request = &request[index + 1..];

  let headers = Header::from(headers);

  Some((headers, rest_of_request))
}

pub enum ParseError {
  InvalidRequest,
  InvalidEncoding,
  InvalidProtocol,
  InvalidMethod,
}

impl ParseError {
  fn message(&self) -> &str {
    match self {
      Self::InvalidRequest => "Invalid Request",
      Self::InvalidEncoding => "Invalid Encoding",
      Self::InvalidProtocol => "Invalid Protocol",
      Self::InvalidMethod => "Invalid Method",
    }
  }
}

impl From<MethodError> for ParseError {
  fn from(_: MethodError) -> Self {
    Self::InvalidMethod
  }
}

impl From<Utf8Error> for ParseError {
  fn from(_: Utf8Error) -> Self {
    Self::InvalidEncoding
  }
}

impl Display for ParseError {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{}", self.message())
  }
}

impl Debug for ParseError {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{}", self.message())
  }
}

impl Error for ParseError {}
