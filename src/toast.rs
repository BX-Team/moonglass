use std::time::Duration;

use gpui_kit::base::box_shadow;
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    AnyElement, App, ClipboardItem, Global, InteractiveElement as _, IntoElement,
    ParentElement as _, SharedString, Styled as _, Window, div, hsla, px, relative,
};

use crate::theme::ActiveTheme as _;
use crate::{Button, ButtonSize, IconName, UiText as _, h_flex, icon, motion, v_flex};

/// How many notifications stay on screen; older ones go first.
const VISIBLE: usize = 4;
const WIDTH: f32 = 360.;
/// The height of the buttons, which text and icon line up with.
const LINE: f32 = 28.;
const COPIED: Duration = Duration::from_millis(1500);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastKind {
    Success,
    Info,
    Warning,
    Error,
}

impl ToastKind {
    fn lifetime(self) -> Duration {
        match self {
            ToastKind::Success => Duration::from_secs(4),
            ToastKind::Info => Duration::from_secs(6),
            ToastKind::Warning => Duration::from_secs(8),
            ToastKind::Error => Duration::from_secs(10),
        }
    }
}

/// A notification shown in the corner of the window for a while.
#[derive(Debug, Clone)]
pub struct Toast {
    id: u64,
    kind: ToastKind,
    text: SharedString,
    /// The text was just copied: the copy button shows a check for a moment.
    copied: bool,
}

#[derive(Default)]
struct Toasts {
    items: Vec<Toast>,
    serial: u64,
}

impl Global for Toasts {}

/// Shows a notification that goes away by itself.
pub fn show(kind: ToastKind, text: impl Into<SharedString>, cx: &mut App) {
    let text = text.into();
    let toasts = cx.default_global::<Toasts>();
    if toasts
        .items
        .iter()
        .any(|t| t.text == text && t.kind == kind)
    {
        return;
    }
    toasts.serial += 1;
    let id = toasts.serial;
    toasts.items.push(Toast {
        id,
        kind,
        text,
        copied: false,
    });
    if toasts.items.len() > VISIBLE {
        toasts.items.remove(0);
    }
    cx.spawn(async move |cx| {
        cx.background_executor().timer(kind.lifetime()).await;
        cx.update(|cx| dismiss(id, cx));
    })
    .detach();
    cx.refresh_windows();
}

/// Copies a notification's text and confirms it on its copy button.
fn copy(id: u64, cx: &mut App) {
    let Some(toast) = cx
        .default_global::<Toasts>()
        .items
        .iter_mut()
        .find(|t| t.id == id)
    else {
        return;
    };
    toast.copied = true;
    let text = toast.text.to_string();
    cx.write_to_clipboard(ClipboardItem::new_string(text));
    cx.spawn(async move |cx| {
        cx.background_executor().timer(COPIED).await;
        cx.update(|cx| {
            if let Some(t) = cx
                .default_global::<Toasts>()
                .items
                .iter_mut()
                .find(|t| t.id == id)
            {
                t.copied = false;
                cx.refresh_windows();
            }
        });
    })
    .detach();
    cx.refresh_windows();
}

pub fn dismiss(id: u64, cx: &mut App) {
    cx.default_global::<Toasts>().items.retain(|t| t.id != id);
    cx.refresh_windows();
}

/// The notification stack, laid over the bottom right corner; `bottom` clears a bottom bar.
pub fn render(bottom: f32, window: &mut Window, cx: &mut App) -> AnyElement {
    let toasts = cx
        .try_global::<Toasts>()
        .map(|t| t.items.clone())
        .unwrap_or_default();
    if toasts.is_empty() {
        return div().into_any_element();
    }
    let c = cx.theme().colors;
    let items: Vec<AnyElement> = toasts
        .into_iter()
        .map(|toast| {
            let (mark, color) = match toast.kind {
                ToastKind::Success => (IconName::Check, c.ok),
                ToastKind::Info => (IconName::Info, c.accent),
                ToastKind::Warning => (IconName::Alert, c.warn),
                ToastKind::Error => (IconName::Close, crate::theme::danger()),
            };
            let id = toast.id;
            let card = h_flex()
                .w(px(WIDTH))
                .items_start()
                .gap(px(10.))
                .pl(px(12.))
                .pr(px(6.))
                .py(px(6.))
                .rounded(px(10.))
                .border_1()
                .border_color(c.border)
                .bg(c.panel)
                .shadow(vec![box_shadow(0., 8., 24., 0., hsla(0., 0., 0., 0.25))])
                .child(
                    h_flex()
                        .h(px(LINE))
                        .flex_none()
                        .child(icon(mark, color).size(px(14.))),
                )
                .child(
                    v_flex()
                        .flex_1()
                        .min_w_0()
                        .min_h(px(LINE))
                        .justify_center()
                        .child(
                            div()
                                .py(px(4.))
                                .line_height(relative(1.45))
                                .child(toast.text.clone()),
                        ),
                )
                .when(toast.kind == ToastKind::Error, |row| {
                    row.child(
                        Button::new(("toast-copy", id as usize))
                            .ghost()
                            .size(ButtonSize::Xs)
                            .icon(if toast.copied {
                                IconName::Check
                            } else {
                                IconName::Copy
                            })
                            .on_click(move |_, _, cx| copy(id, cx)),
                    )
                })
                .child(
                    Button::new(("toast-close", id as usize))
                        .ghost()
                        .size(ButtonSize::Xs)
                        .icon(IconName::Close)
                        .on_click(move |_, _, cx| dismiss(id, cx)),
                );
            motion::enter(("toast", id as usize), card, -6., window, cx)
        })
        .collect();
    v_flex()
        .id("toasts")
        .absolute()
        .right(px(16.))
        .bottom(px(bottom))
        .gap(px(8.))
        .items_end()
        .ui_text(cx)
        .occlude()
        .children(items)
        .into_any_element()
}
