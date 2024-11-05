use crate::r#type::Type;
use chumsky::prelude::*;
use std::{
  fmt::{Display, Write},
  str::FromStr,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Method {
  Method {
    name: String,
    parameters: Vec<Type>,
    return_type: Option<Type>,
  },
  Constructor {
    parameters: Vec<Type>,
  },
}

impl Display for Method {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Method::Method {
        name,
        parameters,
        return_type,
      } => {
        name.fmt(f)?;
        f.write_char('(')?;
        for parameter in parameters {
          parameter.fmt(f)?;
        }
        f.write_char(')')?;
        match return_type {
          Some(ty) => ty.fmt(f),
          None => f.write_char('V'),
        }?;
      }
      Method::Constructor { parameters } => {
        f.write_str("<init>")?;
        f.write_char('(')?;
        for parameter in parameters {
          parameter.fmt(f)?;
        }
        f.write_char(')')?;
        f.write_char('V')?;
      }
    }

    Ok(())
  }
}

impl FromStr for Method {
  type Err = Vec<Simple<char>>;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Self::parser().parse(s)
  }
}

impl Method {
  pub fn parser() -> impl Parser<char, Self, Error = Simple<char>> {
    text::ident()
      .then(Type::parser().repeated().delimited_by(just('('), just(')')))
      .then(Type::parser().map(Some).or(just('V').to(None)))
      .map(|((name, parameters), return_type)| Method::Method {
        name,
        parameters,
        return_type,
      })
      .or(
        just("<init>")
          .ignore_then(Type::parser().repeated().delimited_by(just('('), just(')')))
          .then_ignore(just('V'))
          .map(|parameters| Method::Constructor { parameters }),
      )
  }
}

#[cfg(test)]
mod tests {
  use crate::class::Class;

  use super::*;

  #[test]
  fn fmt() {
    assert_eq!(
      Method::Method {
        name: "hello".to_string(),
        parameters: vec![Type::Class(Class {
          path: vec!["java".to_string(), "lang".to_string(), "String".to_string()],
          subclasses: vec![]
        })],
        return_type: None,
      }
      .to_string(),
      "hello(Ljava/lang/String;)V"
    );

    assert_eq!(
      Method::Constructor {
        parameters: vec![Type::Class(Class {
          path: vec!["java".to_string(), "lang".to_string(), "String".to_string()],
          subclasses: vec![]
        })],
      }
      .to_string(),
      "<init>(Ljava/lang/String;)V"
    );
  }

  #[test]
  fn parse() {
    assert_eq!(
      "hello(Ljava/lang/String;)V".parse(),
      Ok(
        Method::Method {
          name: "hello".to_string(),
          parameters: vec![Type::Class(Class {
            path: vec!["java".to_string(), "lang".to_string(), "String".to_string()],
            subclasses: vec![]
          })],
          return_type: None,
        }
        .to_string()
      )
    );

    assert_eq!(
      "<init>(Ljava/lang/String;)V".parse(),
      Ok(Method::Constructor {
        parameters: vec![Type::Class(Class {
          path: vec!["java".to_string(), "lang".to_string(), "String".to_string()],
          subclasses: vec![]
        })],
      })
    );
  }
}
