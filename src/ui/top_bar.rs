use crate::{
    assets::AppAssets,
    color,
    ui::{
        BAR_SIZE,
        bundles::bar_button,
        left_panel::{LeftPanel, OnHideLeftPanel, OnResizeLeftPanel, OnShowLeftPanel},
    },
};
use bevy::{prelude::*, winit::WinitWindows};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_window,
                toggle_left_panel,
                minimize,
                maximize,
                close,
            ),
        )
        .add_observer(on_resize_left_panel)
        //..
        ;
}

fn setup(mut commands: Commands, assets: Res<AppAssets>) {
    // Top Bar
    commands
        .spawn((
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
                .spawn(Node {
                    width: Val::Px(BAR_SIZE),
                    height: Val::Px(BAR_SIZE),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Row,
                    flex_shrink: 0.,
                    ..default()
                })
                .with_children(|toggle_panel| {
                    toggle_panel
                        .spawn((
                            ToggleLeftPanelButton::Visible,
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
                            ImageNode::new(assets.icons.panel_visible.clone()),
                        ));
                });

            // Tab
            top_bar
                .spawn((
                    Tab,
                    Node {
                        width: Val::Px(Tab::WIDTH + BAR_SIZE * 2.),
                        height: Val::Px(BAR_SIZE),
                        position_type: PositionType::Absolute,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Row,
                        left: Val::Px(Tab::LEFT + BAR_SIZE),
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
                            width: Val::Px(Tab::WIDTH),
                            height: Val::Px(Tab::HEIGHT),
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
                    control_buttons.spawn(bar_button(&assets.icons.minimize, 15., MinimizeButton));

                    // Maximize
                    control_buttons.spawn(bar_button(&assets.icons.maximize, 15., MaximizeButton));

                    // Close
                    control_buttons.spawn(bar_button(&assets.icons.close, 15., CloseButton));
                });
        });
}

#[derive(Default, Component)]
#[require(Button)]
pub enum ToggleLeftPanelButton {
    #[default]
    Visible,
    Hidden,
}

#[derive(Component)]
pub struct ToggleLeftPanelIcon;

pub fn toggle_left_panel(
    button: Single<
        (
            &Interaction,
            &mut ToggleLeftPanelButton,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<ToggleLeftPanelButton>),
    >,
    assets: Res<AppAssets>,
    mut button_img: Single<&mut ImageNode, With<ToggleLeftPanelIcon>>,
    mut commands: Commands,
) {
    let (interact, mut state, mut bg, mut bc) = button.into_inner();

    match interact {
        Interaction::Pressed => {
            *bg = BackgroundColor(color::BLACK44);
            *bc = BorderColor(color::BLACK44);

            match *state {
                ToggleLeftPanelButton::Visible => {
                    *state = ToggleLeftPanelButton::Hidden;
                    button_img.image = assets.icons.panel_hidden.clone();
                    commands.trigger(OnHideLeftPanel);
                }
                ToggleLeftPanelButton::Hidden => {
                    *state = ToggleLeftPanelButton::Visible;
                    button_img.image = assets.icons.panel_visible.clone();
                    commands.trigger(OnShowLeftPanel);
                }
            };
        }
        Interaction::Hovered => {
            *bg = BackgroundColor(color::BLACK38);
            *bc = BorderColor(color::BLACK38);
        }
        Interaction::None => {
            *bg = BackgroundColor(color::BLACK30);
            *bc = BorderColor(color::BLACK30);
        }
    }
}

#[derive(Component)]
pub struct Tab;

impl Tab {
    pub const WIDTH: f32 = 150.;
    pub const LEFT: f32 = LeftPanel::WIDTH;
    pub const HEIGHT: f32 = BAR_SIZE - 25.;
}

#[derive(Component)]
pub struct TabName;

pub fn on_resize_left_panel(
    trigger: Trigger<OnResizeLeftPanel>,
    mut tab: Single<&mut Node, With<Tab>>,
) {
    tab.left = Val::Px(trigger.event().0 + BAR_SIZE)
}

#[derive(Component)]
#[require(Button)]
pub struct MinimizeButton;

pub fn minimize(
    button: Single<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<MinimizeButton>),
    >,
    mut window: Single<&mut Window>,
) {
    let (button_interact, mut button_bg) = button.into_inner();

    match button_interact {
        Interaction::Pressed => {
            *button_bg = BackgroundColor(color::BLACK44);
            window.set_minimized(true);
        }
        Interaction::Hovered => {
            *button_bg = BackgroundColor(color::BLACK38);
        }
        Interaction::None => {
            *button_bg = BackgroundColor(color::BLACK30);
        }
    }
}

#[derive(Component)]
#[require(Button)]
pub struct MaximizeButton;

pub fn maximize(
    button: Single<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<MaximizeButton>),
    >,
    mut winit: NonSendMut<WinitWindows>,
) {
    let (button_interact, mut button_bg) = button.into_inner();

    match button_interact {
        Interaction::Pressed => {
            *button_bg = BackgroundColor(color::BLACK44);

            let Some(winit_window) = winit.windows.iter_mut().next() else {
                return;
            };

            winit_window.1.set_maximized(!winit_window.1.is_maximized());
        }
        Interaction::Hovered => {
            *button_bg = BackgroundColor(color::BLACK38);
        }
        Interaction::None => {
            *button_bg = BackgroundColor(color::BLACK30);
        }
    }
}

#[derive(Component)]
#[require(Button)]
pub struct CloseButton;

pub fn close(
    button: Single<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<CloseButton>)>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    let (button_interact, mut button_bg) = button.into_inner();

    match button_interact {
        Interaction::Pressed => {
            *button_bg = BackgroundColor(color::RED128);
            app_exit_events.send(AppExit::Success);
        }
        Interaction::Hovered => {
            *button_bg = BackgroundColor(color::RED118);
        }
        Interaction::None => {
            *button_bg = BackgroundColor(color::BLACK30);
        }
    }
}

pub fn move_window(input: Res<ButtonInput<MouseButton>>, window: Single<&mut Window>) {
    if input.just_pressed(MouseButton::Left) {
        let Some(cursor_pos) = window.cursor_position() else {
            return;
        };

        let mut window = window.into_inner();

        if cursor_pos.x > BAR_SIZE
            && cursor_pos.x < window.width() - BAR_SIZE * 3.
            && cursor_pos.y <= BAR_SIZE
        {
            window.start_drag_move();
        }
    }
}
