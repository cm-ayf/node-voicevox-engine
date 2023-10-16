#![deny(clippy::all)]
mod opaque;
mod open_jtalk;
mod synthesizer;
mod user_dict;
mod util;
mod voice_model;

use opaque::opaque;

#[cfg(test)]
mod test {
  use super::*;
  use synthesizer::*;
  use voice_model::*;

  #[tokio::test]
  async fn init() {
    let synthesizer = Synthesizer::new(
      "@discordjs-japan".to_string(),
      InitializeOptions {
        acceleration_mode: AccelerationMode::Auto,
        cpu_num_threads: 1,
      },
    )
    .await
    .unwrap();
    let model = VoiceModel::from_path("@discordjs-japan/model/1.vvm".to_string())
      .await
      .unwrap();
    synthesizer.load_voice_model(&model).await.unwrap();
  }
}
