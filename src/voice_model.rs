use napi::bindgen_prelude::*;
use napi_derive::napi;

use crate::opaque;
use crate::util;

opaque! {VoiceModel}

#[napi]
impl VoiceModel {
  #[napi]
  pub async fn from_path(path: String) -> Result<VoiceModel> {
    match voicevox_core::VoiceModel::from_path(path).await {
      Ok(v) => Ok(v.into()),
      Err(e) => Err(util::to_napi_error(e)),
    }
  }

  #[napi(getter)]
  pub fn id(&self) -> &str {
    self.0.id().raw_voice_model_id()
  }

  #[napi(getter)]
  pub fn metas(&self) -> Vec<SpeakerMeta> {
    util::map_clone_into(self.0.metas())
  }
}

opaque! {SpeakerMeta}

#[napi]
impl SpeakerMeta {
  #[napi(getter)]
  pub fn name(&self) -> &str {
    self.0.name()
  }

  #[napi(getter)]
  pub fn styles(&self) -> Vec<StyleMeta> {
    util::map_clone_into(self.0.styles())
  }

  #[napi(getter)]
  pub fn version(&self) -> &str {
    self.0.version().raw_version()
  }

  #[napi(getter)]
  pub fn speaker_uuid(&self) -> &str {
    self.0.speaker_uuid()
  }
}

opaque! {StyleMeta}
#[napi]
impl StyleMeta {
  #[napi(getter)]
  pub fn id(&self) -> u32 {
    self.0.id().raw_id()
  }

  #[napi(getter)]
  pub fn name(&self) -> &str {
    self.0.name()
  }
}
