use crate::{
    assets::AppAssets,
    color,
    ui::{
        BAR_SIZE,
        left_panel::{OnHideLeftPanel, OnResizeLeftPanel, OnShowLeftPanel},
    },
};
use bevy::{prelude::*, winit::WinitWindows};

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
