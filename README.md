# moonglass

BX Team's design system for [gpui](https://github.com/zed-industries/zed). The look is its own;
behavior that is hard to get right — text input with IME, popover placement, focus traps, scrollbars —
comes from [GPUI Base](https://github.com/longbridge/gpui-kit), which has no look of its own.
It is the look shared by [Riven](https://github.com/BX-Team/Riven) and [Nyx](https://github.com/BX-Team/Nyx).

- `theme`: a twelve-color `Palette`, six built-in themes (Graphite, Night, Moss, Ember, Day, Paper), app-registered themes, light/dark/system mode with a 260 ms fade.
- `TitleBar` and resize edges for windows that draw their own frame.
- Components: `Button`, `Tabs`, `Switch`, `TextField`, `TextArea`, `Dropdown`, `ActionMenu`, context menus, `Modal`, `Section` + `setting_row`, `nav_row`, `placeholder`, `copy_button`, toasts.
- `code`: a small lexer for TOML/JSON/YAML and a themed `CodeEditor`.
- `motion`: hover, color and position transitions, entrances, a gliding selection highlight; all respect reduced motion.

```toml
[dependencies]
moonglass = "0.2"
gpui = { package = "gpui-pre", version = "=0.3.12" }
```

gpui is published on crates.io as `gpui-pre` snapshots; keep the app's pin equal to moonglass's.
Naming the dependency `gpui` lets gpui's macros (`actions!`, derives) resolve. GPUI Base, the
platform and the web backend are re-exported as `moonglass::base`, `moonglass::platform` and
`moonglass::web`.

```rust
use gpui::*;

moonglass::application()
    .with_assets(moonglass::Assets)
    .run(|cx| {
        moonglass::init(cx);
        let options = WindowOptions {
            app_id: Some("my-app".into()),
            ..moonglass::chrome::window_options()
        };
        // The root view calls `moonglass::theme::tick(window, cx)` first thing in `render`.
        moonglass::open_window(options, cx, |window, cx| MyApp::view(window, cx)).unwrap();
    });
```

An app with its own assets chains them: try its own source, then `moonglass::Assets`.

## Developing next to an app

Point the app at a local checkout without committing it, in the app's `.cargo/config.toml`:

```toml
[patch.crates-io]
moonglass = { path = "../moonglass" }
```

## License

GPL-3.0-or-later.
