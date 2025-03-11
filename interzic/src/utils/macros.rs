/// If the expresion is `Ok`, early return it. If the value is `Err`, continue with the value.
///
/// This is the inverse of the `?` operator.
#[macro_export]
macro_rules! try_err {
    ($result: expr) => {
        match $result {
            Ok(val) => return Ok(val),
            Err(err) => err,
        }
    };
}
