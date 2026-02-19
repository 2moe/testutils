# ChangeLog

## 0.0.9 (2026-02-19)

- add `const_macros` feature

## 0.0.7 (2026-02-11)

- CommandSpawner:
  - add environment variables and working directory support

Breaking changes:

- RunnableCommand Trait:
  - remove `capture_*()`

## 0.0.6 (2026-01-21)

- CommandRepr:
  - add `.into_tinyvec()`
- os_cmd:
  - add `struct DecodedText`
  - add `struct CommandSpawner`, `enum StdioMode`

Breaking changes:

- CargoCmd:
  - other_args => extra_args
- Runner:
  - add field **inspect_mode**
  - `eprint_cmd` => RunnerInspection::Stderr
  - `log_dbg_cmd` => RunnerInspection::LogDebug
- remove tiny_container
- update traits::BoolExt
  (To avoid conflict with the standard library (rust-lang/rust#142748), the function name has been changed.)
  - `.ok_or_else` => `.then_ok_or_else`
  - add `.ok_or`
- update CargoDoc
  - custom_cfg:
    - **"__unstable_doc"** => **"docsrs"**

## 0.0.5

- feat: add impl `<&str>` for SubCmd

## 0.0.4

- feat(CargoCmd.profile): MiniStr -> CargoProfile
- add `CargoProfile`

```rust
pub enum CargoProfile {
  Release, // "release"
  Debug, // "dev"
  Custom(MiniStr),
}
```

## 0.0.3

- fix(CargoCmd field name): `all_workspaces` -> `all_packages`

```rust
// all_packages: bool
all_packages.then(|| "--workspace".into())
```

## 0.0.2 (Deprecated)

- feat(macro): add `dbg!`
- feat(trait): add RunnableCommand

Breaking Changes:

- CargoBuild -> CargoCmd
