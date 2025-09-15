mod actions;
mod bundles;
mod left_bar;
mod left_panel;
mod top_bar;

use crate::assets::AppAssets;
use crate::color;
use crate::ui::bundles::bar_button;
use crate::ui::left_bar::LeftBar;
use crate::ui::left_panel::{LeftPanel, LeftPanelHandle, LeftPanelList};
use crate::ui::top_bar::{
    CloseButton, MaximizeButton, MinimizeButton, Tab, TabName, ToggleLeftPanelButton,
    ToggleLeftPanelIcon,
};
use bevy::prelude::*;

const PANEL_WIDTH: f32 = 250.;
const PANEL_HANDLE_WIDTH: f32 = 4.;
const BAR_SIZE: f32 = 40.;
const TAB_WIDTH: f32 = 150.;
const TAB_LEFT: f32 = PANEL_WIDTH;
const TAB_HEIGHT: f32 = BAR_SIZE - 25.;
const LIST_ELEM_HEIGHT: f32 = 15.;
const LIST_ELEM_MARGIN: f32 = 3.;
const LIST_ELEM_BORDER: f32 = 5.;

pub fn plugin(app: &mut App) {
    app
        //.add_plugins(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(ClearColor(color::BLACK18))
        // Enabled with features = ["bevy_ui_debug"]
        // .insert_resource(UiDebugOptions {
        //     enabled: true,
        //     ..default()
        // })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                top_bar::move_window,
                top_bar::minimize,
                top_bar::maximize,
                top_bar::close,
            ),
        )
        .add_systems(
            Update,
            (
                left_panel::toggle, //..
                left_panel::resize,
                left_panel::update,
            ),
        )
        .add_systems(Update, (actions::select, actions::toggle).chain());
}

