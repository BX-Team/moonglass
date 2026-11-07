mod assets;
mod button;
mod chip;
pub mod chrome;
pub mod code;
mod context_menu;
pub mod copy;
mod dialog;
mod field;
mod icon;
mod menu;
pub mod motion;
mod nav;
mod section;
mod switch;
mod tabs;
pub mod theme;
pub mod toast;

pub use gpui;
pub use gpui_base as base;
#[cfg(not(any(target_os = "ios", target_os = "android")))]
pub use gpui_platform as platform;
#[cfg(target_family = "wasm")]
pub use gpui_web as web;

/// This crate's version, for an About page.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub use assets::Assets;
pub use button::{Button, ButtonSize};
pub use chip::{Tone, chip, meter};
pub use chrome::TitleBar;
pub use context_menu::{ContextMenu, context_menu};
pub use copy::copy_button;
pub use dialog::{Modal, modal, modal_frame};
pub use field::{TextArea, TextField, textarea};
pub use icon::{IconName, icon};
pub use menu::{ActionMenu, Dropdown, MenuEntry, MenuItem};
pub use nav::{nav_row, placeholder};
pub use section::{Section, caption, setting_block, setting_row};
pub use switch::Switch;
pub use tabs::Tabs;
pub use theme::{ActiveTheme, Palette, ThemeMode};

pub use gpui_base::{h_flex, v_flex};

use gpui::{
    AnyView, AnyWindowHandle, App, AppContext as _, Context, Div, Entity, FontWeight, Global,
    IntoElement, ParentElement as _, Render, ScrollHandle, SharedString,
    StatefulInteractiveElement, Styled, Window, WindowOptions, div, px, relative,
};
use gpui_base::{Scrollbar, ScrollbarHandle};

/// The desktop or web application; its platform comes with moonglass.
#[cfg(not(any(target_os = "ios", target_os = "android")))]
pub fn application() -> gpui::Application {
    gpui_platform::application()
}

/// Sets up GPUI Base, the theme and the default labels; call once before opening a window.
pub fn init(cx: &mut App) {
    gpui_base::init(cx);
    theme::init(cx);
    cx.set_global(Labels::default());
}

/// Opens a window with GPUI Base's root around the view `build` returns, and gives back both.
pub fn open_window<V: Render>(
    options: WindowOptions,
    cx: &mut App,
    build: impl FnOnce(&mut Window, &mut App) -> Entity<V>,
) -> gpui::Result<(AnyWindowHandle, Entity<V>)> {
    let mut built = None;
    let window = cx.open_window(options, |window, cx| {
        let view = build(window, cx);
        built = Some(view.clone());
        cx.new(|cx| gpui_base::Root::new(view, window, cx))
    })?;
    let view = built.expect("open_window ran its build closure");
    Ok((window.into(), view))
}

/// The few words the components show on their own, in the app's language.
#[derive(Debug, Clone)]
pub struct Labels {
    /// The tooltip of a text field's clear button.
    pub clear: SharedString,
}

impl Default for Labels {
    fn default() -> Self {
        Self {
            clear: "Clear".into(),
        }
    }
}

impl Global for Labels {}

/// Replaces the labels, such as after the app's language changes.
pub fn set_labels(labels: Labels, cx: &mut App) {
    cx.set_global(labels);
    cx.refresh_windows();
}

pub(crate) fn labels(cx: &App) -> &Labels {
    cx.global::<Labels>()
}

pub const W_SEMIBOLD: FontWeight = FontWeight(650.);
pub const W_HEAVY: FontWeight = FontWeight(750.);

/// The window's text style; popovers, dialogs and tooltips paint outside the tree and set it again.
pub trait UiText: Styled + Sized {
    fn ui_text(self, cx: &App) -> Self {
        let theme = cx.theme();
        self.font_family(theme.font.clone())
            .text_size(px(13.))
            .line_height(relative(1.4))
            .text_color(theme.colors.text)
    }
}

impl<T: Styled> UiText for T {}

struct Tip(SharedString);

impl Render for Tip {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let c = cx.theme().colors;
        let tip = gpui_base::Tooltip::new("tip")
            .ui_text(cx)
            .px(px(8.))
            .py(px(4.))
            .rounded(px(6.))
            .border_1()
            .border_color(c.border)
            .bg(c.panel)
            .text_size(px(12.))
            .child(self.0.clone());
        // gpui puts a tooltip 1 px from the pointer, under the cursor arrow; the gap clears it.
        div()
            .pt(px(18.))
            .pl(px(6.))
            .child(motion::enter("tip-in", tip, 3., window, cx))
    }
}

/// A builder for gpui's `.tooltip(...)` showing one line of text.
pub fn tooltip(text: SharedString) -> impl Fn(&mut Window, &mut App) -> AnyView + 'static {
    move |_, cx| cx.new(|_| Tip(text.clone())).into()
}

/// An avatar-like square with a few letters: instance tiles, account icons.
pub fn tile(text: SharedString, size: f32, radius: f32, cx: &App) -> gpui::Div {
    let c = cx.theme().colors;
    div()
        .size(px(size))
        .flex_none()
        .rounded(px(radius))
        .bg(c.sel)
        .flex()
        .items_center()
        .justify_center()
        .font_weight(FontWeight::EXTRA_BOLD)
        .child(text)
}

/// A list scrolling inside a scrolling page keeps the wheel while it has more to show,
/// instead of scrolling the page along with it.
pub trait ContainScroll: StatefulInteractiveElement + Sized {
    fn contain_scroll(self, handle: &ScrollHandle) -> Self {
        let room = handle.clone();
        self.track_scroll(handle).on_scroll(move |_, _, cx| {
            if room.max_offset().y > px(0.) {
                cx.stop_propagation();
            }
        })
    }
}

impl<T: StatefulInteractiveElement> ContainScroll for T {}

/// A vertical scrollbar laid over a scrolling element; place it in a `relative` parent.
#[track_caller]
pub fn scrollbar<H: ScrollbarHandle + Clone>(handle: &H) -> Div {
    div()
        .absolute()
        .inset_0()
        .child(Scrollbar::vertical(handle).viewport_from_layout())
}

/// Vertical and horizontal scrollbars laid over a scrolling element.
#[track_caller]
pub fn scrollbars<H: ScrollbarHandle + Clone>(handle: &H) -> Div {
    div()
        .absolute()
        .inset_0()
        .child(Scrollbar::new(handle).viewport_from_layout())
}
