use crate::os_cmd::{MiniStr, presets::cargo_build::flags::try_into_mini_arg};

#[derive(Debug, Clone)]
/// rustc --print code-models
///
/// From the rustc book:
///
/// > Code models put constraints on address ranges that the program and its
/// > symbols may use.
/// >
/// > With smaller address ranges machine instructions may be
/// > able to use more compact addressing modes.
pub enum CodeModel {
  Tiny,
  Small,
  Kernel,
  Medium,
  Large,
  Ignore,
}

impl From<&str> for CodeModel {
  fn from(value: &str) -> Self {
    use CodeModel::*;
    match value {
      "tiny" => Tiny,
      "small" => Small,
      "kernel" => Kernel,
      "medium" => Medium,
      "large" => Large,
      _ => Ignore,
    }
  }
}

impl CodeModel {
  /// Converts CodeModel as `&str`
  pub const fn as_str(&self) -> &str {
    use CodeModel::*;
    match self {
      Tiny => "tiny",
      Small => "small",
      Kernel => "kernel",
      Medium => "medium",
      Large => "large",
      Ignore => "",
    }
  }
}

impl AsRef<str> for CodeModel {
  fn as_ref(&self) -> &str {
    self.as_str()
  }
}

impl From<CodeModel> for Option<MiniStr> {
  fn from(model: CodeModel) -> Self {
    try_into_mini_arg("code-model", model)
  }
}

impl Default for CodeModel {
  /// Default: Ignore
  fn default() -> Self {
    Self::Ignore
  }
}
