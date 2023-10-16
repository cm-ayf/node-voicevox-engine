use crate::opaque;
use crate::open_jtalk::OpenJtalk;
use crate::util;
use crate::voice_model::SpeakerMeta;
use crate::voice_model::VoiceModel;
use napi::bindgen_prelude::*;
use napi::tokio::sync::Mutex;
use napi_derive::napi;
use std::ops::Deref;
use std::sync::Arc;
use voicevox_core::StyleId;
use voicevox_core::VoiceModelId;

opaque! {Synthesizer, Arc<Mutex<voicevox_core::Synthesizer>>}

#[napi]
impl Synthesizer {
  #[inline]
  async fn new_with_initialize_inner(
    open_jtalk: Arc<voicevox_core::OpenJtalk>,
    options: InitializeOptions,
  ) -> Result<Self> {
    match voicevox_core::Synthesizer::new_with_initialize(open_jtalk, &options.into()).await {
      Ok(v) => Ok(Self(Arc::new(Mutex::new(v)))),
      Err(e) => Err(util::to_napi_error(e)),
    }
  }

  #[napi]
  pub async fn new(open_jtalk_dict_dir: String, options: InitializeOptions) -> Result<Synthesizer> {
    let open_jtalk = OpenJtalk::new(open_jtalk_dict_dir)?;
    Self::new_with_initialize_inner(open_jtalk.into(), options).await
  }

  #[napi]
  pub async fn new_with_initialize(
    open_jtalk: &OpenJtalk,
    options: InitializeOptions,
  ) -> Result<Synthesizer> {
    Self::new_with_initialize_inner(open_jtalk.clone().into(), options).await
  }

  #[napi]
  pub fn is_gpu_mode(&self) -> Result<bool> {
    let synthesizer = self.0.blocking_lock();
    Ok(synthesizer.is_gpu_mode())
  }

  #[napi]
  pub fn metas(&self) -> Result<Vec<SpeakerMeta>> {
    let synthesizer = self.0.blocking_lock();
    Ok(util::map_into(synthesizer.metas()))
  }

  #[napi]
  pub async fn load_voice_model(&self, model: &VoiceModel) -> Result<()> {
    let synthesizer = self.0.lock().await;
    match synthesizer.load_voice_model(model.into()).await {
      Ok(v) => Ok(v.into()),
      Err(e) => Err(util::to_napi_error(e)),
    }
  }

  #[napi]
  pub async fn unload_voice_model(&self, voice_model_id: String) -> Result<()> {
    let synthesizer = self.0.lock().await;
    synthesizer
      .unload_voice_model(&VoiceModelId::new(voice_model_id))
      .map_err(util::to_napi_error)
  }

  #[napi]
  pub async fn is_loaded_voice_model(&self, voice_model_id: String) -> Result<bool> {
    let synthesizer = self.0.lock().await;
    Ok(synthesizer.is_loaded_voice_model(&VoiceModelId::new(voice_model_id)))
  }

  #[napi]
  pub async fn audio_query_from_kana(
    &self,
    kana: String,
    style_id: u32,
  ) -> Result<AudioQueryModel> {
    let synthesizer = self.0.lock().await;
    match synthesizer
      .audio_query_from_kana(&kana, StyleId::new(style_id))
      .await
    {
      Ok(model) => Ok(model.into()),
      Err(e) => Err(util::to_napi_error(e)),
    }
  }

  #[napi]
  pub async fn audio_query(&self, text: String, style_id: u32) -> Result<AudioQueryModel> {
    let synthesizer = self.0.lock().await;
    match synthesizer.audio_query(&text, StyleId::new(style_id)).await {
      Ok(model) => Ok(model.into()),
      Err(e) => Err(util::to_napi_error(e)),
    }
  }

  #[napi]
  pub async fn create_accent_phrases_from_kana(
    &self,
    kana: String,
    style_id: u32,
  ) -> Result<AccentPhrases> {
    let synthesizer = self.0.lock().await;
    match synthesizer
      .create_accent_phrases_from_kana(&kana, StyleId::new(style_id))
      .await
    {
      Ok(phrases) => Ok(phrases.into()),
      Err(e) => Err(util::to_napi_error(e)),
    }
  }

  #[napi]
  pub async fn create_accent_phrases(&self, text: String, style_id: u32) -> Result<AccentPhrases> {
    let synthesizer = self.0.lock().await;
    match synthesizer
      .create_accent_phrases(&text, StyleId::new(style_id))
      .await
    {
      Ok(phrases) => Ok(phrases.into()),
      Err(e) => Err(util::to_napi_error(e)),
    }
  }