fn setup(mut commands: Commands, assets: Res<AppAssets>) {
    commands
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|root| {
            // Top Bar
            root.spawn((
                Node {
                    width: Val::Percent(100.),
                    height: Val::Px(BAR_SIZE),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                BackgroundColor(color::BLACK30),
            ))
            .with_children(|top_bar| {
                // Toggle Panel Button
                top_bar
                    .spawn((Node {
                        width: Val::Px(BAR_SIZE),
                        height: Val::Px(BAR_SIZE),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Row,
                        flex_shrink: 0.,
                        ..default()
                    },))
                    .with_children(|toggle_panel| {
                        toggle_panel
                            .spawn((
                                ToggleLeftPanelButton,
                                Node {
                                    width: Val::Px(20.),
                                    height: Val::Px(20.),
                                    box_sizing: BoxSizing::ContentBox,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    flex_direction: FlexDirection::Row,
                                    border: UiRect::all(Val::Px(3.)),
                                    ..default()
                                },
                                BackgroundColor(color::BLACK18),
                                BorderColor(color::BLACK18),
                                BorderRadius::all(Val::Px(3.)),
                            ))
                            .with_child((
                                ToggleLeftPanelIcon,
                                Node {
                                    width: Val::Px(20.),
                                    height: Val::Px(20.),
                                    box_sizing: BoxSizing::ContentBox,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    flex_direction: FlexDirection::Row,
                                    ..default()
                                },
                                ImageNode::new(assets.icons.panel_hide.clone()),
                            ));
                    });

                // Tab
                top_bar
                    .spawn((
                        Tab,
                        Node {
                            width: Val::Px(TAB_WIDTH + BAR_SIZE * 2.),
                            height: Val::Px(BAR_SIZE),
                            position_type: PositionType::Absolute,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            flex_direction: FlexDirection::Row,
                            left: Val::Px(TAB_LEFT + BAR_SIZE),
                            ..default()
                        },
                    ))
                    .with_children(|tab| {
                        // Left Curve
                        tab.spawn((
                            Node {
                                width: Val::Px(BAR_SIZE),
                                height: Val::Px(BAR_SIZE),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                flex_direction: FlexDirection::Row,
                                ..default()
                            },
                            BackgroundColor(color::BLACK18),
                        ))
                        .with_child((
                            Node {
                                width: Val::Px(BAR_SIZE),
                                height: Val::Px(BAR_SIZE),
                                align_items: AlignItems::Start,
                                justify_content: JustifyContent::Center,
                                flex_direction: FlexDirection::Row,
                                border: UiRect {
                                    left: Val::Px(10.),
                                    right: Val::Px(10.),
                                    top: Val::Px(10.),
                                    bottom: Val::Px(0.),
                                },
                                ..default()
                            },
                            BackgroundColor(color::BLACK30),
                            BorderColor(color::BLACK30),
                            BorderRadius::bottom_right(Val::Px(10.)),
                        ));

                        // Middle
                        tab.spawn((
                            Node {
                                box_sizing: BoxSizing::ContentBox,
                                width: Val::Px(TAB_WIDTH),
                                height: Val::Px(TAB_HEIGHT),
                                align_self: AlignSelf::End,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                flex_direction: FlexDirection::Row,
                                border: UiRect {
                                    left: Val::Px(10.),
                                    right: Val::Px(10.),
                                    top: Val::Px(10.),
                                    bottom: Val::Px(5.),
                                },
                                ..default()
                            },
                            BackgroundColor(color::BLACK18),
                            BorderColor(color::BLACK18),
                            BorderRadius::top(Val::Px(10.)),
                        ))
                        .with_child((
                            TabName,
                            Text::from("New Project"),
                            TextFont {
                                font: assets.fonts.iosevka.italic.clone(),
                                font_size: 12.,
                                ..default()
                            },
                        ));

                        // Right Curve
                        tab.spawn((
                            Node {
                                width: Val::Px(BAR_SIZE),
                                height: Val::Px(BAR_SIZE),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                flex_direction: FlexDirection::Row,
                                ..default()
                            },
                            BackgroundColor(color::BLACK18),
                        ))
                        .with_child((
                            Node {
                                width: Val::Px(BAR_SIZE),
                                height: Val::Px(BAR_SIZE),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                flex_direction: FlexDirection::Row,
                                border: UiRect {
                                    left: Val::Px(10.),
                                    right: Val::Px(10.),
                                    top: Val::Px(10.),
                                    bottom: Val::Px(0.),
                                },
                                ..default()
                            },
                            BackgroundColor(color::BLACK30),
                            BorderColor(color::BLACK30),
                            BorderRadius::bottom_left(Val::Px(10.)),
                        ));
                    });

                // Control Buttons
                top_bar
                    .spawn(Node {
                        width: Val::Px(BAR_SIZE * 3.),
                        height: Val::Px(BAR_SIZE),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Row,
                        ..default()
                    })
                    .with_children(|control_buttons| {
                        // Minimize
                        control_buttons.spawn(bar_button(
                            &assets.icons.minimize,
                            15.,
                            MinimizeButton,
                        ));

                        // Maximize
                        control_buttons.spawn(bar_button(
                            &assets.icons.maximize,
                            15.,
                            MaximizeButton,
                        ));

                        // Close
                        control_buttons.spawn(bar_button(
                            &assets.icons.close, //..
                            15.,
                            CloseButton,
                        ));
                    });
            });

            // Container
            root.spawn(Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                flex_direction: FlexDirection::Row,
                ..default()
            })
            .with_children(|container| {
                // Left Bar
                container.spawn((
                    LeftBar,
                    Node {
                        width: Val::Px(BAR_SIZE),
                        height: Val::Percent(100.),
                        align_items: AlignItems::Start,
                        justify_content: JustifyContent::Start,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    BackgroundColor(color::BLACK34),
                ));

                // Left Panel
                container
                    .spawn((
                        LeftPanel,
                        Node {
                            width: Val::Px(PANEL_WIDTH),
                            height: Val::Percent(100.),
                            align_items: AlignItems::Start,
                            justify_content: JustifyContent::Start,
                            flex_direction: FlexDirection::Row,
                            border: UiRect::left(Val::Px(2.)),
                            ..default()
                        },
                        BackgroundColor(color::BLACK34),
                        BorderColor(color::BLACK30),
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
                                width: Val::Px(PANEL_HANDLE_WIDTH),
                                height: Val::Percent(100.),
                                align_items: AlignItems::Start,
                                justify_content: JustifyContent::Start,
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            BackgroundColor(color::BLACK34),
                        ));
                    });

                // Logo
                // container
                //     .spawn(Node {
                //         width: Val::Px(102.),
                //         height: Val::Px(42.),
                //         align_self: AlignSelf::End,
                //         justify_self: JustifySelf::End,
                //         align_items: AlignItems::Center,
                //         justify_content: JustifyContent::Center,
                //         flex_direction: FlexDirection::Column,
                //         margin: UiRect::all(Val::Px(5.)),
                //         ..default()
                //     })
                //     .with_child(ImageNode::new(assets.image.logo.clone()));
            });
        });
}
