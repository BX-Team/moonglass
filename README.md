# moonglass

BX Team's design system for [gpui](https://github.com/zed-industries/zed), built on
[GPUI Kit](https://github.com/longbridge/gpui-kit). It is the look shared by
[Riven](https://github.com/BX-Team/Riven) and [Nyx](https://github.com/BX-Team/Nyx).

- `theme`: a twelve-color `Palette`, six built-in themes (Graphite, Night, Moss, Ember, Day, Paper), app-registered themes, light/dark/system mode with a 260 ms fade.
- `TitleBar` and resize edges for windows that draw their own frame.
- Components: `Button`, `Tabs`, `Switch`, `TextField`, `TextArea`, `Dropdown`, `ActionMenu`, context menus, `Modal`, `Section` + `setting_row`, `nav_row`, `placeholder`, `copy_button`, toasts.
- `code`: a small lexer for TOML/JSON/YAML and a themed `CodeEditor`.
- `motion`: hover, color and position transitions, entrances, a gliding selection highlight; all respect reduced motion.

```toml
[dependencies]
moonglass = "0.1"
```

moonglass re-exports `gpui_kit`; depend on gpui through it so the versions cannot drift.

```rust
use moonglass::gpui_kit::*;

gpui_kit::application()
    .with_assets(moonglass::Assets)
    .run(|cx| {
        gpui_kit::init(cx);
        moonglass::init(cx);
        let options = WindowOptions {
            app_id: Some("my-app".into()),
            ..moonglass::chrome::window_options()
        };
        // The root view calls `moonglass::theme::tick(window, cx)` first thing in `render`.
        gpui_kit::open_window(options, cx, |window, cx| MyApp::view(window, cx)).unwrap();
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
