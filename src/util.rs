use napi::bindgen_prelude::*;

pub fn to_napi_error(e: impl std::error::Error) -> Error {
  napi::Error::new(Status::GenericFailure, e)
}

pub fn map_into<T, U: Clone + Into<T>, F: FromIterator<T>>(iter: impl IntoIterator<Item = U>) -> F {
  iter.into_iter().map(|i| i.into()).collect()
}

pub fn map_clone_into<'a, T, U: Clone + Into<T> + 'a, F: FromIterator<T>>(
  iter: impl IntoIterator<Item = &'a U>,
) -> F {
  iter.into_iter().map(|i| i.clone().into()).collect()
}
