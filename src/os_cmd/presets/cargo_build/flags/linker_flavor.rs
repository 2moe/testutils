use crate::os_cmd::{MiniStr, presets::cargo_build::flags::try_into_mini_arg};

#[derive(Debug, Clone)]
/// > From the rustc book: This flag controls the linker flavor used by rustc.
/// > If a linker is given with the -C linker flag, then the linker flavor is
/// > inferred from the value provided. If no linker is given then the linker
/// > flavor is used to determine the linker to use.
/// >Every rustc target defaults to some linker flavor.
pub enum LinkerFlavor {
  EmscriptenEmcc,
  GCC,
  LD,
  MSVC,
  WasmLD,
  DarwinLLVMLLD,
  GNUbinutilsLLVMLLD,
  MSLinkExeLLD,
  Ignore,
}

impl LinkerFlavor {
  /// Converts LinkerFlavor as `&str`
  pub const fn as_str(&self) -> &str {
    use LinkerFlavor::*;
    match self {
      EmscriptenEmcc => "em",
      GCC => "gcc",
      LD => "ld",
      MSVC => "msvc",
      WasmLD => "wasm-ld",
      DarwinLLVMLLD => "ld64.link_self_contained",
      GNUbinutilsLLVMLLD => "ld.lld",
      MSLinkExeLLD => "lld-link",
      Ignore => "",
    }
  }
}

impl AsRef<str> for LinkerFlavor {
  fn as_ref(&self) -> &str {
    self.as_str()
  }
}

impl From<LinkerFlavor> for Option<MiniStr> {
  fn from(value: LinkerFlavor) -> Self {
    try_into_mini_arg("linker-flavor", value)
  }
}

impl Default for LinkerFlavor {
  /// Default: Ignore
  fn default() -> Self {
    Self::Ignore
  }
}
