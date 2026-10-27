use std::borrow::Cow;

use gpui_kit::{AssetSource, Result, SharedString};

macro_rules! icons {
    ($($name:literal),* $(,)?) => {
        const ICONS: &[(&str, &[u8])] = &[
            $((concat!("icons/", $name, ".svg"), include_bytes!(concat!("../assets/icons/", $name, ".svg")))),*
        ];
    };
}

icons!(
    "activity",
    "alert",
    "arrow-down",
    "arrow-left",
    "arrow-up",
    "arrow-up-down",
    "book-open",
    "check",
    "chevron-down",
    "chevron-right",
    "chevron-up",
    "close",
    "code",
    "compass",
    "copy",
    "download",
    "external-link",
    "eye",
    "file",
    "file-plus",
    "file-text",
    "filter",
    "folder",
    "folder-plus",
    "gauge",
    "git-branch",
    "globe",
    "history",
    "image",
    "info",
    "keyboard",
    "layout-dashboard",
    "layout-grid",
    "link",
    "loader",
    "lock",
    "more",
    "network",
    "package",
    "panel-bottom",
    "panel-left",
    "panel-right",
    "pause",
    "pencil",
    "play",
    "plus",
    "power",
    "refresh",
    "route",
    "search",
    "server",
    "settings",
    "shield",
    "stop",
    "tag",
    "terminal",
    "trash",
    "undo",
    "upload",
    "window-close",
    "window-maximize",
    "window-minimize",
    "window-restore",
    "zap",
);

/// The icons moonglass draws, embedded in the binary; chain it with the app's own source.
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        Ok(ICONS
            .iter()
            .find(|(p, _)| *p == path)
            .map(|(_, bytes)| Cow::Borrowed(*bytes)))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(ICONS
            .iter()
            .filter(|(p, _)| p.starts_with(path))
            .map(|(p, _)| (*p).into())
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use crate::IconName;

    #[test]
    fn every_icon_is_embedded() {
        for icon in IconName::ALL {
            let path = icon.path();
            assert!(
                super::ICONS.iter().any(|(p, _)| *p == path),
                "{path} is missing from the icons! list"
            );
        }
    }
}
