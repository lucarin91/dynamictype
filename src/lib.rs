use std::convert::TryFrom;
use std::fmt::Display;
use std::ops::Add;

#[derive(Debug, Clone)]
pub enum DType {
    Bool(bool),
    Num(i32),
    Str(String),
    NaN,
}

impl From<i32> for DType {
    fn from(n: i32) -> Self {
        DType::Num(n)
    }
}

impl From<bool> for DType {
    fn from(b: bool) -> Self {
        DType::Bool(b)
    }
}

impl From<String> for DType {
    fn from(s: String) -> Self {
        DType::Str(s)
    }
}

impl From<&str> for DType {
    fn from(s: &str) -> Self {
        DType::Str(s.to_owned())
    }
}

impl TryFrom<DType> for i32 {
    type Error = ();

    fn try_from(value: DType) -> Result<Self, Self::Error> {
        match value {
            DType::Bool(b) if b => Ok(1),
            DType::Bool(b) if !b => Ok(0),
            DType::Str(s) => {
                if let Ok(n) = s.parse::<i32>() {
                    Ok(n)
                } else {
                    Err(())
                }
            }
            DType::Num(n) => Ok(n),
            DType::NaN => Err(()),
            _ => unimplemented!(),
        }
    }
}

impl From<DType> for String {
    fn from(value: DType) -> Self {
        match value {
            DType::Bool(b) if b => "1".to_owned(),
            DType::Bool(b) if !b => "0".to_owned(),
            DType::Str(s) => s,
            DType::Num(n) => n.to_string(),
            DType::NaN => "NaN".to_owned(),
            _ => unimplemented!(),
        }
    }
}

impl Add for DType {
    type Output = DType;

    fn add(self, rhs: Self) -> Self::Output {
        // TODO: 1 + "asd" should be "1asd" and not NaN
        match (self, rhs) {
            (x @ DType::Num(_), y @ DType::Num(_))
            | (x @ DType::Num(_), y @ DType::Bool(_))
            | (x @ DType::Bool(_), y @ DType::Bool(_))
            | (x @ DType::Bool(_), y @ DType::Num(_))
            | (x @ DType::Num(_), y @ DType::Str(_)) => {
                let x = i32::try_from(x);
                let y = i32::try_from(y);
                match (x, y) {
                    (Ok(x), Ok(y)) => DType::Num(x + y),
                    _ => DType::NaN,
                }
            }
            (x @ DType::Str(_), y @ DType::Str(_))
            | (x @ DType::Bool(_), y @ DType::Str(_))
            | (x @ DType::Str(_), y @ DType::Bool(_))
            | (x @ DType::Str(_), y @ DType::Num(_)) => {
                let x = String::from(x);
                let y = String::from(y);
                DType::Str(x + &y)
            }
            (DType::NaN, _) | (_, DType::NaN) => DType::NaN,
        }
    }
}

impl Display for DType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DType::Bool(b) if *b => f.write_str("true"),
            DType::Bool(b) if !*b => f.write_str("false"),
            DType::Str(s) => f.write_fmt(format_args!("\"{}\"", s)),
            DType::Num(n) => f.write_fmt(format_args!("{}", n)),
            DType::NaN => f.write_str("NaN"),
            _ => unimplemented!(),
        }
    }
}
