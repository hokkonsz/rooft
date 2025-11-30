use bevy::prelude::*;

use crate::{
    assets::AppAssets,
    color,
    core::ElementList,
    ui::{BAR_SIZE, bundles::elem_button},
};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(Update, (resize, update))
        .add_observer(on_show)
        .add_observer(on_hide)
        //..
        ;
}

fn setup(mut commands: Commands) {
    commands
        .spawn((
            LeftPanel,
            Node {
                width: Val::Px(LeftPanel::WIDTH),
                height: Val::Percent(100.),
                top: Val::Px(BAR_SIZE),
                left: Val::Px(BAR_SIZE),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                flex_direction: FlexDirection::Row,
                border: UiRect::left(Val::Px(2.)),
                flex_shrink: 0.,
                ..default()
            },
            BackgroundColor(color::BLACK34),
            BorderColor::all(color::BLACK30),
        ))
        .with_children(|left_panel| {
            left_panel.spawn((
                LeftPanelList,
                Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    padding: UiRect::horizontal(Val::Px(10.)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Start,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                BackgroundColor(color::BLACK34),
            ));

            left_panel.spawn((
                LeftPanelHandle,
                Node {
                    width: Val::Px(LeftPanel::HANDLE_WIDTH),
                    height: Val::Percent(100.),
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Start,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                BackgroundColor(color::BLACK34),
            ));
        });
}

#[derive(Component)]
pub struct LeftPanel;

impl LeftPanel {
    pub const WIDTH: f32 = 250.;
    pub const HANDLE_WIDTH: f32 = 4.;
}

#[derive(Component)]
pub struct LeftPanelList;

#[derive(Component)]
#[require(Button)]
pub struct LeftPanelHandle;

#[derive(Event)]
pub struct ShowLeftPanel;

#[derive(Event)]
pub struct HideLeftPanel;

#[derive(Event)]
pub struct ResizeLeftPanel(pub f32);

fn on_show(
    _on_show_left_panel: On<ShowLeftPanel>,
    panel: Single<(&Node, &mut Visibility), With<LeftPanel>>,
    mut commands: Commands,
) {
    let (node, mut visibility) = panel.into_inner();

    *visibility = Visibility::Inherited;

    if let Val::Px(width) = node.width {
        commands.trigger(ResizeLeftPanel(width));
    }
}

fn on_hide(
    _on_hide_left_panel: On<HideLeftPanel>,
    mut visibility: Single<&mut Visibility, With<LeftPanel>>,
    mut commands: Commands,
) {
    **visibility = Visibility::Hidden;

    commands.trigger(ResizeLeftPanel(0.));
}

fn resize(
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

            let new_width = (cursor_pos.x - BAR_SIZE)
                .clamp(LeftPanel::WIDTH * 0.5, LeftPanel::WIDTH * 2.)
                + LeftPanel::HANDLE_WIDTH;

            panel_node.width = Val::Px(new_width);
            commands.trigger(ResizeLeftPanel(new_width));
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

fn update(
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
