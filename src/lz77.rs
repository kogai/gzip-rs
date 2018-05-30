#[derive(PartialEq, Debug)]
pub enum Code {
  Character(u8),
  Pair(u16, u32), // (len, pos)
}

pub fn compose(raw: &[u8]) -> Vec<Code> {
  // unimplemented!();
  vec![]
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_compose() {
    let actual = "ABCDEFGABCDEF".to_owned();
    assert_eq!(
      compose(actual.as_bytes()),
      vec![
        Code::Character(65),
        Code::Character(66),
        Code::Character(67),
        Code::Character(68),
        Code::Character(69),
        Code::Character(70),
        Code::Character(71),
        Code::Pair(6, 7),
      ]
    );
  }
}
