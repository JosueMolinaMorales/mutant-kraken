use std::{fmt, str::FromStr};

use rand::seq::SliceRandom;

use crate::error::{MutantKrakenError, Result};

// TODO: Add more exceptions, and move to a separate file
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum KotlinExceptions {
    ArithmArithmeticException,
    NullPointerException,
    IllegalArgumentException,
    IllegalStateException,
    IndexOutOfBoundsException,
    NoSuchElementException,
    UnsupportedOperationException,
}

impl fmt::Display for KotlinExceptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KotlinExceptions::ArithmArithmeticException => write!(f, "ArithmeticException"),
            KotlinExceptions::NullPointerException => write!(f, "NullPointerException"),
            KotlinExceptions::IllegalArgumentException => write!(f, "IllegalArgumentException"),
            KotlinExceptions::IllegalStateException => write!(f, "IllegalStateException"),
            KotlinExceptions::IndexOutOfBoundsException => write!(f, "IndexOutOfBoundsException"),
            KotlinExceptions::NoSuchElementException => write!(f, "NoSuchElementException"),
            KotlinExceptions::UnsupportedOperationException => {
                write!(f, "UnsupportedOperationException")
            }
        }
    }
}

impl FromStr for KotlinExceptions {
    type Err = MutantKrakenError;

    fn from_str(s: &str) -> Result<Self> {
        let res = match s {
            "ArithmeticException" => KotlinExceptions::ArithmArithmeticException,
            "NullPointerException" => KotlinExceptions::NullPointerException,
            "IllegalArgumentException" => KotlinExceptions::IllegalArgumentException,
            "IllegalStateException" => KotlinExceptions::IllegalStateException,
            "IndexOutOfBoundsException" => KotlinExceptions::IndexOutOfBoundsException,
            "NoSuchElementException" => KotlinExceptions::NoSuchElementException,
            "UnsupportedOperationException" => KotlinExceptions::UnsupportedOperationException,
            _ => return Err(MutantKrakenError::ConversionError),
        };
        Ok(res)
    }
}

impl KotlinExceptions {
    pub fn get_all_exceptions() -> Vec<KotlinExceptions> {
        vec![
            KotlinExceptions::ArithmArithmeticException,
            KotlinExceptions::NullPointerException,
            KotlinExceptions::IllegalArgumentException,
            KotlinExceptions::IllegalStateException,
            KotlinExceptions::IndexOutOfBoundsException,
            KotlinExceptions::NoSuchElementException,
            KotlinExceptions::UnsupportedOperationException,
        ]
    }

    pub fn get_random_exception(&self) -> KotlinExceptions {
        let mut rng = rand::thread_rng();
        let exceptions = KotlinExceptions::get_all_exceptions();
        let mut rnd = self;
        while rnd == self {
            rnd = exceptions.choose(&mut rng).unwrap();
        }

        *rnd
    }
}

mutant_kraken_macros::generate_kotlin_types_enum!();

#[cfg(test)]
mod tests {
    use crate::kotlin_types::NON_NAMED_TYPES;

    use super::KotlinTypes;

    #[test]
    fn should_successfully_convert_kotlin_types() {
        let res = KotlinTypes::new("value_argument").unwrap();
        assert!(res == KotlinTypes::ValueArgument);
    }

    #[test]
    fn should_return_an_err_for_converting_kotlin_types() {
        let res = KotlinTypes::new("Not_valid_Type");
        assert!(res.is_err());
    }

    #[test]
    fn should_successfully_convert_kotlin_types_to_string() {
        let res = KotlinTypes::ValueArgument.as_str();
        assert!(res == "value_argument");
    }

    #[test]
    fn should_successfully_convert_non_named_type() {
        let non_named_type = NON_NAMED_TYPES[0];
        let res = KotlinTypes::new(non_named_type).unwrap();
        assert!(res == KotlinTypes::NonNamedType(non_named_type.to_string()));
    }

    #[test]
    fn should_successfully_convert_non_named_type_to_string() {
        let non_named_type = NON_NAMED_TYPES[0];
        let res = KotlinTypes::NonNamedType(non_named_type.to_string()).as_str();
        assert!(res == *non_named_type);
    }
}
