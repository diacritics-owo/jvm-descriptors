use crate::class::Class;
use std::fmt::{Display, Write};

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
    assert_eq!(
      Type::Array(Box::new(Type::Array(Box::new(Type::Int)))).to_string(),
      "[[I"
    );
    assert_eq!(
      Type::Array(Box::new(Type::Array(Box::new(Type::Array(Box::new(
        Type::Int
      ))))))
      .to_string(),
      "[[[I"
    );
  }
}
