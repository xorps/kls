use std::convert::identity;

pub fn flatten<T, E>(result: Result<Result<T, E>, E>) -> Result<T, E> {
    result.and_then(identity)
}
