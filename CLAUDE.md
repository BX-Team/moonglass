# moonglass

BX Team's design system for [gpui](https://github.com/zed-industries/zed) apps: theme, components and motion on `gpui-pre` (pinned with `=`) and `gpui-base`. It also re-exports the platform crates, so apps get `application`, `init` and `open_window` from here. Used by Riven and Nyx.

## Architecture

| Module | Responsibility |
| ------ | -------------- |
| `theme` | The `Palette` tokens, the built-in themes, `ActiveTheme`. |
| `motion` | Enter, highlight and fade animations; honors reduced motion. |
| `icon`, `assets` | `IconName` and the embedded SVGs in `assets/icons`. |
| `chrome` | Title bar and window controls. |
| `button`, `field`, `menu`, `context_menu`, `dialog`, `tabs`, `nav`, `chip`, `switch`, `section`, `toast`, `copy`, `code` | One component each. |

- Nothing app-specific: no routes, state or `rust_i18n`. Words the components show come from `Labels` (`set_labels`) or from the caller.
- Colors only from `cx.theme().colors` and `theme::danger()`; no hex outside `theme.rs`.
- Every icon in `assets/icons` has an `IconName` variant and an `icons!` entry.
- Animations go through `motion`.

## Commands

```bash
nix develop --command cargo check
cargo fmt
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
```

CI runs `cargo fmt --all --check`, clippy and the tests on Linux, Windows and macOS.

## Working on Tasks

- If the message is a question, investigate and answer only. Edit files only when asked to fix or implement.
- Stay within the scope of the request. Mention related problems you notice instead of fixing them.
- Fix the cause, not the symptom. No special-case constants or hardcoded lists to make one case work.
- When a change can't be verified without running the app, say so.
- Trust the user's reports about what they observed in the running app.
- Don't edit `CLAUDE.md` or `AGENTS.md` unless asked. Decisions, plans and progress don't belong in them.

## Code Guidelines

### Comments

- NO file-header banners and NO divider comments (`// --- helpers ---`).
- Avoid inline comments. Add one only where the code is genuinely non-obvious: a footgun, a wire-format quirk, a reason a thing is done backwards. One line, two at most.
- Why a thing is done this way, or the trap it avoids, is that one short comment at the code, not a paragraph and not an entry here.
- Doc comments on public items are fine; keep them to a sentence.

### Style

- rustfmt is the source of truth — never hand-format against it.
- Match the builder idiom of the component you are editing.

### User-facing strings

None of its own: English defaults in `Labels`, which apps replace with their translations.

### Testing

Tests live next to the code in `#[cfg(test)]` modules. One test per real trap, not one per function.

## Bash Guidelines

- Don't pipe output through `head`/`tail`/`less` to truncate — use tool-native flags (`git log -n 10`, `cargo clippy --message-format=short`). Read the full output.
- Don't create scratch files (scripts, notes) unless asked.
- When given failures, just fix them — don't argue about who introduced them.
