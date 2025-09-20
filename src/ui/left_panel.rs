use crate::assets::AppAssets;
use crate::color;
use crate::core::ElementList;
use crate::ui::bundles::elem_button;
use crate::ui::{BAR_SIZE, PANEL_WIDTH};
use bevy::prelude::*;

#[derive(Component)]
pub struct LeftPanel;

#[derive(Component)]
pub struct LeftPanelList;

#[derive(Component)]
#[require(Button)]
pub struct LeftPanelHandle;

#[derive(Event)]
pub struct OnShowLeftPanel;

#[derive(Event)]
pub struct OnHideLeftPanel;

#[derive(Event)]
pub struct OnResizeLeftPanel(pub f32);

pub fn on_show(
    _trigger: Trigger<OnShowLeftPanel>,
    panel: Single<(&Node, &mut Visibility), With<LeftPanel>>,
    mut commands: Commands,
) {
    let (node, mut visibility) = panel.into_inner();

    *visibility = Visibility::Inherited;

    if let Val::Px(width) = node.width {
        commands.trigger(OnResizeLeftPanel(width));
    }
}

pub fn on_hide(
    _trigger: Trigger<OnHideLeftPanel>,
    mut visibility: Single<&mut Visibility, With<LeftPanel>>,
    mut commands: Commands,
) {
    **visibility = Visibility::Hidden;

    commands.trigger(OnResizeLeftPanel(0.));
}

pub fn resize(
    window: Single<&mut Window>,
    panel_handle: Single<(&Interaction, &mut BackgroundColor), With<LeftPanelHandle>>,
    mut panel_node: Single<&mut Node, With<LeftPanel>>,
    mut commands: Commands,
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
            commands.trigger(OnResizeLeftPanel(new_width));
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
                .spawn(elem_button(
                    text,
                    assets.fonts.iosevka.italic.clone(),
                    PanelElemButton,
                ))
                .id(),
        );
    }

    commands.entity(*panel_entity).add_children(&children);
}
