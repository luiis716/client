//! The Settings screen.
//!
//! A screen rather than a dialog: choosing a theme is exploratory, and a modal
//! you have to dismiss to see what it did to the window is the wrong shape for
//! it. Edits apply live and Escape closes it.

mod appearance;
mod panes;

use gpui::{
    App, Context, Entity, InteractiveElement, IntoElement, ParentElement, SharedString,
    StatefulInteractiveElement, Styled, Window, div, prelude::FluentBuilder as _,
};
use gpui_component::ActiveTheme as _;
use gpui_component::button::{Button, ButtonVariants as _};
use gpui_component::{Icon, IconName, Selectable as _, Sizable as _};

use crate::app::{SettingsSection, WhatsAppApp};
use crate::components::ProductIcon;
use crate::components::parts;
use crate::theme::Metrics;

pub fn render_settings_view(
    app: &mut WhatsAppApp,
    window: &mut Window,
    cx: &mut Context<WhatsAppApp>,
) -> impl IntoElement + use<> {
    // Before anything reads a plugin's tree, exactly as the conversation
    // does it. Settings is where a plugin's own panel lives, so leaving this
    // to the connected view alone meant a text field a plugin published there
    // had nowhere to hold what was typed and drew no box at all — the one
    // screen where it matters most.
    app.sync_plugin_fields(window, cx);

    let layout = app.responsive_layout(window, cx);
    let metrics = *layout.metrics();
    let entity = cx.entity().clone();
    let Some(settings) = app.settings(cx) else {
        return div().into_any_element();
    };
    let section = settings.section;
    // A fixed column beside a pane needs a window with room to spare. On a
    // phone it left the Appearance previews a few characters wide, so the
    // sections become a strip above the pane instead and the screen is one
    // column all the way down.
    let is_mobile = layout.is_mobile();

    div()
        .size_full()
        .flex()
        .bg(cx.theme().background)
        .when(!is_mobile, |el| {
            el.child(render_nav(section, entity.clone(), metrics, cx))
        })
        .child(
            parts::detail_stack()
                .child(render_header(
                    section,
                    is_mobile,
                    entity.clone(),
                    metrics,
                    cx,
                ))
                .when(is_mobile, |el| {
                    el.child(render_section_strip(section, entity.clone(), metrics, cx))
                })
                .child(
                    div()
                        .id("settings-body")
                        .flex_1()
                        .min_h_0()
                        .overflow_y_scroll()
                        .p(metrics.space_xxl())
                        .child(match section {
                            SettingsSection::Appearance => {
                                appearance::render(settings, entity, metrics, cx).into_any_element()
                            }
                            other => panes::render(other, app, entity, metrics, cx),
                        }),
                ),
        )
        .into_any_element()
}

fn render_nav(
    section: SettingsSection,
    entity: Entity<WhatsAppApp>,
    metrics: Metrics,
    cx: &App,
) -> impl IntoElement + use<> {
    let close_entity = entity.clone();

    div()
        .w(metrics.settings_nav_width())
        .flex_shrink_0()
        .flex()
        .flex_col()
        .bg(cx.theme().sidebar)
        .border_r_1()
        .border_color(cx.theme().border)
        .child(
            div()
                .h(metrics.sidebar_header_height())
                .flex()
                .items_center()
                .gap(metrics.space_lg())
                .px(metrics.space_lg())
                .child(
                    Button::new("settings-back")
                        .icon(IconName::ArrowLeft)
                        .ghost()
                        .small()
                        .tooltip(crate::l10n::tr("Back to chats"))
                        .cursor_pointer()
                        .on_click(move |_, _window, cx| {
                            close_entity.update(cx, |app, cx| {
                                app.close_settings(cx);
                            });
                        }),
                )
                .child(
                    div()
                        .text_size(metrics.text_title())
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                        .text_color(cx.theme().foreground)
                        .child("Configurações"),
                ),
        )
        .child(
            div()
                .flex_1()
                .min_h_0()
                .flex()
                .flex_col()
                .gap(metrics.space_xxs())
                .px(metrics.space_md())
                .children(SettingsSection::ALL.into_iter().map(|item| {
                    render_nav_item(item, item == section, entity.clone(), metrics, cx)
                })),
        )
}

/// One destination in the side nav.
///
/// A `Button`, because picking a section is a command and a styled `div`
/// carries neither focus nor keyboard activation — which left the whole
/// desktop navigation pointer-only while the phone strip beside it, built from
/// `Button`s, was reachable from the keyboard. The selection bar is drawn as a
/// child rather than through a variant, because it is the same bar the
/// conversation list uses and "where am I" should read the same way in both.
///
/// The row is one child rather than two, because a `Button` centres its own
/// content: `justify_start` on the frame left seven destinations reading down
/// the middle of a column, which is neither where a list of places starts nor
/// where the eye looks for the next one.
///
/// And the bar is a sibling of the button rather than a child of it, for the
/// same reason turned inside out. A button's children go into that content
/// box, which sits inside the button's own padding — so an absolutely
/// positioned child pinned to "the left edge" lands at the left edge of the
/// *text*, drawn straight through the icon it was supposed to sit beside.
fn render_nav_item(
    section: SettingsSection,
    is_selected: bool,
    entity: Entity<WhatsAppApp>,
    metrics: Metrics,
    cx: &App,
) -> impl IntoElement + use<> {
    div()
        .relative()
        .w_full()
        .child(
            Button::new(SharedString::from(format!("settings-nav-{}", section.id())))
                .ghost()
                .selected(is_selected)
                .w_full()
                .h(metrics.nav_item_height())
                // Room for the bar, and the same room on every row, so the
                // labels line up whether or not one is selected.
                .pl(metrics.space_xl())
                .pr(metrics.space_lg())
                .rounded(metrics.radius_md())
                .text_size(metrics.text_secondary())
                .child(
                    div()
                        .w_full()
                        .flex()
                        .items_center()
                        .justify_start()
                        .gap(metrics.space_lg())
                        .child(
                            Icon::new(icon_for(section))
                                .size(metrics.icon_small())
                                .flex_shrink_0(),
                        )
                        .child(section.label()),
                )
                .cursor_pointer()
                .on_click(move |_, _window, cx| {
                    entity.update(cx, |app, cx| app.set_settings_section(section, cx));
                }),
        )
        .when(is_selected, |el| {
            el.child(
                div()
                    .absolute()
                    .left_0()
                    .top(metrics.space_md())
                    .bottom(metrics.space_md())
                    .w(metrics.selection_bar_width())
                    .rounded_r(metrics.selection_bar_width())
                    .bg(cx.theme().primary),
            )
        })
}

