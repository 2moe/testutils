use crate::os_cmd::{MiniStr, presets::cargo_build::flags::try_into_mini_arg};

#[derive(Debug, Clone)]
/// rustc --print relocation-models
pub enum RelocationModel {
  Static,
  Pic,
  Pie,
  DynamicNoPic,
  Ropi,
  Rwpi,
  RopiRwpi,
  Default,
  Ignore,
}

impl RelocationModel {
  /// Converts RelocationModel as `&str`
  ///
  /// ```ignore
  /// {
  ///   Static => "static",
  ///   Pic => "pic",
  ///   Pie => "pie",
  ///   DynamicNoPic => "dynamic-no-pic",
  ///   Ropi => "ropi",
  ///   Rwpi => "rwpi",
  ///   RopiRwpi => "ropi-rwpi",
  ///   Default => "default",
  ///   Ignore => "",
  /// }
  /// ```
  pub const fn as_str(&self) -> &str {
    use RelocationModel::*;
    match self {
      Static => "static",
      Pic => "pic",
      Pie => "pie",
      DynamicNoPic => "dynamic-no-pic",
      Ropi => "ropi",
      Rwpi => "rwpi",
      RopiRwpi => "ropi-rwpi",
      Default => "default",
      Ignore => "",
    }
  }
}

impl AsRef<str> for RelocationModel {
  fn as_ref(&self) -> &str {
    self.as_str()
  }
}

impl From<RelocationModel> for Option<MiniStr> {
  fn from(model: RelocationModel) -> Self {
    try_into_mini_arg("relocation-model", model)
  }
}

impl Default for RelocationModel {
  /// Default: Ignore
  fn default() -> Self {
    Self::Ignore
  }
}
