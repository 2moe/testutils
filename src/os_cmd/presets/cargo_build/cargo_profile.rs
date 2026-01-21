use crate::os_cmd::MiniStr;

#[derive(Debug, Clone, PartialEq, Eq)]
/// cargo profile name: e.g., build, dev
#[derive(Default)]
pub enum CargoProfile {
  /// "release"
  #[default]
  Release,
  /// "dev"
  Debug,
  Custom(MiniStr),
}

impl core::fmt::Display for CargoProfile {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str(self.as_str())
  }
}

impl From<&str> for CargoProfile {
  fn from(value: &str) -> Self {
    match value {
      "debug" | "dev" => Self::Debug,
      "release" => Self::Release,
      _ => Self::Custom(value.into()),
    }
  }
}

impl CargoProfile {
  /// Converts CargoProfile as `&str`
  pub fn as_str(&self) -> &str {
    match self {
      Self::Debug => "dev",
      Self::Release => "release",
      Self::Custom(s) => s.as_ref(),
    }
  }
}
impl AsRef<str> for CargoProfile {
  fn as_ref(&self) -> &str {
    self.as_str()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[ignore]
  #[test]
  fn test_cargo_profile_dev() {
    let profile: CargoProfile = CargoProfile::Debug;
    assert_eq!(profile, "debug".into());
    assert_eq!(profile, "dev".into());
    assert_eq!(profile.as_str(), "dev");
  }
}
