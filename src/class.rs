use chumsky::prelude::*;
use std::{
  fmt::{Display, Write},
  str::FromStr,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Class {
  pub path: Vec<String>,
  pub subclasses: Vec<String>,
}

impl Display for Class {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let path = self.path.join("/");
    path.fmt(f)?;

    for subclass in self.subclasses.clone() {
      f.write_char('$')?;
      subclass.fmt(f)?;
    }

    Ok(())
  }
}

impl FromStr for Class {
  type Err = Vec<Simple<char>>;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Self::parser().parse(s)
  }
}

impl<'a> Class {
  pub fn parser() -> impl Parser<char, Self, Error = Simple<char>> {
    text::ident::<char, Simple<char>>()
      .separated_by(just('/'))
      .then(just('$').ignore_then(text::ident()).repeated())
      .map(|(path, subclasses)| Class { path, subclasses })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn fmt() {
    assert_eq!(
      Class {
        path: vec!["java".to_string(), "lang".to_string(), "Object".to_string()],
        subclasses: vec![],
      }
      .to_string(),
      "java/lang/Object"
    );

    assert_eq!(
      Class {
        path: vec!["com".to_string(), "example".to_string(), "Foo".to_string()],
        subclasses: vec!["Bar".to_string(), "Baz".to_string()],
      }
      .to_string(),
      "com/example/Foo$Bar$Baz"
    );
  }

  #[test]
  fn parse() {
    assert_eq!(
      "java/lang/Object".parse(),
      Ok(Class {
        path: vec!["java".to_string(), "lang".to_string(), "Object".to_string()],
        subclasses: vec![],
      })
    );

    assert_eq!(
      "com/example/Foo$Bar$Baz".parse(),
      Ok(Class {
        path: vec!["com".to_string(), "example".to_string(), "Foo".to_string()],
        subclasses: vec!["Bar".to_string(), "Baz".to_string()],
      })
    );
  }
}
