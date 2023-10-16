macro_rules! opaque {
  ($id:ident) => {
    opaque!($id, voicevox_core::$id);
  };
  ($id:ident, $t:ty) => {
    #[napi]
    #[derive(Clone)]
    pub struct $id($t);

    impl From<$id> for $t {
      fn from(value: $id) -> Self {
        value.0
      }
    }

    impl<'a> From<&'a $id> for &'a $t {
      fn from(value: &'a $id) -> Self {
        &value.0
      }
    }

    impl From<$t> for $id {
      fn from(value: $t) -> Self {
        Self(value)
      }
    }
  };
}

pub(crate) use opaque;
