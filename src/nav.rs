use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    App, Div, FontWeight, ParentElement as _, SharedString, Styled as _, Window, div, px, relative,
};

use crate::theme::ActiveTheme as _;
use crate::{IconName, W_SEMIBOLD, icon, motion, v_flex};

/// A 30 px sidebar row that tints under the pointer; the selection pill is drawn by the list.
pub fn nav_row(
    id: impl Into<gpui_kit::ElementId>,
    active: bool,
    window: &mut Window,
    cx: &mut App,
) -> gpui_kit::base::Button {
    let c = cx.theme().colors;
    let id: gpui_kit::ElementId = id.into();
    let key = format!("nav:{id}");
    let hover = motion::hover(SharedString::from(format!("{key}:hover")), window, cx);
    let bg = motion::animate(
        SharedString::from(format!("{key}:bg")),
        if hover.on && !active {
            c.row
        } else {
            c.row.opacity(0.)
        },
        window,
        cx,
    );
    hover
        .track(gpui_kit::base::Button::new(id))
        .selected(active)
        .justify_start()
        .h(px(30.))
        .line_height(relative(1.4))
        .px(px(8.))
        .gap(px(8.))
        .rounded(px(6.))
        .cursor_pointer()
        .bg(bg)
        .map(|b| {
            if active {
                b.text_color(c.text).font_weight(FontWeight::SEMIBOLD)
            } else {
                b.text_color(c.text2)
            }
        })
}

/// A centered empty state with an icon, a title and a hint.
pub fn placeholder(name: IconName, title: SharedString, hint: SharedString, cx: &App) -> Div {
    let c = cx.theme().colors;
    v_flex()
        .size_full()
        .items_center()
        .justify_center()
        .gap(px(6.))
        .p(px(24.))
        .child(
            div()
                .size(px(44.))
                .mb(px(6.))
                .rounded(px(9.))
                .bg(c.sel)
                .flex()
                .items_center()
                .justify_center()
                .child(icon(name, c.muted).size(px(20.))),
        )
        .child(
            div()
                .font_weight(W_SEMIBOLD)
                .text_size(px(15.))
                .child(title),
        )
        .child(
            div()
                .max_w(px(360.))
                .text_center()
                .text_color(c.muted)
                .line_height(relative(1.5))
                .child(hint),
        )
}
