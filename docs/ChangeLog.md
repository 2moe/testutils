# ChangeLog

## 0.0.2

- feat(CargoCmd): add `all_workspaces` field

```rust
// all_workspaces: bool
all_workspaces.then(|| "--workspace".into())
```

- feat(macro): add `dbg!`
- feat(trait): add RunnableCommand

Breaking Changes:

- CargoBuild -> CargoCmd
