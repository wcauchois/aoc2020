use nom::error::ParseError;
use nom::IResult;

/// Take 1 of any item from the input slice. Fails if there's no input.
pub fn take1<'a, T, E>(input: &'a [T]) -> IResult<&'a [T], T, E>
where
    T: 'a + Clone,
    E: ParseError<&'a [T]>,
{
    if input.len() < 1 {
        Err(nom::Err::Error(E::from_error_kind(
            input,
            nom::error::ErrorKind::Eof,
        )))
    } else {
        Ok((&input[1..], input[0].clone()))
    }
}

/// Take 1 item and verify that it matches the predicate.
pub fn take1_verify<'a, T, E, F>(pred: F) -> impl FnMut(&'a [T]) -> IResult<&'a [T], T, E>
where
    F: Fn(&T) -> bool,
    T: 'a + Clone,
    E: ParseError<&'a [T]>,
{
    move |input: &[T]| {
        if input.len() < 1 {
            Err(nom::Err::Error(E::from_error_kind(
                input,
                nom::error::ErrorKind::Eof,
            )))
        } else if pred(&input[0]) {
            Ok((&input[1..], input[0].clone()))
        } else {
            Err(nom::Err::Error(E::from_error_kind(
                input,
                nom::error::ErrorKind::Verify,
            )))
        }
    }
}
