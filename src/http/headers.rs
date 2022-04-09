use std::collections::HashMap;

#[derive(Debug)]
pub struct Header<'a> {
  data: HashMap<&'a str, &'a str>,
}

impl<'a> From<&'a str> for Header<'a> {
  fn from(s: &'a str) -> Self {
    let mut data = HashMap::new();
    let headers = s.split("\r\n");
    for header in headers {
      let delimiter_index = header.chars().position(|c| c == ':').unwrap();
      let key = &header[..delimiter_index];
      let value = if header.as_bytes()[delimiter_index + 1] == ' ' as u8 {
        &header[delimiter_index + 2..]
      } else {
        &header[delimiter_index + 1..]
      };

      data
        .entry(key)
        .and_modify(|string| {
          *string = value;
        })
        .or_insert(value);
    }
    Self { data }
  }
}
