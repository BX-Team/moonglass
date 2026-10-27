# moonglass

BX Team's gpui design system on GPUI Kit (`gpui-kit`, `default-features = false`). Used by Riven and Nyx; anything app-specific (routes, state, i18n) stays in the apps.

## Commands

```bash
nix develop --command cargo check
cargo fmt
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
```

CI runs clippy on Linux, Windows and macOS.

## Rules

- No `rust_i18n` here: words the components show come from `Labels` (`set_labels`) or from the caller.
- Colors only from `cx.theme().colors` (the `Palette`) and `theme::danger()`; no hex outside `theme.rs`.
- Every icon in `assets/icons` has an `IconName` variant and an `icons!` entry; the test checks it.
- Animations go through `motion` so reduced motion and theme fades are honored.
- Comments: no file banners or divider comments; `///` on public items, one line.
