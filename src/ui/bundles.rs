use bevy::{
    ecs::{relationship::RelatedSpawner, spawn::SpawnWith},
    prelude::*,
};

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

pub fn elem<T: Into<String>>(text: T, font: Handle<Font>, background_color: Color) -> impl Bundle {
    (
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
        BackgroundColor(background_color),
        BorderColor(background_color),
        BorderRadius::all(Val::Px(LIST_ELEM_BORDER)),
        Children::spawn_one((
            Text::from(text.into()),
            TextFont {
                font: font,
                font_size: 12.,
                ..default()
            },
        )),
    )
}

pub fn elem_button<T: Into<String>>(
    text: T,
    font: Handle<Font>,
    list_marker: impl Component,
) -> impl Bundle {
    (
        list_marker,
        Button,
        elem(text, font.clone(), color::BLACK30),
    )
}

#[derive(Component)]
struct OkButton;

#[derive(Component)]
struct CloseButton;

pub trait InputData: std::marker::Send + std::marker::Sync + 'static {
    fn name(&self) -> String;
    fn value(&self) -> f32;
}

pub fn input_panel<T: Into<String>>(
    left: Val,
    top: Val,
    text: T,
    font: Handle<Font>,
    data_list: Vec<(String, f32)>,
    list_marker: impl Component,
) -> impl Bundle {
    let moved_font = font.clone();
    (
        list_marker,
        Node {
            width: Val::Px(150.),
            height: Val::Px(150.),
            box_sizing: BoxSizing::ContentBox,
            left,
            top,
            border: UiRect::all(Val::Px(10.)),
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            align_content: AlignContent::Center,
            justify_items: JustifyItems::Start,
            justify_content: JustifyContent::Start,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(color::BLACK34),
        BorderColor(color::BLACK34),
        BorderRadius::all(Val::Px(10.)),
        Children::spawn((
            Spawn((
                Text::from(text.into()),
                TextFont {
                    font: font.clone(),
                    font_size: 12.,
                    ..default()
                },
            )),
            SpawnWith(move |panel: &mut RelatedSpawner<ChildOf>| {
                for data in data_list {
                    panel.spawn((
                        Node {
                            justify_content: JustifyContent::SpaceBetween,
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        Children::spawn((
                            Spawn(elem(data.0, moved_font.clone(), color::BLACK30)),
                            Spawn(elem(data.1.to_string(), moved_font.clone(), color::BLACK23)),
                        )),
                    ));
                }
            }),
            Spawn((
                Node {
                    justify_content: JustifyContent::SpaceBetween,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                Children::spawn((
                    Spawn(elem_button("Ok", font.clone(), OkButton)),
                    Spawn(elem_button("Close", font.clone(), CloseButton)),
                )),
            )),
        )),
    )
}
