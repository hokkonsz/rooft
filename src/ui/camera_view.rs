use crate::{
    assets::AppAssets,
    camera::{CameraLock, CameraView},
    color,
    ui::{
        BAR_SIZE,
        left_panel::{LeftPanel, OnResizeLeftPanel},
    },
};
use bevy::prelude::*;

const GAP: f32 = 15.;

pub fn plugin(app: &mut App) {
    app.init_state::<Dropdown>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(Dropdown::Expanded), expand)
        .add_systems(OnEnter(Dropdown::Collapsed), collapse)
        .add_systems(OnEnter(CameraLock::Locked), show_lock)
        .add_systems(OnEnter(CameraLock::Unlocked), hide_lock)
        .add_systems(Update, (header_interact, elem_interact, view_transition))
        .add_observer(on_resize_left_panel)
        //..
        ;
}

fn setup(mut commands: Commands, assets: Res<AppAssets>, camera_view: Res<State<CameraView>>) {
    commands
        .spawn((
            ViewDisplay,
            Node {
                top: Val::Px(BAR_SIZE + GAP),
                left: Val::Px(BAR_SIZE + LeftPanel::WIDTH + GAP),
                width: Val::Px(100.),
                height: Val::Px(50.),
                flex_direction: FlexDirection::Row,
                ..default()
            },
        ))
        .with_children(|view_display| {
            // View Text
            view_display.spawn((
                Text::from("View: "),
                TextColor(color::WHITE200),
                TextFont {
                    font: assets.fonts.iosevka.bold.clone(),
                    font_size: 12.,
                    ..default()
                },
            ));

            // View Dropdown
            view_display
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    ..default()
                })
                .with_children(|dropdown| {
                    dropdown.spawn((
                        DropdownHeader,
                        Button,
                        Text::from(camera_view.to_string()),
                        TextColor(color::WHITE200),
                        TextFont {
                            font: assets.fonts.iosevka.bold.clone(),
                            font_size: 12.,
                            ..default()
                        },
                    ));
                });

            // Locked Icon
            view_display.spawn((
                CameraLockIcon,
                Node {
                    width: Val::Px(9.),
                    height: Val::Px(13.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    margin: UiRect::left(Val::Px(5.)),
                    ..default()
                },
                Visibility::Hidden,
                ImageNode::new(assets.icons.lock.clone()),
            ));
        });
}

fn expand(
    mut commands: Commands,
    header_parent: Single<&ChildOf, With<DropdownHeader>>,
    camera_view: Res<State<CameraView>>,
    assets: Res<AppAssets>,
) {
    let mut expanded_list = Vec::with_capacity(CameraView::LEN);

    let header = commands
        .spawn((
            DropdownHeader,
            Button,
            Text::from(camera_view.to_string()),
            TextColor(color::WHITE200),
            TextFont {
                font: assets.fonts.iosevka.bold.clone(),
                font_size: 12.,
                ..default()
            },
        ))
        .id();

    expanded_list.push(header);

    for view in CameraView::LIST {
        if view == **camera_view {
            continue;
        }

        let elem = commands
            .spawn((
                DropdownElem,
                Button,
                view.clone(),
                Text::from(view.to_string()),
                TextColor(color::WHITE200),
                TextFont {
                    font: assets.fonts.iosevka.bold_italic.clone(),
                    font_size: 12.,
                    ..default()
                },
            ))
            .id();

        expanded_list.push(elem);
    }

    commands
        .entity(header_parent.0)
        .despawn_related::<Children>()
        .add_children(&expanded_list);
}

fn collapse(
    mut commands: Commands,
    header_parent: Single<&ChildOf, With<DropdownHeader>>,
    camera_view: Res<State<CameraView>>,
    assets: Res<AppAssets>,
) {
    let header = commands
        .spawn((
            DropdownHeader,
            Button,
            Text::from(camera_view.to_string()),
            TextColor(color::WHITE200),
            TextFont {
                font: assets.fonts.iosevka.bold.clone(),
                font_size: 12.,
                ..default()
            },
        ))
        .id();

    commands
        .entity(header_parent.0)
        .despawn_related::<Children>()
        .add_child(header);
}

#[derive(Component)]
pub struct CameraLockIcon;

fn show_lock(mut lock: Single<&mut Visibility, With<CameraLockIcon>>) {
    **lock = Visibility::Inherited;
}

fn hide_lock(mut lock: Single<&mut Visibility, With<CameraLockIcon>>) {
    **lock = Visibility::Hidden;
}

#[derive(Component)]
pub struct ViewDisplay;

fn on_resize_left_panel(
    trigger: Trigger<OnResizeLeftPanel>,
    mut display: Single<&mut Node, With<ViewDisplay>>,
) {
    display.left = Val::Px(trigger.event().0 + BAR_SIZE + GAP)
}

#[derive(Component)]
pub struct DropdownHeader;

fn header_interact(
    header: Single<(&Interaction, &mut TextColor), (Changed<Interaction>, With<DropdownHeader>)>,
    dropdown_curr: Res<State<Dropdown>>,
    mut dropdown_next: ResMut<NextState<Dropdown>>,
) {
    let (interact, mut text_color) = header.into_inner();

    match *interact {
        Interaction::Pressed => {
            **text_color = color::WHITE220;
            match **dropdown_curr {
                Dropdown::Collapsed => dropdown_next.set(Dropdown::Expanded),
                Dropdown::Expanded => dropdown_next.set(Dropdown::Collapsed),
            }
        }
        Interaction::Hovered => **text_color = color::WHITE150,
        Interaction::None => **text_color = color::WHITE200,
    }
}

#[derive(Component)]
pub struct DropdownElem;

fn elem_interact(
    elem_list: Query<
        (&Interaction, &CameraView, &mut TextColor),
        (Changed<Interaction>, With<DropdownElem>),
    >,
    mut camera_view_next: ResMut<NextState<CameraView>>,
) {
    for (interact, view, mut text_color) in elem_list {
        match *interact {
            Interaction::Pressed => {
                **text_color = color::WHITE220;
                camera_view_next.set(view.clone());
                break;
            }
            Interaction::Hovered => **text_color = color::WHITE150,
            Interaction::None => **text_color = color::WHITE200,
        }
    }
}

#[derive(Debug, Default, States, Hash, PartialEq, Eq, Clone)]
enum Dropdown {
    #[default]
    Collapsed,
    Expanded,
}

fn view_transition(
    mut state_events: EventReader<StateTransitionEvent<CameraView>>,
    mut header_text: Single<&mut Text, With<DropdownHeader>>,
    mut dropdown_next: ResMut<NextState<Dropdown>>,
) {
    let Some(transition) = state_events.read().next() else {
        return;
    };

    let Some(new_view) = &transition.entered else {
        return;
    };

    ***header_text = new_view.to_string();

    dropdown_next.set(Dropdown::Collapsed);
}
