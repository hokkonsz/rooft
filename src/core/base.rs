use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology, VertexAttributeValues},
};

use crate::{
    assets::AppAssets,
    core::{
        ElementList,
        actions::{ActionList, Actions},
    },
};

#[derive(Component)]
pub struct Base;

#[derive(Event)]
pub struct OnSpawnBase(pub Vec2);

pub fn on_spawn_base(
    trigger: Trigger<OnSpawnBase>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    mut elements: ResMut<ElementList>,
    mut actions: ResMut<ActionList>,
    assets: Res<AppAssets>,
) {
    info!(
        target = ?trigger.target(),
        "on_spawn_base",
    );

    let id = commands
        .spawn((
            Base,
            Mesh3d(meshes.add(create_mesh(trigger.0))),
            MeshMaterial3d(assets.materials.matcaps.gray.clone()),
            Transform::from_xyz(0.0, 150., 0.0),
        ))
        .id();

    let name = format!("Base {}", id.index());
    commands.entity(id).insert(Name::from(name.clone()));
    elements.list.push((id, name));

    *actions = ActionList {
        list: vec![Actions::ResizeBaze],
    };
}

pub fn test_spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<AppAssets>,
) {
    commands.spawn((
        Base,
        // PICKABLE,
        Mesh3d(meshes.add(create_mesh(Vec2::new(15000., 10000.)))),
        MeshMaterial3d(assets.materials.matcaps.gray.clone()),
        Transform::from_xyz(0.0, 150., 0.0),
    ));
}

#[derive(Event)]
pub struct OnResizeBase(pub Vec2);

pub fn on_resize_base(
    trigger: Trigger<OnResizeBase>,
    base: Single<&Mesh3d, With<Base>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    info!(
        target = ?trigger.target(),
        "on_resize_base",
    );

    let Some(mesh) = meshes.get_mut(*base) else {
        return;
    };

    // Calculate half size (mm)
    let size = trigger.0 * 0.5;

    if let Some(VertexAttributeValues::Float32x3(positions)) =
        mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
    {
        for position in positions.iter_mut() {
            *position = [
                position[0].signum() * size.x,
                position[1],
                position[2].signum() * size.y,
            ];
        }
    }
}

fn create_mesh(size: Vec2) -> Mesh {
    // Calculate half size (mm)
    let size = size * 0.5;
    let height = 150.;

    // Keep the mesh data accessible in future frames to be able to mutate it in toggle_texture.
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        // Each array is an [x, y, z] coordinate in local space.
        // The camera coordinate space is right-handed x-right, y-up, z-back. This means "forward" is -Z.
        // Meshes always rotate around their local [0, 0, 0] when a rotation is applied to their Transform.
        // By centering our mesh around the origin, rotating the mesh preserves its center of mass.
        vec![
            // top (facing towards +y)
            [-size.x, height, -size.y], // vertex with index 0
            [size.x, height, -size.y],  // vertex with index 1
            [size.x, height, size.y],   // etc. until 23
            [-size.x, height, size.y],
            // bottom   (-y)
            [-size.x, -height, -size.y],
            [size.x, -height, -size.y],
            [size.x, -height, size.y],
            [-size.x, -height, size.y],
            // right    (+x)
            [size.x, -height, -size.y],
            [size.x, -height, size.y],
            [size.x, height, size.y],
            [size.x, height, -size.y],
            // left     (-x)
            [-size.x, -height, -size.y],
            [-size.x, -height, size.y],
            [-size.x, height, size.y],
            [-size.x, height, -size.y],
            // back     (+z)
            [-size.x, -height, size.y],
            [-size.x, height, size.y],
            [size.x, height, size.y],
            [size.x, -height, size.y],
            // forward  (-z)
            [-size.x, -height, -size.y],
            [-size.x, height, -size.y],
            [size.x, height, -size.y],
            [size.x, -height, -size.y],
        ],
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![
            // Normals for the top side (towards +y)
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            // Normals for the bottom side (towards -y)
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            // Normals for the right side (towards +x)
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            // Normals for the left side (towards -x)
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            // Normals for the back side (towards +z)
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            // Normals for the forward side (towards -z)
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
        ],
    )
    .with_inserted_indices(Indices::U32(vec![
        0, 3, 1, 1, 3, 2, // triangles making up the top (+y) facing side.
        4, 5, 7, 5, 6, 7, // bottom (-y)
        8, 11, 9, 9, 11, 10, // right (+x)
        12, 13, 15, 13, 14, 15, // left (-x)
        16, 19, 17, 17, 19, 18, // back (+z)
        20, 21, 23, 21, 22, 23, // forward (-z)
    ]))
}
