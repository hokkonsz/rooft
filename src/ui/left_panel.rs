use crate::assets::AppAssets;
use crate::color;
use crate::core::ElementList;
use crate::ui::bundles::list_elem;
use crate::ui::left_bar::LeftBar;
use crate::ui::top_bar::{Tab, ToggleLeftPanelButton, ToggleLeftPanelIcon};
use crate::ui::{BAR_SIZE, PANEL_WIDTH};
use bevy::prelude::*;

#[derive(Component)]
pub struct LeftPanel;

#[derive(Component)]
pub struct LeftPanelList;

#[derive(Component)]
#[require(Button)]
pub struct LeftPanelHandle;

pub fn toggle(
    button: Single<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<ToggleLeftPanelButton>),
    >,
    mut button_img: Single<&mut ImageNode, With<ToggleLeftPanelIcon>>,
    panel: Single<(&Node, &mut Visibility), With<LeftPanel>>,
    mut tab_node: Single<&mut Node, (With<Tab>, Without<LeftPanel>)>,
    mut bar_bg: Single<&mut BackgroundColor, (With<LeftBar>, Without<ToggleLeftPanelButton>)>,
    assets: Res<AppAssets>,
) {
    let (button_interact, mut button_bg, mut button_bc) = button.into_inner();
    let (panel_node, mut panel_vis) = panel.into_inner();

    match button_interact {
        Interaction::Pressed => {
            *button_bg = BackgroundColor(color::BLACK44);
            *button_bc = BorderColor(color::BLACK44);
            match *panel_vis {
                Visibility::Inherited => {
                    *panel_vis = Visibility::Hidden;
                    **bar_bg = BackgroundColor(color::BLACK30);
                    button_img.image = assets.icons.panel_show.clone();
                    tab_node.left = Val::Px(BAR_SIZE);
                }
                Visibility::Visible => {
                    *panel_vis = Visibility::Hidden;
                    **bar_bg = BackgroundColor(color::BLACK30);
                    button_img.image = assets.icons.panel_show.clone();
                    tab_node.left = Val::Px(BAR_SIZE);
                }
                Visibility::Hidden => {
                    *panel_vis = Visibility::Inherited;
                    **bar_bg = BackgroundColor(color::BLACK34);
                    button_img.image = assets.icons.panel_hide.clone();
                    if let Val::Px(width) = panel_node.width {
                        tab_node.left = Val::Px(width + BAR_SIZE);
                    }
                }
            }
        }
        Interaction::Hovered => {
            *button_bg = BackgroundColor(color::BLACK38);
            *button_bc = BorderColor(color::BLACK38);
        }
        Interaction::None => {
            *button_bg = BackgroundColor(color::BLACK30);
            *button_bc = BorderColor(color::BLACK30);
        }
    }
}

pub fn resize(
    window: Single<&mut Window>,
    panel_handle: Single<(&Interaction, &mut BackgroundColor), With<LeftPanelHandle>>,
    mut panel_node: Single<&mut Node, With<LeftPanel>>,
    mut tab_node: Single<&mut Node, (With<Tab>, Without<LeftPanel>)>,
) {
    let (ph_interact, mut ph_bg) = panel_handle.into_inner();

    match ph_interact {
        Interaction::Pressed => {
            *ph_bg = BackgroundColor(color::BLACK54);
            let Some(cursor_pos) = window.into_inner().cursor_position() else {
                return;
            };

            let new_width = (cursor_pos.x - BAR_SIZE).clamp(PANEL_WIDTH * 0.5, PANEL_WIDTH * 2.);

            panel_node.width = Val::Px(new_width);
            tab_node.left = Val::Px(new_width + BAR_SIZE);
        }
        Interaction::Hovered => {
            *ph_bg = BackgroundColor(color::BLACK44);
        }
        Interaction::None => {
            *ph_bg = BackgroundColor(color::BLACK34);
        }
    }
}

#[derive(Component)]
#[require(Button)]
struct PanelElemButton;

pub fn update(
    elements: Res<ElementList>,
    assets: Res<AppAssets>,
    panel_entity: Single<Entity, With<LeftPanelList>>,
    mut commands: Commands,
) {
    if !elements.is_changed() {
        return;
    }

    commands.entity(*panel_entity).despawn_related::<Children>();

    if elements.list.is_empty() {
        return;
    }

    let mut children = Vec::with_capacity(elements.list.len());
    for (entity, name) in elements.list.iter() {
        let text = format!("{} : {name}", entity.index());
        children.push(
            commands
                .spawn(list_elem(
                    text,
                    &assets.fonts.iosevka.italic,
                    PanelElemButton,
                ))
                .id(),
        );
    }

    commands.entity(*panel_entity).add_children(&children);
}
