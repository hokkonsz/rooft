//! A [mesh ray casting](ray_cast) backend for [`bevy_picking`](crate).
//!
//! By default, all meshes are pickable. Picking can be disabled for individual entities
//! by adding [`Pickable::IGNORE`].
//!
//! To make mesh picking entirely opt-in, set [`MeshPickingSettings::require_markers`]
//! to `true` and add [`MeshPickingCamera`] and [`Pickable`] components to the desired camera and
//! target entities.
//!
//! To manually perform mesh ray casts independent of picking, use the [`MeshRayCast`] system parameter.
//!
//! ## Implementation Notes
//!
//! - The `position` reported in `HitData` is in world space. The `normal` is a vector pointing
//!   away from the face, it is not guaranteed to be normalized for scaled meshes.

use bevy::app::prelude::*;
use bevy::ecs::component::HookContext;
use bevy::ecs::prelude::*;
use bevy::ecs::world::DeferredWorld;
use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;
use bevy::pbr::MeshMaterial3d;
use bevy::picking::backend::ray::RayMap;
use bevy::picking::backend::{HitData, PointerHits};
use bevy::picking::events::{Pointer, Released};
use bevy::picking::mesh_picking::ray_cast::{
    MeshRayCast, MeshRayCastSettings, RayCastVisibility, SimplifiedMesh,
};
use bevy::picking::pointer::PointerButton;
use bevy::picking::{PickSet, Pickable};
use bevy::reflect::prelude::*;
use bevy::render::{prelude::*, view::RenderLayers};

use crate::assets::AppAssets;
use crate::materials::MatCap;

/// An optional component that marks cameras that should be used in the [`MeshPickingPlugin`].
///
/// Only needed if [`MeshPickingSettings::require_markers`] is set to `true`, and ignored otherwise.
#[derive(Debug, Clone, Default, Component, Reflect)]
#[reflect(Debug, Default, Component)]
pub struct MeshPickingCamera;

/// Runtime settings for the [`MeshPickingPlugin`].
#[derive(Resource, Reflect)]
#[reflect(Resource, Default)]
pub struct MeshPickingSettings {
    /// When set to `true` ray casting will only consider cameras marked with
    /// [`MeshPickingCamera`] and entities marked with [`Pickable`]. `false` by default.
    ///
    /// This setting is provided to give you fine-grained control over which cameras and entities
    /// should be used by the mesh picking backend at runtime.
    pub require_markers: bool,

    /// Determines how mesh picking should consider [`Visibility`]. When set to [`RayCastVisibility::Any`],
    /// ray casts can be performed against both visible and hidden entities.
    ///
    /// Defaults to [`RayCastVisibility::VisibleInView`], only performing picking against visible entities
    /// that are in the view of a camera.
    pub ray_cast_visibility: RayCastVisibility,
}

impl Default for MeshPickingSettings {
    fn default() -> Self {
        Self {
            require_markers: false,
            ray_cast_visibility: RayCastVisibility::VisibleInView,
        }
    }
}

pub fn plugin(app: &mut App) {
    app.init_resource::<MeshPickingSettings>()
        .register_type::<MeshPickingSettings>()
        .register_type::<SimplifiedMesh>()
        .add_systems(PreUpdate, update_hits.in_set(PickSet::Backend))
        .add_observer(on_mesh_pick);
}

/// Casts rays into the scene using [`MeshPickingSettings`] and sends [`PointerHits`] events.
pub fn update_hits(
    backend_settings: Res<MeshPickingSettings>,
    ray_map: Res<RayMap>,
    picking_cameras: Query<(&Camera, Has<MeshPickingCamera>, Option<&RenderLayers>)>,
    pickables: Query<&Pickable>,
    marked_targets: Query<&Pickable>,
    layers: Query<&RenderLayers>,
    mut ray_cast: MeshRayCast,
    mut output: EventWriter<PointerHits>,
) {
    for (&ray_id, &ray) in ray_map.iter() {
        let Ok((camera, cam_can_pick, cam_layers)) = picking_cameras.get(ray_id.camera) else {
            continue;
        };
        if backend_settings.require_markers && !cam_can_pick {
            continue;
        }

        let cam_layers = cam_layers.to_owned().unwrap_or_default();

        let settings = MeshRayCastSettings {
            visibility: backend_settings.ray_cast_visibility,
            filter: &|entity| {
                let marker_requirement =
                    !backend_settings.require_markers || marked_targets.get(entity).is_ok();

                // Other entities missing render layers are on the default layer 0
                let entity_layers = layers.get(entity).cloned().unwrap_or_default();
                let render_layers_match = cam_layers.intersects(&entity_layers);

                let is_pickable = pickables.get(entity).ok().is_none_or(|p| p.is_hoverable);

                marker_requirement && render_layers_match && is_pickable
            },
            early_exit_test: &|entity_hit| {
                pickables
                    .get(entity_hit)
                    .is_ok_and(|pickable| pickable.should_block_lower)
            },
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
            output.write(PointerHits::new(ray_id.pointer, picks, order));
        }
    }
}

#[derive(Component)]
#[component(on_add = on_add)]
#[component(on_remove = on_remove)]
pub struct SelectedMesh;

fn on_add(mut world: DeferredWorld, HookContext { entity, .. }: HookContext) {
    let assets = world.resource_ref::<AppAssets>();

    *world.get_mut::<MeshMaterial3d<MatCap>>(entity).unwrap() =
        MeshMaterial3d(assets.materials.matcaps.blue.clone());
}

fn on_remove(mut world: DeferredWorld, HookContext { entity, .. }: HookContext) {
    let assets = world.resource_ref::<AppAssets>();

    *world.get_mut::<MeshMaterial3d<MatCap>>(entity).unwrap() =
        MeshMaterial3d(assets.materials.matcaps.gray.clone());
}

pub fn on_mesh_pick(
    trigger: Trigger<Pointer<Released>>,
    selected: Query<Entity, With<SelectedMesh>>,
    nonselected: Query<Entity, (With<Mesh3d>, Without<SelectedMesh>)>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    if trigger.button != PointerButton::Primary {
        return;
    }

    if !input.pressed(KeyCode::ShiftLeft) {
        for entity in selected {
            if entity == trigger.target {
                return;
            }

            commands.entity(entity).remove::<SelectedMesh>();
        }
    }

    for entity in nonselected {
        if entity == trigger.target {
            commands.entity(entity).insert(SelectedMesh);
            break;
        }
    }
}
