use std::rc::Rc;

use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    AnyElement, App, ElementId, FontWeight, InteractiveElement as _, IntoElement,
    ParentElement as _, RenderOnce, Role, SharedString, StatefulInteractiveElement as _,
    Styled as _, Window, div, px, relative,
};

use super::{h_flex, motion};
use crate::theme::ActiveTheme as _;

type OnSelect = Rc<dyn Fn(usize, &mut Window, &mut App)>;

/// Pill tabs: the open one sits on a pill that slides between them, the rest are muted text.
#[derive(IntoElement)]
pub struct Tabs {
    id: ElementId,
    labels: Vec<SharedString>,
    selected: usize,
    fill: bool,
    on_select: OnSelect,
}

impl Tabs {
    pub fn new(
        id: impl Into<ElementId>,
        labels: Vec<SharedString>,
        selected: usize,
        on_select: impl Fn(usize, &mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            labels,
            selected,
            fill: false,
            on_select: Rc::new(on_select),
        }
    }

    /// Stretches the tabs to share the full width, as a segmented control.
    pub fn fill(mut self) -> Self {
        self.fill = true;
        self
    }
}

impl RenderOnce for Tabs {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;
        let key = format!("tabs:{}", self.id);
        let tabs: Vec<AnyElement> = self
            .labels
            .into_iter()
            .enumerate()
            .map(|(i, label)| {
                let on = i == self.selected;
                let f = self.on_select.clone();
                let hover =
                    motion::hover(SharedString::from(format!("{key}:{i}:hover")), window, cx);
                let fg = motion::animate(
                    SharedString::from(format!("{key}:{i}:fg")),
                    match (on, hover.on) {
                        (true, _) => c.text,
                        (false, true) => c.text2,
                        _ => c.muted,
                    },
                    window,
                    cx,
                );
                hover
                    .track(gpui_kit::base::Button::new(SharedString::from(format!(
                        "tab-{i}"
                    ))))
                    .role(Role::Tab)
                    .selected(on)
                    .h(px(28.))
                    .line_height(relative(1.4))
                    .px(px(12.))
                    .when(self.fill, |b| b.flex_1().justify_center())
                    .rounded(px(6.))
                    .cursor_pointer()
                    .text_color(fg)
                    .when(on, |b| b.font_weight(FontWeight::SEMIBOLD))
                    .on_click(move |_, window, cx| f(i, window, cx))
                    .child(label)
                    .into_any_element()
            })
            .collect();
        div()
            .id(self.id)
            .role(Role::TabList)
            .when(self.fill, |d| d.w_full())
            .child(motion::highlighted(
                &key,
                h_flex().gap(px(2.)).when(self.fill, |row| row.w_full()),
                tabs,
                Some(self.selected),
                |d| d.rounded(px(6.)).bg(c.sel),
                window,
                cx,
            ))
    }
}
