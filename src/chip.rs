use gpui_kit::{App, Div, Hsla, ParentElement as _, SharedString, Styled as _, div, px, relative};

use crate::theme::ActiveTheme as _;

/// What a chip or meter says about its value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tone {
    #[default]
    Muted,
    Accent,
    Ok,
    Warn,
    Danger,
}

impl Tone {
    pub fn color(self, cx: &App) -> Hsla {
        let c = cx.theme().colors;
        match self {
            Self::Muted => c.text2,
            Self::Accent => c.accent,
            Self::Ok => c.ok,
            Self::Warn => c.warn,
            Self::Danger => crate::theme::danger(),
        }
    }
}

/// A small tinted label: a kind, a state, a count.
pub fn chip(label: impl Into<SharedString>, tone: Tone, cx: &App) -> Div {
    let c = cx.theme().colors;
    let ink = tone.color(cx);
    let bg = match tone {
        Tone::Muted => c.row,
        _ => ink.opacity(0.14),
    };
    div()
        .flex_none()
        .px(px(6.))
        .h(px(18.))
        .flex()
        .items_center()
        .rounded(px(5.))
        .bg(bg)
        .text_color(ink)
        .text_size(px(11.))
        .font_family(cx.theme().mono.clone())
        .child(label.into())
}

/// A 6 px bar filled to `fraction` (0–1).
pub fn meter(fraction: f32, tone: Tone, cx: &App) -> Div {
    let c = cx.theme().colors;
    let fill = match tone {
        Tone::Muted => c.muted,
        _ => tone.color(cx),
    };
    div().h(px(6.)).w_full().rounded(px(3.)).bg(c.row).child(
        div()
            .h_full()
            .w(relative(fraction.clamp(0., 1.)))
            .rounded(px(3.))
            .bg(fill),
    )
}
