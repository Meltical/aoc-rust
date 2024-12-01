use std::str::FromStr;

pub trait ParseOps {
    fn int<T>(&self) -> T
    where
        T: FromStr,
        T::Err: std::fmt::Debug;
}

impl ParseOps for &str {
    fn int<T>(&self) -> T
    where
        T: FromStr,
        T::Err: std::fmt::Debug,
    {
        match T::from_str(self) {
            Ok(t) => t,
            Err(e) => panic!("Not a valid number: {}, error: {:?}", self, e),
        }
    }
}