/// The sections as a scrolling strip, for a window with no room beside the
/// pane. Same destinations as the side nav, same selection.
fn render_section_strip(
    section: SettingsSection,
    entity: Entity<WhatsAppApp>,
    metrics: Metrics,
    cx: &App,
) -> impl IntoElement + use<> {
    div()
        .id("settings-sections")
        .flex_shrink_0()
        .flex()
        .gap(metrics.space_xs())
        .px(metrics.space_lg())
        .py(metrics.space_md())
        .overflow_x_scroll()
        .border_b_1()
        .border_color(cx.theme().border)
        .children(SettingsSection::ALL.into_iter().map(|item| {
            let entity = entity.clone();
            Button::new(SharedString::from(format!("settings-tab-{}", item.id())))
                .label(item.label())
                .ghost()
                .small()
                .selected(item == section)
                .cursor_pointer()
                .on_click(move |_, _window, cx| {
                    entity.update(cx, |app, cx| app.set_settings_section(item, cx));
                })
        }))
}

fn render_header(
    section: SettingsSection,
    is_mobile: bool,
    entity: Entity<WhatsAppApp>,
    metrics: Metrics,
    cx: &App,
) -> impl IntoElement + use<> {
    let back_entity = entity.clone();

    div()
        .flex_shrink_0()
        .flex()
        .items_center()
        .justify_between()
        .gap(metrics.space_lg())
        .h(metrics.header_height())
        .px(metrics.space_xxl())
        .border_b_1()
        .border_color(cx.theme().border)
        // With no side nav there is no back button in it either, and Escape
        // is not a key a phone has.
        .when(is_mobile, |el| {
            el.child(
                Button::new("settings-back-mobile")
                    .icon(IconName::ArrowLeft)
                    .ghost()
                    .small()
                    .tooltip(crate::l10n::tr("Back to chats"))
                    .cursor_pointer()
                    .on_click(move |_, _window, cx| {
                        back_entity.update(cx, |app, cx| app.close_settings(cx));
                    }),
            )
        })
        .child(
            div()
                .flex()
                .flex_col()
                .child(
                    div()
                        .text_size(metrics.text_heading())
                        .font_weight(gpui::FontWeight::SEMIBOLD)
                        .text_color(cx.theme().foreground)
                        .child(section.label()),
                )
                .children(description_for(section).map(|text| {
                    div()
                        .text_size(metrics.text_small())
                        .text_color(cx.theme().muted_foreground)
                        .child(text)
                })),
        )
        .child(
            Button::new("settings-close")
                .icon(IconName::Close)
                .ghost()
                .small()
                .tooltip(crate::l10n::tr("Close settings"))
                .cursor_pointer()
                .on_click(move |_, _window, cx| {
                    entity.update(cx, |app, cx| {
                        app.close_settings(cx);
                    });
                }),
        )
}

fn description_for(section: SettingsSection) -> Option<&'static str> {
    match section {
        SettingsSection::Appearance => {
            Some("Os presets gravam no mesmo theme.json que você pode editar à mão.")
        }
        SettingsSection::Privacy => Some("A identidade deste dispositivo e como recomeçar."),
        SettingsSection::Storage => Some("O que este cliente guarda no disco."),
        SettingsSection::Plugins => Some(match crate::platform::plugins::home() {
            crate::platform::PluginHome::Folder => {
                "Carregados da pasta de plugins. Cada um diz o que pode fazer."
            }
            crate::platform::PluginHome::Page => {
                "Guardados neste navegador. Cada um diz o que pode fazer."
            }
            crate::platform::PluginHome::AnotherTab => {
                "Guardados neste navegador e carregados pela aba que segura esta conta."
            }
        }),
        _ => None,
    }
}

fn icon_for(section: SettingsSection) -> Icon {
    match section {
        SettingsSection::Account => Icon::new(IconName::CircleUser),
        SettingsSection::Appearance => Icon::new(IconName::Palette),
        SettingsSection::Notifications => Icon::new(IconName::Bell),
        SettingsSection::AudioVideo => Icon::new(ProductIcon::Volume),
        SettingsSection::Privacy => Icon::new(ProductIcon::Shield),
        SettingsSection::Storage => Icon::new(IconName::HardDrive),
        SettingsSection::Plugins => Icon::new(IconName::LayoutDashboard),
        SettingsSection::Advanced => Icon::new(IconName::Settings2),
    }
}
