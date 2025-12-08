#![allow(unused)]

use bevy::prelude::*;

use crate::materials::{FlatColorMat, MatCap};

pub fn plugin(app: &mut App) {
    app.init_resource::<AppAssets>()
        .add_systems(PreStartup, load_assets);
}

fn load_assets(
    asset_server: Res<AssetServer>,
    mut app_assets: ResMut<AppAssets>,
    mut matcaps: ResMut<Assets<MatCap>>,
    mut flat_colors: ResMut<Assets<FlatColorMat>>,
    mut meshes: ResMut<Assets<Mesh>>,
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
        lock: asset_server.load("icons/lock.png"),
    };

    app_assets.images = ImageAssets {
        logo: asset_server.load("image/rooft.png"),
        shape_l: asset_server.load("image/shape_l.png"),
        shape_n: asset_server.load("image/shape_n.png"),
        shape_rect: asset_server.load("image/shape_rect.png"),
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

    app_assets.materials.flat_colors = FlatColorMatAssets {
        red: flat_colors.add(FlatColorMat {
            color: LinearRgba::new(1., 0., 0., 1.),
        }),
        green: flat_colors.add(FlatColorMat {
            color: LinearRgba::new(0., 1., 0., 1.),
        }),
        blue: flat_colors.add(FlatColorMat {
            color: LinearRgba::new(0., 0., 1., 1.),
        }),
        orange: flat_colors.add(FlatColorMat {
            color: LinearRgba::new(1., 0.5, 0., 1.),
        }),
        dark_red: flat_colors.add(FlatColorMat {
            color: LinearRgba::new(0.5, 0., 0., 1.),
        }),
        dark_green: flat_colors.add(FlatColorMat {
            color: LinearRgba::new(0., 0.5, 0., 1.),
        }),
        dark_blue: flat_colors.add(FlatColorMat {
            color: LinearRgba::new(0., 0., 0.5, 1.),
        }),
        dark_orange: flat_colors.add(FlatColorMat {
            color: LinearRgba::new(0.5, 0.25, 0., 1.),
        }),
        light_red: flat_colors.add(FlatColorMat {
            color: LinearRgba::new(1., 0.25, 0.25, 1.),
        }),
        light_green: flat_colors.add(FlatColorMat {
            color: LinearRgba::new(0.25, 1., 0.25, 1.),
        }),
        light_blue: flat_colors.add(FlatColorMat {
            color: LinearRgba::new(0.25, 0.25, 1.0, 1.),
        }),
        light_orange: flat_colors.add(FlatColorMat {
            color: LinearRgba::new(1., 0.75, 0.25, 1.),
        }),
    };

    app_assets.meshes = MeshAssets {
        sphere_r100: meshes.add(Sphere::new(100.).mesh().ico(8).unwrap()),
        cylinder_r12: meshes.add(Cylinder::new(12., 350.).mesh().segments(8)),
    }
}

#[derive(Default, Resource)]
pub struct AppAssets {
    pub fonts: FontsAssets,
    pub icons: IconsAssets,
    pub images: ImageAssets,
    pub materials: MaterialAssets,
    pub meshes: MeshAssets,
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
    pub lock: Handle<Image>,
}

#[derive(Default)]
pub struct ImageAssets {
    pub logo: Handle<Image>,
    pub shape_l: Handle<Image>,
    pub shape_n: Handle<Image>,
    pub shape_rect: Handle<Image>,
}

#[derive(Default)]
pub struct MaterialAssets {
    pub matcaps: MatCapAssets,
    pub flat_colors: FlatColorMatAssets,
}

#[derive(Default)]
pub struct MatCapAssets {
    pub gray: Handle<MatCap>,
    pub blue: Handle<MatCap>,
}

#[derive(Default)]
pub struct FlatColorMatAssets {
    pub red: Handle<FlatColorMat>,
    pub green: Handle<FlatColorMat>,
    pub blue: Handle<FlatColorMat>,
    pub orange: Handle<FlatColorMat>,
    pub dark_red: Handle<FlatColorMat>,
    pub dark_green: Handle<FlatColorMat>,
    pub dark_blue: Handle<FlatColorMat>,
    pub dark_orange: Handle<FlatColorMat>,
    pub light_red: Handle<FlatColorMat>,
    pub light_green: Handle<FlatColorMat>,
    pub light_blue: Handle<FlatColorMat>,
    pub light_orange: Handle<FlatColorMat>,
}

#[derive(Default)]
pub struct MeshAssets {
    pub sphere_r100: Handle<Mesh>,
    pub cylinder_r12: Handle<Mesh>,
}
