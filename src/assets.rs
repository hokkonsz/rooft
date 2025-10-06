#![allow(unused)]

use bevy::prelude::*;

use crate::materials::MatCap;

pub fn plugin(app: &mut App) {
    app.init_resource::<AppAssets>()
        .add_systems(PreStartup, load_assets);
}

fn load_assets(
    asset_server: Res<AssetServer>,
    mut app_assets: ResMut<AppAssets>,
    mut matcaps: ResMut<Assets<MatCap>>,
) {
    app_assets.fonts.iosevka = IosevkaFont {
        regular: asset_server.load("fonts/Iosevka-Regular.ttf"),
        italic: asset_server.load("fonts/Iosevka-Italic.ttf"),
        bold: asset_server.load("fonts/Iosevka-Bold.ttf"),
        bold_italic: asset_server.load("fonts/Iosevka-BoldItalic.ttf"),
    };

    app_assets.icons = IconsAssets {
        minimize: asset_server.load("icons/minimize.png"),
        maximize: asset_server.load("icons/maximize.png"),
        close: asset_server.load("icons/close.png"),
        panel_hidden: asset_server.load("icons/panel_hidden.png"),
        panel_visible: asset_server.load("icons/panel_visible.png"),
    };

    app_assets.images = ImageAssets {
        logo: asset_server.load("image/rooft.png"),
    };

    app_assets.materials.matcaps = MatCapAssets {
        gray: matcaps.add(MatCap {
            texture: asset_server.load("image/matcaps/gray.png"),
            alpha_mode: AlphaMode::default(),
        }),
        blue: matcaps.add(MatCap {
            texture: asset_server.load("image/matcaps/blue.png"),
            alpha_mode: AlphaMode::default(),
        }),
    };
}

#[derive(Default, Resource)]
pub struct AppAssets {
    pub fonts: FontsAssets,
    pub icons: IconsAssets,
    pub images: ImageAssets,
    pub materials: MaterialAssets,
}

#[derive(Default)]
pub struct FontsAssets {
    pub iosevka: IosevkaFont,
}

#[derive(Default)]
pub struct IosevkaFont {
    pub regular: Handle<Font>,
    pub italic: Handle<Font>,
    pub bold: Handle<Font>,
    pub bold_italic: Handle<Font>,
}

#[derive(Default)]
pub struct IconsAssets {
    pub minimize: Handle<Image>,
    pub maximize: Handle<Image>,
    pub close: Handle<Image>,
    pub panel_hidden: Handle<Image>,
    pub panel_visible: Handle<Image>,
}

#[derive(Default)]
pub struct ImageAssets {
    pub logo: Handle<Image>,
}

#[derive(Default)]
pub struct MaterialAssets {
    pub matcaps: MatCapAssets,
}

#[derive(Default)]
pub struct MatCapAssets {
    pub gray: Handle<MatCap>,
    pub blue: Handle<MatCap>,
}
