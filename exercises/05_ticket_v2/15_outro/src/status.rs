// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `Status` enum.
//  The parsing should be case-insensitive.

#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug, thiserror::Error)]
#[error("`{invalid_status} is not a vaild status`")]
pub struct InvalidStausError {
    invalid_status: String
}

impl TryFrom<String> for Status {
    type Error = InvalidStausError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let v = value.to_lowercase();
        match v.as_str() {
            "done" => return Ok(Status::Done),
            "todo" => return Ok(Status::ToDo),
            "inprogress" => return Ok(Status::InProgress),
            _ => return Err(InvalidStausError{invalid_status: String::from(v)})
        }
    }
}

impl TryFrom<&str> for Status {
    type Error = InvalidStausError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let v = value.to_lowercase();
        match v.as_str() {
            "done" => return Ok(Status::Done),
            "todo" => return Ok(Status::ToDo),
            "inprogress" => return Ok(Status::InProgress),
            _ => return Err(InvalidStausError{invalid_status: String::from(v)})
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("ToDO").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done").unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_invalid() {
        let status = Status::try_from("Invalid");
        assert!(status.is_err());
    }
}
