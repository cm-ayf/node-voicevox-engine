use napi::bindgen_prelude::*;
use napi_derive::napi;

use crate::opaque;
use crate::util;

opaque! {UserDict}

#[napi]
impl UserDict {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self(Default::default())
  }

  #[napi]
  pub fn load(&mut self, path: String) -> Result<()> {
    self.0.load(&path).map_err(util::to_napi_error)
  }

  #[napi]
  pub fn save(&self, path: String) -> Result<()> {
    self.0.save(&path).map_err(util::to_napi_error)
  }

  #[napi]
  pub fn add_word(&mut self, word: UserDictWord) -> Result<String> {
    let word = word.try_into().map_err(util::to_napi_error)?;
    let uuid = self.0.add_word(word).map_err(util::to_napi_error)?;
    Ok(uuid.to_string())
  }

  #[napi]
  pub fn update_word(&mut self, word_uuid: String, word: UserDictWord) -> Result<()> {
    let word_uuid = word_uuid.parse().map_err(util::to_napi_error)?;
    let word = word.try_into().map_err(util::to_napi_error)?;
    self
      .0
      .update_word(word_uuid, word)
      .map_err(util::to_napi_error)?;
    Ok(())
  }

  #[napi]
  pub fn remove_word(&mut self, word_uuid: String) -> Result<UserDictWord> {
    let word_uuid = word_uuid.parse().map_err(util::to_napi_error)?;
    match self.0.remove_word(word_uuid) {
      Ok(word) => Ok(word.into()),
      Err(e) => Err(util::to_napi_error(e)),
    }
  }

  #[napi]
  pub fn import(&mut self, other: &UserDict) -> Result<()> {
    self.0.import(&other.0).map_err(util::to_napi_error)
  }

  #[napi(getter)]
  pub fn words(&self) -> Result<()> {
    todo!()
  }
}

#[napi(object)]
pub struct UserDictWord {
  /// 単語の表記。
  pub surface: String,
  /// 単語の読み。
  pub pronunciation: String,
  /// アクセント型。
  pub accent_type: u32,
  /// 単語の種類。
  pub word_type: UserDictWordType,
  /// 単語の優先度。
  pub priority: u32,
}

impl TryFrom<UserDictWord> for voicevox_core::UserDictWord {
  type Error = Error;

  fn try_from(value: UserDictWord) -> Result<Self> {
    let UserDictWord {
      surface,
      pronunciation,
      accent_type,
      word_type,
      priority,
    } = value;
    Self::new(
      &surface,
      pronunciation,
      accent_type as usize,
      word_type.into(),
      priority,
    )
    .map_err(util::to_napi_error)
  }
}

impl From<voicevox_core::UserDictWord> for UserDictWord {
  fn from(value: voicevox_core::UserDictWord) -> Self {
    let voicevox_core::UserDictWord {
      surface,
      pronunciation,
      accent_type,
      word_type,
      priority,
      ..
    } = value;
    Self {
      surface,
      pronunciation,
      accent_type: accent_type as u32,
      word_type: word_type.into(),
      priority,
    }
  }
}

#[napi]
pub enum UserDictWordType {
  /// 固有名詞。
  ProperNoun,
  /// 一般名詞。
  CommonNoun,
  /// 動詞。
  Verb,
  /// 形容詞。
  Adjective,
  /// 接尾辞。
  Suffix,
}

impl From<UserDictWordType> for voicevox_core::UserDictWordType {
  fn from(value: UserDictWordType) -> Self {
    match value {
      UserDictWordType::ProperNoun => Self::ProperNoun,
      UserDictWordType::CommonNoun => Self::CommonNoun,
      UserDictWordType::Verb => Self::Verb,
      UserDictWordType::Adjective => Self::Adjective,
      UserDictWordType::Suffix => Self::Suffix,
    }
  }
}

impl From<voicevox_core::UserDictWordType> for UserDictWordType {
  fn from(value: voicevox_core::UserDictWordType) -> Self {
    match value {
      voicevox_core::UserDictWordType::ProperNoun => Self::ProperNoun,
      voicevox_core::UserDictWordType::CommonNoun => Self::CommonNoun,
      voicevox_core::UserDictWordType::Verb => Self::Verb,
      voicevox_core::UserDictWordType::Adjective => Self::Adjective,
      voicevox_core::UserDictWordType::Suffix => Self::Suffix,
    }
  }
}