  #[napi]
  pub async fn replace_mora_data(
    &self,
    accent_phrases: &AccentPhrases,
    style_id: u32,
  ) -> Result<AccentPhrases> {
    let synthesizer = self.0.lock().await;
    match synthesizer
      .replace_mora_data(accent_phrases, StyleId::new(style_id))
      .await
    {
      Ok(phrases) => Ok(phrases.into()),
      Err(e) => Err(util::to_napi_error(e)),
    }
  }

  #[napi]
  pub async fn replace_phoneme_length(
    &self,
    accent_phrases: &AccentPhrases,
    style_id: u32,
  ) -> Result<AccentPhrases> {
    let synthesizer = self.0.lock().await;
    match synthesizer
      .replace_phoneme_length(accent_phrases, StyleId::new(style_id))
      .await
    {
      Ok(phrases) => Ok(phrases.into()),
      Err(e) => Err(util::to_napi_error(e)),
    }
  }

  #[napi]
  pub async fn replace_mora_pitch(
    &self,
    accent_phrases: &AccentPhrases,
    style_id: u32,
  ) -> Result<AccentPhrases> {
    let accent_phrases = accent_phrases.clone();
    let synthesizer = self.0.lock().await;
    match synthesizer
      .replace_mora_pitch(&Vec::from(accent_phrases), StyleId::new(style_id))
      .await
    {
      Ok(phrases) => Ok(phrases.into()),
      Err(e) => Err(util::to_napi_error(e)),
    }
  }

  #[napi]
  pub async fn synthesis(
    &self,
    audio_query: &AudioQueryModel,
    style_id: u32,
    options: Option<&TtsOptions>,
  ) -> Result<Uint8Array> {
    let options = options
      .map(Into::<voicevox_core::TtsOptions>::into)
      .unwrap_or_default();
    let audio_query = audio_query.clone();
    let synthesizer = self.0.lock().await;
    match synthesizer
      .synthesis(
        &audio_query.into(),
        StyleId::new(style_id),
        &(&options).into(),
      )
      .await
    {
      Ok(wav) => Ok(wav.into()),
      Err(e) => Err(util::to_napi_error(e)),
    }
  }

  #[napi]
  pub async fn tts_from_kana(
    &self,
    kana: String,
    style_id: u32,
    options: Option<&TtsOptions>,
  ) -> Result<Uint8Array> {
    let options = options
      .map(Into::<voicevox_core::TtsOptions>::into)
      .unwrap_or_default();
    let synthesizer = self.0.lock().await;
    match synthesizer
      .tts_from_kana(&kana, StyleId::new(style_id), &options)
      .await
    {
      Ok(wav) => Ok(wav.into()),
      Err(e) => Err(util::to_napi_error(e)),
    }
  }

  #[napi]
  pub async fn tts(
    &self,
    text: String,
    style_id: u32,
    options: Option<&TtsOptions>,
  ) -> Result<Uint8Array> {
    let options = options
      .map(Into::<voicevox_core::TtsOptions>::into)
      .unwrap_or_default();
    let synthesizer = self.0.lock().await;
    match synthesizer
      .tts_from_kana(&text, StyleId::new(style_id), &options)
      .await
    {
      Ok(wav) => Ok(wav.into()),
      Err(e) => Err(util::to_napi_error(e)),
    }
  }
}

#[napi(object)]
pub struct InitializeOptions {
  pub acceleration_mode: AccelerationMode,
  pub cpu_num_threads: u32,
}

impl From<InitializeOptions> for voicevox_core::InitializeOptions {
  fn from(value: InitializeOptions) -> Self {
    let InitializeOptions {
      acceleration_mode,
      cpu_num_threads,
    } = value;
    Self {
      acceleration_mode: acceleration_mode.into(),
      cpu_num_threads: cpu_num_threads as u16,
    }
  }
}

#[napi]
#[derive(Default)]
pub enum AccelerationMode {
  #[default]
  Auto,
  Cpu,
  Gpu,
}

impl From<AccelerationMode> for voicevox_core::AccelerationMode {
  fn from(value: AccelerationMode) -> Self {
    match value {
      AccelerationMode::Auto => Self::Auto,
      AccelerationMode::Cpu => Self::Cpu,
      AccelerationMode::Gpu => Self::Gpu,
    }
  }
}

#[napi]
pub struct TtsOptions {
  pub enable_interrogative_upspeak: bool,
}

impl From<&TtsOptions> for voicevox_core::TtsOptions {
  fn from(value: &TtsOptions) -> Self {
    Self {
      enable_interrogative_upspeak: value.enable_interrogative_upspeak,
    }
  }
}

opaque! {AudioQueryModel}
opaque! {AccentPhrases, Vec<voicevox_core::AccentPhraseModel>}
impl Deref for AccentPhrases {
  type Target = [voicevox_core::AccentPhraseModel];

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
