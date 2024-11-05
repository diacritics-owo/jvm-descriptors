use crate::class::Class;
use chumsky::prelude::*;
use std::{
  fmt::{Display, Write},
  str::FromStr,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
  Byte,
  Char,
  Double,
  Float,
  Int,
  Long,
  Class(Class),
  Short,
  Boolean,
  Array(Box<Type>),
}

impl Display for Type {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Type::Class(class) => {
        f.write_char('L')?;
        class.fmt(f)?;
        f.write_char(';')?;
      }
      Type::Array(ty) => {
        f.write_char('[')?;
        ty.fmt(f)?;
      }
      _ => f.write_str(match self {
        Type::Byte => "B",
        Type::Char => "C",
        Type::Double => "D",
        Type::Float => "F",
        Type::Int => "I",
        Type::Long => "I",
        Type::Short => "S",
        Type::Boolean => "Z",
        Type::Class(_) => unreachable!(),
        Type::Array(_) => unreachable!(),
      })?,
    };
    Ok(())
  }
}

impl FromStr for Type {
  type Err = Vec<Simple<char>>;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Self::parser().parse(s)
  }
}

impl Type {
  pub fn parser() -> impl Parser<char, Self, Error = Simple<char>> {
    recursive(|ty| {
      choice((
        just('B').to(Type::Byte),
        just('C').to(Type::Char),
        just('D').to(Type::Double),
        just('F').to(Type::Float),
        just('I').to(Type::Int),
        just('J').to(Type::Long),
        just('S').to(Type::Short),
        just('Z').to(Type::Boolean),
        Class::parser()
          .delimited_by(just('L'), just(';'))
          .map(|class| Type::Class(class)),
        just('[')
          .ignore_then(ty)
          .map(|ty| Type::Array(Box::new(ty))),
      ))
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn fmt() {
    assert_eq!(Type::Byte.to_string(), "B");
    assert_eq!(Type::Char.to_string(), "C");
    assert_eq!(Type::Double.to_string(), "D");
    assert_eq!(Type::Float.to_string(), "F");
    assert_eq!(Type::Int.to_string(), "I");
    assert_eq!(Type::Long.to_string(), "I");
    assert_eq!(Type::Short.to_string(), "S");
    assert_eq!(Type::Boolean.to_string(), "Z");
    assert_eq!(
      Type::Class(Class {
        path: vec!["java".to_string(), "lang".to_string(), "Object".to_string()],
        subclasses: vec![],
      })
      .to_string(),
      "Ljava/lang/Object;"
    );
    assert_eq!(Type::Array(Box::new(Type::Int)).to_string(), "[I");
  }

  #[test]
  fn parse() {
    assert_eq!("B".parse(), Ok(Type::Byte));
    assert_eq!("C".parse(), Ok(Type::Char));
    assert_eq!("D".parse(), Ok(Type::Double));
    assert_eq!("F".parse(), Ok(Type::Float));
    assert_eq!("I".parse(), Ok(Type::Int));
    assert_eq!("J".parse(), Ok(Type::Long));
    assert_eq!("S".parse(), Ok(Type::Short));
    assert_eq!("Z".parse(), Ok(Type::Boolean));
    assert_eq!(
      "Ljava/lang/Object;".parse(),
      Ok(Type::Class(Class {
        path: vec!["java".to_string(), "lang".to_string(), "Object".to_string()],
        subclasses: vec![],
      }))
    );
    assert_eq!("[I".parse(), Ok(Type::Array(Box::new(Type::Int))));
  }
}
