#![allow(unused)]

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_resource::<AppAssets>()
        .add_systems(PreStartup, load_assets);
}

fn load_assets(asset_server: Res<AssetServer>, mut app_assets: ResMut<AppAssets>) {
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
        panel_hide: asset_server.load("icons/panel_hide.png"),
        panel_show: asset_server.load("icons/panel_show.png"),
    };

    app_assets.image = ImageAssets {
        logo: asset_server.load("image/rooft.png"),
    };
}

#[derive(Default, Resource)]
pub struct AppAssets {
    pub fonts: FontsAssets,
    pub icons: IconsAssets,
    pub image: ImageAssets,
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
    pub panel_hide: Handle<Image>,
    pub panel_show: Handle<Image>,
}

#[derive(Default)]
pub struct ImageAssets {
    pub logo: Handle<Image>,
}
