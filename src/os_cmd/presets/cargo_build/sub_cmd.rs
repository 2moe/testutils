use crate::os_cmd::MiniStr;

#[derive(Debug, Clone)]
/// cargo sub command: e.g., build, run
pub enum SubCmd {
  Build,
  Run,
  Test,
  Bench,
  Check,
  Rustc,
  Custom(MiniStr),
}

impl From<&str> for SubCmd {
  fn from(value: &str) -> Self {
    use SubCmd::*;
    match value {
      "build" => Build,
      "run" => Run,
      "test" => Test,
      "bench" => Bench,
      "check" => Check,
      "rustc" => Rustc,
      v => Self::Custom(v.into()),
    }
  }
}

impl SubCmd {
  const fn ignore_custom_as_str(&self) -> &str {
    use SubCmd::*;

    match self {
      Build => "build",
      Run => "run",
      Test => "test",
      Bench => "bench",
      Check => "check",
      Rustc => "rustc",
      _ => "",
    }
  }
  /// Converts SubCmd as `&str`
  pub fn as_str(&self) -> &str {
    match self {
      Self::Custom(s) => s.as_ref(),
      _ => self.ignore_custom_as_str(),
    }
  }
}
impl AsRef<str> for SubCmd {
  fn as_ref(&self) -> &str {
    self.as_str()
  }
}
impl Default for SubCmd {
  fn default() -> Self {
    Self::Build
  }
}
