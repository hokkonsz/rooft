use bevy::prelude::*;
use bevy_ui_text_input::{TextInputFilter, TextInputMode, TextInputNode, TextInputStyle};

use crate::{
    assets::FontsAssets,
    color,
    ui::{BAR_SIZE, LIST_ELEM_BORDER, LIST_ELEM_HEIGHT, LIST_ELEM_MARGIN},
};

pub fn button(
    width: f32,
    height: f32,
    image: &Handle<Image>,
    image_width: f32,
    image_height: f32,
    button_marker: impl Component,
) -> impl Bundle {
    (
        button_marker,
        Button,
        Node {
            width: Val::Px(width),
            height: Val::Px(height),
            box_sizing: BoxSizing::ContentBox,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Row,
            border: UiRect::all(Val::Px(10.)),
            ..default()
        },
        BackgroundColor(color::BLACK28),
        BorderColor::all(color::BLACK28),
        BorderRadius::all(Val::Px(5.)),
        Children::spawn_one((
            Node {
                width: Val::Px(image_width),
                height: Val::Px(image_height),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ImageNode::new(image.clone()),
        )),
    )
}

pub fn button_simple<T: Into<String>>(
    text: T,
    font: Handle<Font>,
    width: f32,
    button_marker: impl Component,
) -> impl Bundle {
    (
        button_marker,
        Button,
        Node {
            width: Val::Px(width),
            height: Val::Px(LIST_ELEM_HEIGHT),
            box_sizing: BoxSizing::ContentBox,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Row,
            flex_shrink: 0.,
            border: UiRect::all(Val::Px(10.)),
            ..default()
        },
        BackgroundColor(color::BLACK28),
        BorderColor::all(color::BLACK28),
        BorderRadius::all(Val::Px(5.)),
        Children::spawn_one((
            Text::from(text.into()),
            TextFont {
                font: font,
                font_size: 12.,
                ..default()
            },
            TextColor(color::WHITE200),
        )),
    )
}

// fn button_interact(
//     interact: &Interaction,
//     background_color: &mut BackgroundColor,
//     border_color: &mut BorderColor,
//     on_press: impl FnOnce() -> (),
// ) {
//     match interact {
//         Interaction::Pressed => on_press(),
//         Interaction::Hovered => {
//             *background_color = BackgroundColor(color::BLACK44);
//             *border_color = BorderColor::all(color::BLACK44);
//         }
//         Interaction::None => {
//             *background_color = BackgroundColor(color::BLACK30);
//             *border_color = BorderColor::all(color::BLACK30);
//         }
//     }
// }

pub fn button_bar(
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
            overflow: Overflow::clip(),
            margin: UiRect::vertical(Val::Px(LIST_ELEM_MARGIN)),
            ..default()
        },
        BackgroundColor(background_color),
        BorderColor::all(background_color),
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
        elem(text, font.clone(), color::BLACK23),
    )
}

#[derive(Default)]
pub enum Unit {
    #[default]
    None,
    Millimeter,
    Meter,
}

impl Unit {
    pub fn name(&self) -> String {
        match self {
            Unit::None => String::from(""),
            Unit::Millimeter => String::from("mm"),
            Unit::Meter => String::from("m"),
        }
    }
}

pub trait ChildSpawnerExt {
    fn input_field(
        &mut self,
        fonts: &FontsAssets,
        name: impl Into<String>,
        marker: impl Component,
        unit: Unit,
    );
}

impl ChildSpawnerExt for ChildSpawnerCommands<'_> {
    fn input_field(
        &mut self,
        fonts: &FontsAssets,
        name: impl Into<String>,
        marker: impl Component,
        unit: Unit,
    ) {
        self.spawn(Node {
            width: Val::Percent(100.),
            height: Val::Px(LIST_ELEM_HEIGHT),
            box_sizing: BoxSizing::ContentBox,
            border: UiRect::all(Val::Px(LIST_ELEM_BORDER)),
            position_type: PositionType::Relative,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceEvenly,
            flex_direction: FlexDirection::Row,
            margin: UiRect::vertical(Val::Px(LIST_ELEM_MARGIN)),
            ..default()
        })
        .with_children(|field| {
            // Name
            field.spawn((
                Node {
                    width: Val::Px(20.),
                    height: Val::Px(20.),
                    ..default()
                },
                Text::from(name.into()),
                TextFont {
                    font: fonts.iosevka.regular.clone(),
                    font_size: 12.,
                    ..default()
                },
                TextColor(color::WHITE200),
            ));

            // Input
            field.spawn((
                marker,
                Node {
                    width: Val::Px(100.),
                    height: Val::Px(20.),
                    ..default()
                },
                TextInputNode {
                    clear_on_submit: false,
                    mode: TextInputMode::SingleLine,
                    max_chars: Some(20),
                    justification: Justify::Center,
                    ..default()
                },
                TextInputFilter::Decimal,
                TextInputStyle {
                    cursor_color: color::WHITE200,
                    selection_color: color::BLACK68,
                    cursor_width: 1.,
                    cursor_height: 1.5,
                    ..default()
                },
                TextFont {
                    font: fonts.iosevka.italic.clone(),
                    font_size: 12.,
                    ..default()
                },
                TextColor(color::WHITE200),
                BackgroundColor(color::BLACK23),
            ));

            // Unit
            field.spawn((
                Node {
                    width: Val::Px(20.),
                    height: Val::Px(20.),
                    ..default()
                },
                Text::from(unit.name()),
                TextFont {
                    font: fonts.iosevka.italic.clone(),
                    font_size: 12.,
                    ..default()
                },
                TextColor(color::WHITE200),
            ));
        });
    }
}
