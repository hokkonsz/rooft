use bevy::prelude::*;

use crate::{
    color,
    ui::{BAR_SIZE, LIST_ELEM_BORDER, LIST_ELEM_HEIGHT, LIST_ELEM_MARGIN},
};

pub fn bar_button(
    icon: &Handle<Image>,
    icon_size: f32,
    button_marker: impl Component,
) -> impl Bundle {
    (
        button_marker,
        Node {
            width: Val::Px(BAR_SIZE),
            height: Val::Px(BAR_SIZE),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Row,
            ..default()
        },
        Children::spawn_one((
            Node {
                width: Val::Px(icon_size),
                height: Val::Px(icon_size),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ImageNode::new(icon.clone()),
        )),
    )
}

pub fn list_elem<T: Into<String>>(
    text: T,
    font: &Handle<Font>,
    list_marker: impl Component,
) -> impl Bundle {
    (
        list_marker,
        Node {
            width: Val::Percent(100.),
            height: Val::Px(LIST_ELEM_HEIGHT),
            box_sizing: BoxSizing::ContentBox,
            border: UiRect::all(Val::Px(LIST_ELEM_BORDER)),
            position_type: PositionType::Relative,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Start,
            flex_direction: FlexDirection::Column,
            margin: UiRect::vertical(Val::Px(LIST_ELEM_MARGIN)),
            ..default()
        },
        BackgroundColor(color::BLACK30),
        BorderColor(color::BLACK30),
        BorderRadius::all(Val::Px(LIST_ELEM_BORDER)),
        Children::spawn_one((
            Text::from(text.into()),
            TextFont {
                font: font.clone(),
                font_size: 12.,
                ..default()
            },
        )),
    )
}
