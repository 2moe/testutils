# ChangeLog

## 0.0.4 (Upcoming)

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
