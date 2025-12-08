use bevy::prelude::*;

use crate::{
    assets::AppAssets,
    core::{ElementList, base::Base},
};

pub fn plugin(app: &mut App) {
    app.add_observer(on_spawn_base);
}

#[derive(Event)]
pub struct SpawnBase(pub BaseShape);

pub fn on_spawn_base(
    on_spawn_base: On<SpawnBase>,
    meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    mut elements: ResMut<ElementList>,
    assets: Res<AppAssets>,
) {
    let base = on_spawn_base.0.create(meshes);

    let id = commands
        .spawn((
            Name::from("Base"),
            Mesh3d(base.mesh()),
            MeshMaterial3d(assets.materials.matcaps.gray.clone()),
            Transform::from_xyz(0., Base::HALF_SIZE_DEFAULT.y, 0.),
        ))
        .id();

    commands.insert_resource(base);

    elements.list.push((id, String::from("Base")));
}

#[derive(Component, Clone, Copy)]
pub enum BaseShape {
    Rectangle,
    L,
    N,
}

impl BaseShape {
    pub fn create(&self, meshes: ResMut<Assets<Mesh>>) -> Base {
        // Calculate half size (mm)
        let x = Base::HALF_SIZE_DEFAULT.x;
        let z = Base::HALF_SIZE_DEFAULT.z;

        match self {
            BaseShape::Rectangle => Base::builder()
                .start_point(-x, -z)
                .move_x_to(x)
                .move_z_to(z)
                .move_x_to(-x)
                .build(meshes),
            BaseShape::L => Base::builder()
                .start_point(-x, -z)
                .move_z_to(z)
                .move_x_to(0.)
                .move_z_to(0.)
                .move_x_to(x)
                .move_z_to(-z)
                .build(meshes),
            BaseShape::N => {
                // Calculate special points
                let p0 = x / 3.;
                let p1 = z / 3.;

                Base::builder()
                    .start_point(-x, -z)
                    .move_x_to(x)
                    .move_z_to(z)
                    .move_x_to(p0)
                    .move_z_to(p1)
                    .move_x_to(-p0)
                    .move_z_to(z)
                    .move_x_to(-x)
                    .build(meshes)
            }
        }
    }
}
