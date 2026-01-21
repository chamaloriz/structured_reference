#[derive(Debug, Clone)]
pub struct StructuredData {
    first_part: u64,
    second_part: u64,
    third_part: u64,
    checksum: u64,
}

#[derive(Debug)]
pub enum StructuredDataError {
    FirstPartOutOfRange(u64),
    SecondPartOutOfRange(u64),
    ThirdPartOutOfRange(u64),
    CodeParsingIssue,
}

impl std::fmt::Display for StructuredDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FirstPartOutOfRange(v) => write!(f, "first_part must be 0-999, got {}", v),
            Self::SecondPartOutOfRange(v) => write!(f, "second_part must be 0-9999, got {}", v),
            Self::ThirdPartOutOfRange(v) => write!(f, "third_part must be 0-999, got {}", v),
            Self::CodeParsingIssue => write!(f, "Failed to parse code"),
        }
    }
}

impl std::error::Error for StructuredDataError {}

impl StructuredData {
    pub fn new(
        first_part: u64,
        second_part: u64,
        third_part: u64,
    ) -> Result<Self, StructuredDataError> {
        if first_part > 999 {
            return Err(StructuredDataError::FirstPartOutOfRange(first_part));
        }

        if second_part > 9999 {
            return Err(StructuredDataError::SecondPartOutOfRange(second_part));
        }

        if third_part > 999 {
            return Err(StructuredDataError::ThirdPartOutOfRange(third_part));
        }

        let code: u64 = format!("{:03}{:04}{:03}", first_part, second_part, third_part)
            .parse()
            .map_err(|_| StructuredDataError::CodeParsingIssue)?;

        let mut checksum = code % 97;

        if checksum == 0 {
            checksum = 97;
        }

        Ok(Self {
            first_part,
            second_part,
            third_part,
            checksum,
        })
    }

    pub fn to_bank_format(&self) -> String {
        format!(
            "+++ {:03}/{:04}/{:03}{:02} +++",
            self.first_part, self.second_part, self.third_part, self.checksum
        )
    }

    pub fn to_digits(&self) -> String {
        format!(
            "{:03}{:04}{:03}{:02}",
            self.first_part, self.second_part, self.third_part, self.checksum
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn various_checksum_values() {
        let test_cases = [
            (10, 1, 26),
            (100, 0, 0),
            (0, 1000, 0),
            (999, 9999, 999),
            (123, 4567, 890),
        ];

        for (a, b, c) in test_cases {
            let data = StructuredData::new(a, b, c).unwrap();
            let digits = data.to_digits();

            let code: u64 = digits[0..10].parse().unwrap();
            let checksum: u64 = digits[10..12].parse().unwrap();

            let expected = if code % 97 == 0 { 97 } else { code % 97 };
            assert_eq!(checksum, expected, "Checksum mismatch for {:?}", (a, b, c));
        }
    }
}

pub mod prelude {
    pub use crate::{StructuredData, StructuredDataError};
}
