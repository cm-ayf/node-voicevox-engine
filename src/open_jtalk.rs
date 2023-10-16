use std::sync::Arc;

use napi::bindgen_prelude::*;
use napi_derive::napi;

use crate::opaque;
use crate::user_dict::UserDict;
use crate::util;

opaque! {OpenJtalk, Arc<voicevox_core::OpenJtalk>}

#[napi]
impl OpenJtalk {
  #[napi(constructor)]
  pub fn new(open_jtalk_dict_dir: String) -> Result<Self> {
    let open_jtalk = voicevox_core::OpenJtalk::new_with_initialize(open_jtalk_dict_dir)
      .map_err(util::to_napi_error)?;
    Ok(Self(Arc::new(open_jtalk)))
  }

  #[napi]
  pub fn use_user_dict(&self, user_dict: &UserDict) -> Result<()> {
    self
      .0
      .use_user_dict(user_dict.into())
      .map_err(util::to_napi_error)?;

    Ok(())
  }
}
