//! ## Implementation Notes
//!
//! - The `position` reported in `HitData` is in world space. The `normal` is a vector pointing
//!   away from the face, it is not guaranteed to be normalized for scaled meshes.

use bevy::app::prelude::*;
use bevy::camera::Camera;
use bevy::camera::visibility::RenderLayers;
use bevy::ecs::prelude::*;
use bevy::picking::PickingSystems;
use bevy::picking::backend::ray::RayMap;
use bevy::picking::backend::{HitData, PointerHits};
use bevy::picking::mesh_picking::ray_cast::{
    MeshRayCast, MeshRayCastSettings, RayCastVisibility, SimplifiedMesh,
};

use crate::core::Axis3d;

pub fn plugin(app: &mut App) {
    app.register_type::<SimplifiedMesh>()
        .add_systems(PreUpdate, update_hits.in_set(PickingSystems::Backend));
}

/// Casts rays into the scene and sends [`PointerHits`] events.
fn update_hits(
    ray_map: Res<RayMap>,
    picking_cameras: Query<(&Camera, Option<&RenderLayers>)>,
    pickables: Query<&Pickable>,
    layers: Query<&RenderLayers>,
    mut ray_cast: MeshRayCast,
    mut pointer_hits_writer: MessageWriter<PointerHits>,
) {
    for (&ray_id, &ray) in ray_map.iter() {
        let Ok((camera, cam_layers)) = picking_cameras.get(ray_id.camera) else {
            continue;
        };

        let cam_layers = cam_layers.to_owned().unwrap_or_default();

        let settings = MeshRayCastSettings {
            visibility: RayCastVisibility::VisibleInView,
            filter: &|entity| {
                let marker_requirement = pickables.get(entity).is_ok();

                // Other entities missing render layers are on the default layer 0
                let entity_layers = layers.get(entity).cloned().unwrap_or_default();
                let render_layers_match = cam_layers.intersects(&entity_layers);

                marker_requirement && render_layers_match
            },
            early_exit_test: &|entity_hit| pickables.get(entity_hit).is_ok(),
        };
        let picks = ray_cast
            .cast_ray(ray, &settings)
            .iter()
            .map(|(entity, hit)| {
                let hit_data = HitData::new(
                    ray_id.camera,
                    hit.distance,
                    Some(hit.point),
                    Some(hit.normal),
                );
                (*entity, hit_data)
            })
            .collect::<Vec<_>>();
        let order = camera.order as f32;
        if !picks.is_empty() {
            pointer_hits_writer.write(PointerHits::new(ray_id.pointer, picks, order));
        }
    }
}

#[derive(Component, PartialEq, Eq)]
pub enum Pickable {
    Node,
    NodeAxis(Axis3d),
}
