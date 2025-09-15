use crate::{color, ui::BAR_SIZE};
use bevy::{prelude::*, winit::WinitWindows};

#[derive(Component)]
#[require(Button)]
pub struct ToggleLeftPanelButton;

#[derive(Component)]
pub struct ToggleLeftPanelIcon;

#[derive(Component)]
pub struct Tab;

#[derive(Component)]
pub struct TabName;

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
