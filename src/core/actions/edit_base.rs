use std::f32::consts::FRAC_PI_2;

use bevy::{input::keyboard::Key, prelude::*, window::CursorOptions};
use smol_str::SmolStr;

use crate::{
    assets::AppAssets,
    camera::{CameraLock, CameraTargetChanged, CameraView},
    color,
    core::{
        Axis3d,
        actions::ActionState,
        base::{self, Base},
        inputs::{DIGIT_KEYS, InputMode},
        picking::Pickable,
    },
    materials::ChangeMaterial,
};

const AXIS_HANDLE_OFFSET: f32 = 500.;

pub fn plugin(app: &mut App) {
    app.init_state::<EditBaseState>()
        .init_resource::<EditBaseContext>()
        // ActionState::EditBase
        .add_systems(OnEnter(ActionState::EditBase), setup_scene)
        .add_systems(
            Update,
            display_node_edges.run_if(in_state(ActionState::EditBase)),
        )
        .add_systems(
            OnExit(ActionState::EditBase),
            (update_camera_target, update_base, cleanup_scene).chain(),
        )
        // EditBaseState::PickingNode
        .add_systems(OnEnter(EditBaseState::PickingNode), enter_picking_node)
        .add_systems(
            Update,
            (picking_node, transition_picking_node)
                .chain()
                .run_if(in_state(EditBaseState::PickingNode)),
        )
        // EditBaseState::PickingAxis
        .add_systems(OnEnter(EditBaseState::PickingAxis), enter_picking_axis)
        .add_systems(
            Update,
            (picking_axis, transition_picking_axis)
                .chain()
                .run_if(in_state(EditBaseState::PickingAxis)),
        )
        // EditBaseState::Reposition
        .add_systems(OnEnter(EditBaseState::Reposition), enter_reposition)
        .add_systems(
            Update,
            (
                reposition_with_mouse,
                reposition_with_keyboard,
                reposition_check,
                reposition_node,
                transition_reposition,
            )
                .chain()
                .run_if(in_state(EditBaseState::Reposition)),
        )
        .add_systems(OnExit(EditBaseState::Reposition), exit_reposition);
}

fn setup_scene(
    mut commands: Commands,
    base: Res<Base>,
    assets: Res<AppAssets>,
    meshes: Query<&mut Visibility, With<Mesh3d>>,
    mut edit_base_context: ResMut<EditBaseContext>,
    mut edit_base_next: ResMut<NextState<EditBaseState>>,
    mut camera_view_next: ResMut<NextState<CameraView>>,
    mut camera_lock_next: ResMut<NextState<CameraLock>>,
) {
    let node_count = base.nodes().len();
    let mut node_entities = Vec::with_capacity(node_count);

    for node in base.nodes().iter() {
        let node_entity = commands
            .spawn((
                Pickable::Node,
                Mesh3d(assets.meshes.sphere_r100.clone()),
                MeshMaterial3d(assets.materials.flat_colors.orange.clone()),
                Transform::from_xyz(node.x(), Base::HALF_SIZE_DEFAULT.y * 2., node.z()),
            ))
            .id();

        node_entities.push(node_entity);
    }

    for curr_i in 0..node_count {
        let prev_i = base.node(curr_i).prev();
        let next_i = base.node(curr_i).next();

        let prev2_i = base.node(prev_i).prev();
        let next2_i = base.node(next_i).next();

        let mut prev = NodeEdge {
            index: prev_i,
            entity: node_entities[prev_i],
            axis: Axis3d::X,
            limit_nodes: [
                NodeEntity::new(prev2_i, node_entities[prev2_i]),
                NodeEntity::new(next_i, node_entities[next_i]),
            ],
        };

        let mut next = NodeEdge {
            index: next_i,
            entity: node_entities[next_i],
            axis: Axis3d::Z,
            limit_nodes: [
                NodeEntity::new(prev_i, node_entities[prev_i]),
                NodeEntity::new(next2_i, node_entities[next2_i]),
            ],
        };

        if base.node(curr_i).z() == base.node(prev_i).z() {
            prev.axis = Axis3d::Z;
            next.axis = Axis3d::X;
        }

        commands
            .entity(node_entities[curr_i])
            .insert(NodeEdgeConnection {
                index: curr_i,
                prev,
                next,
            });
    }

    for mut mesh_visibility in meshes {
        *mesh_visibility = Visibility::Hidden;
    }

    *edit_base_context = EditBaseContext::DEFAULT;
    edit_base_next.set(EditBaseState::default());
    camera_view_next.set(CameraView::Top);
    camera_lock_next.set(CameraLock::Locked);
}

fn update_base(mut base: ResMut<Base>, meshes: ResMut<Assets<Mesh>>) {
    base.update(meshes);
}

fn update_camera_target(mut commands: Commands, base: Res<Base>) {
    let (mut min_x, mut min_z) = (f32::MAX, f32::MAX);
    let (mut max_x, mut max_z) = (f32::MIN, f32::MIN);

    for node in base.nodes().iter() {
        if node.x() < min_x {
            min_x = node.x();
        }

        if node.z() < min_z {
            min_z = node.z()
        }

        if node.x() > max_x {
            max_x = node.x()
        }

        if node.z() > max_z {
            max_z = node.z()
        }
    }

    let origo = Vec3::new((max_x + min_x) / 2., 0., (max_z + min_z) / 2.);

    commands.trigger(CameraTargetChanged(origo));
}

fn cleanup_scene(
    mut commands: Commands,
    mesh3ds: Query<(Entity, Option<&Pickable>, &mut Visibility), With<Mesh3d>>,
    mut edit_base_context: ResMut<EditBaseContext>,
    mut edit_base_next: ResMut<NextState<EditBaseState>>,
    mut camera_lock_next: ResMut<NextState<CameraLock>>,
) {
    for (entity, pickable, mut visibility) in mesh3ds {
        let Some(Pickable::Node) = pickable else {
            *visibility = Visibility::Inherited;
            continue;
        };

        commands.entity(entity).despawn();
    }

    *edit_base_context = EditBaseContext::DEFAULT;
    edit_base_next.set(EditBaseState::default());
    camera_lock_next.set(CameraLock::Unlocked);
}

fn display_node_edges(
    mut gizmos: Gizmos,
    nodes: Query<(&NodeEdgeConnection, &Transform), With<NodeEdgeConnection>>,
) {
    for (
        curr_node,
        Transform {
            translation: curr_pos,
            ..
        },
    ) in nodes
    {
        let Ok((
            _,
            Transform {
                translation: next_pos,
                ..
            },
        )) = nodes.get(curr_node.next.entity)
        else {
            continue;
        };

        gizmos.line(*curr_pos, *next_pos, color::WHITE150);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum EditBaseState {
    #[default]
    PickingNode,
    PickingAxis,
    Reposition,
}

fn enter_picking_node(
    mut commands: Commands,
    mut edit_base_context: ResMut<EditBaseContext>,
    assets: Res<AppAssets>,
) {
    if let Some(selected_node) = edit_base_context.selected_node {
        commands
            .entity(selected_node.entity)
            .insert(ChangeMaterial::flat_color(
                assets.materials.flat_colors.orange.clone(),
            ))
            .despawn_related::<Children>();
    }
    *edit_base_context = EditBaseContext::DEFAULT;
}

fn picking_node(
    mut pointer_reader: MessageReader<Pointer<Press>>,
    nodes: Query<(Entity, &NodeEdgeConnection, &Transform), With<NodeEdgeConnection>>,
    mut commands: Commands,
    assets: Res<AppAssets>,
    mut edit_base_context: ResMut<EditBaseContext>,
) {
    let Some(pointer) = pointer_reader.read().next() else {
        return;
    };

    if pointer.button != PointerButton::Primary {
        return;
    }

    let Ok((entity, node, transform)) = nodes.get(pointer.entity) else {
        return;
    };

    commands.entity(entity).insert(ChangeMaterial::flat_color(
        assets.materials.flat_colors.dark_orange.clone(),
    ));

    let axis_x = commands
        .spawn((
            Pickable::NodeAxis(Axis3d::X),
            Mesh3d(assets.meshes.sphere_r100.clone()),
            MeshMaterial3d(assets.materials.flat_colors.red.clone()),
            transform.with_translation(Vec3::new(AXIS_HANDLE_OFFSET, 0., 0.)),
        ))
        .with_child((
            Mesh3d(assets.meshes.cylinder_r12.clone()),
            MeshMaterial3d(assets.materials.flat_colors.red.clone()),
            transform
                .with_translation(Vec3::new(-AXIS_HANDLE_OFFSET / 2., 0., 0.))
                .with_rotation(Quat::from_rotation_z(FRAC_PI_2)),
        ))
        .id();

    let axis_z = commands
        .spawn((
            Pickable::NodeAxis(Axis3d::Z),
            Mesh3d(assets.meshes.sphere_r100.clone()),
            MeshMaterial3d(assets.materials.flat_colors.blue.clone()),
            transform.with_translation(Vec3::new(0., 0., AXIS_HANDLE_OFFSET)),
        ))
        .with_child((
            Mesh3d(assets.meshes.cylinder_r12.clone()),
            MeshMaterial3d(assets.materials.flat_colors.blue.clone()),
            transform
                .with_translation(Vec3::new(0., 0., -AXIS_HANDLE_OFFSET / 2.))
                .with_rotation(Quat::from_rotation_x(FRAC_PI_2)),
        ))
        .id();

    edit_base_context.original_position = transform.translation;
    edit_base_context.selected_node = Some(NodeEntity {
        index: node.index,
        entity,
    });

    commands.entity(entity).add_children(&[axis_x, axis_z]);
}

fn transition_picking_node(
    edit_base_context: Res<EditBaseContext>,
    mut edit_base_next: ResMut<NextState<EditBaseState>>,
) {
    if edit_base_context.selected_node.is_some() {
        edit_base_next.set(EditBaseState::PickingAxis);
    }
}

fn enter_picking_axis(
    mut edit_base_context: ResMut<EditBaseContext>,
    mut pickables: Query<(&Pickable, &mut Visibility), With<Pickable>>,
) {
    for (pickable, mut visibility) in pickables.iter_mut() {
        match pickable {
            Pickable::NodeAxis(_) => {
                *visibility = Visibility::Inherited;
            }
            _ => (),
        }
    }
    edit_base_context.edge_node = None;
}

fn picking_axis(
    mut pointer_reader: MessageReader<Pointer<Press>>,
    pickables: Query<&Pickable, With<Pickable>>,
    nodes: Query<&NodeEdgeConnection>,
    mut edit_base_context: ResMut<EditBaseContext>,
) {
    let Some(pointer) = pointer_reader.read().next() else {
        return;
    };

    if pointer.button != PointerButton::Primary {
        return;
    }

    let Ok(pickable) = pickables.get(pointer.entity) else {
        return;
    };

    let Some(selected_node) = edit_base_context.selected_node else {
        return;
    };

    let Ok(selected_node) = nodes.get(selected_node.entity) else {
        return;
    };

    match pickable {
        Pickable::NodeAxis(axis) => {
            if selected_node.prev.axis == *axis {
                edit_base_context.edge_node = Some(selected_node.prev.clone());
            } else {
                edit_base_context.edge_node = Some(selected_node.next.clone());
            }
        }
        _ => (),
    }
}

fn transition_picking_axis(
    keyboard: Res<ButtonInput<KeyCode>>,
    edit_base_context: ResMut<EditBaseContext>,
    mut edit_base_next: ResMut<NextState<EditBaseState>>,
) {
    // Cancel
    if keyboard.just_pressed(KeyCode::Escape) || keyboard.just_pressed(KeyCode::Enter) {
        edit_base_next.set(EditBaseState::PickingNode);
        return;
    }

    if edit_base_context.edge_node.is_some() {
        edit_base_next.set(EditBaseState::Reposition);
    }
}

fn enter_reposition(
    mut pickables: Query<(&Pickable, &mut Visibility), With<Pickable>>,
    edit_base_context: Res<EditBaseContext>,
    mut cursor: Single<&mut CursorOptions>,
) {
    if let Some(edge_node) = &edit_base_context.edge_node {
        for (pickable, mut visibility) in pickables.iter_mut() {
            match (pickable, edge_node.axis) {
                (Pickable::NodeAxis(Axis3d::Z), Axis3d::X) => {
                    *visibility = Visibility::Hidden;
                }
                (Pickable::NodeAxis(Axis3d::X), Axis3d::Z) => {
                    *visibility = Visibility::Hidden;
                }
                _ => (),
            }
        }
    }

    cursor.visible = false;
}

fn reposition_with_mouse(
    camera: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    mut edit_base_context: ResMut<EditBaseContext>,
) {
    if edit_base_context.input_mode == InputMode::Keyboard {
        return;
    }

    let Some(edge_node) = &edit_base_context.edge_node else {
        return;
    };

    let (camera, camera_transform) = camera.into_inner();

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    let Some(distance) = ray.intersect_plane(
        Vec3::new(0., Base::HALF_SIZE_DEFAULT.y + 2., 0.),
        InfinitePlane3d::new(Dir3::Y),
    ) else {
        return;
    };

    let mouse_world_pos = ray.get_point(distance);

    match edge_node.axis {
        Axis3d::X => {
            edit_base_context.new_position.x = mouse_world_pos.x - AXIS_HANDLE_OFFSET;
        }
        Axis3d::Y => (),
        Axis3d::Z => {
            edit_base_context.new_position.z = mouse_world_pos.z - AXIS_HANDLE_OFFSET;
        }
    }
}

fn reposition_with_keyboard(
    keyboard_logical: Res<ButtonInput<Key>>,
    mut edit_base_context: ResMut<EditBaseContext>,
) {
    let Some(edge_node) = &edit_base_context.edge_node else {
        return;
    };

    match edit_base_context.input_mode {
        InputMode::Mouse => {
            let position = match edge_node.axis {
                Axis3d::X => &mut edit_base_context.new_position.x,
                Axis3d::Y => return,
                Axis3d::Z => &mut edit_base_context.new_position.z,
            };

            for index in 0..10 {
                if keyboard_logical.just_pressed(DIGIT_KEYS[index].clone()) {
                    *position = index as f32;
                    edit_base_context.input_mode = InputMode::Keyboard;
                    break;
                }
            }
        }
        InputMode::Keyboard => {
            let position = match edge_node.axis {
                Axis3d::X => &mut edit_base_context.new_position.x,
                Axis3d::Y => return,
                Axis3d::Z => &mut edit_base_context.new_position.z,
            };

            if keyboard_logical.just_pressed(Key::Backspace) {
                if *position == 0. {
                    edit_base_context.input_mode = InputMode::Mouse;
                } else {
                    *position = *position / 10.;
                }
            } else {
                for index in 0..10 {
                    if keyboard_logical.just_pressed(DIGIT_KEYS[index].clone()) {
                        *position = *position * 10. + index as f32;
                    }
                }
                if keyboard_logical.just_pressed(Key::Character(SmolStr::new_inline("-"))) {
                    *position = -*position;
                }
            }
        }
    }
}

fn reposition_check(mut edit_base_context: ResMut<EditBaseContext>, base: Res<Base>) {
    let Some(edge_node) = &edit_base_context.edge_node else {
        return;
    };

    let axis = edge_node.axis;
    let limit_nodes = edge_node.limit_nodes;

    let (old_position, new_position) = match axis {
        Axis3d::X => (
            edit_base_context.original_position.x,
            &mut edit_base_context.new_position.x,
        ),
        Axis3d::Y => return,
        Axis3d::Z => (
            edit_base_context.original_position.z,
            &mut edit_base_context.new_position.z,
        ),
    };

    if new_position.is_sign_positive() {
        *new_position = new_position.floor();
    } else {
        *new_position = new_position.ceil();
    }

    let (mut min, mut max) = (base::Node::MIN, base::Node::MAX);
    for limit_node in limit_nodes {
        let limit = base.position(limit_node.index, axis);

        if limit < old_position {
            if limit > min {
                min = limit + base::Node::MIN_DISTANCE;
            }
        } else {
            if limit < max {
                max = limit - base::Node::MIN_DISTANCE;
            }
        }
    }

    *new_position = new_position.clamp(min, max);
}

fn reposition_node(
    edit_base_context: Res<EditBaseContext>,
    mut pickables: Query<&mut Transform, With<Pickable>>,
    mut base: ResMut<Base>,
) {
    let Some(selected_node) = &edit_base_context.selected_node else {
        return;
    };

    let Some(edge_node) = &edit_base_context.edge_node else {
        return;
    };

    let selected_node_index = selected_node.index;
    let edge_node_index = edge_node.index;
    let axis = edge_node.axis;

    let Ok([mut selected_node_transform, mut edge_node_transform]) =
        pickables.get_many_mut([selected_node.entity, edge_node.entity])
    else {
        return;
    };

    match axis {
        Axis3d::X => {
            selected_node_transform.translation.x = edit_base_context.new_position.x;
            edge_node_transform.translation.x = edit_base_context.new_position.x;
            *base.node_x_mut(selected_node_index) = edit_base_context.new_position.x;
            *base.node_x_mut(edge_node_index) = edit_base_context.new_position.x;
        }
        Axis3d::Y => (),
        Axis3d::Z => {
            selected_node_transform.translation.z = edit_base_context.new_position.z;
            edge_node_transform.translation.z = edit_base_context.new_position.z;
            *base.node_z_mut(selected_node_index) = edit_base_context.new_position.z;
            *base.node_z_mut(edge_node_index) = edit_base_context.new_position.z;
        }
    }
}

fn transition_reposition(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut nodes: Query<&mut Transform, With<NodeEdgeConnection>>,
    mut edit_base_context: ResMut<EditBaseContext>,
    mut edit_base_next: ResMut<NextState<EditBaseState>>,
    mut base: ResMut<Base>,
) {
    // Cancel
    if keyboard.just_pressed(KeyCode::Escape)
        || (keyboard.pressed(KeyCode::ControlLeft) && keyboard.just_pressed(KeyCode::KeyZ))
    {
        let Some(selected_node) = &edit_base_context.selected_node else {
            return;
        };

        let Some(edge_node) = &edit_base_context.edge_node else {
            return;
        };

        let axis = edge_node.axis;
        let edge_node_index = edge_node.index;

        *base.node_x_mut(selected_node.index) = edit_base_context.original_position.x;
        *base.node_z_mut(selected_node.index) = edit_base_context.original_position.z;

        let Ok([mut selected_node_transform, mut edge_node_transform]) =
            nodes.get_many_mut([selected_node.entity, edge_node.entity])
        else {
            return;
        };

        selected_node_transform.translation = edit_base_context.original_position;

        if axis == Axis3d::X {
            *base.node_x_mut(edge_node_index) = edit_base_context.original_position.x;
            edge_node_transform.translation.x = edit_base_context.original_position.x;
        } else {
            *base.node_z_mut(edge_node_index) = edit_base_context.original_position.z;
            edge_node_transform.translation.z = edit_base_context.original_position.z;
        }

        edit_base_next.set(EditBaseState::PickingAxis);

    // Save
    } else if keyboard.just_pressed(KeyCode::Enter) {
        edit_base_context.original_position = edit_base_context.new_position;

        edit_base_next.set(EditBaseState::PickingAxis);
    }
}

fn exit_reposition(mut cursor: Single<&mut CursorOptions>) {
    cursor.visible = true;
}

#[derive(Resource)]
pub struct EditBaseContext {
    selected_node: Option<NodeEntity>,
    edge_node: Option<NodeEdge>,
    input_mode: InputMode,
    original_position: Vec3,
    new_position: Vec3,
}

impl EditBaseContext {
    const DEFAULT: Self = Self {
        selected_node: None,
        edge_node: None,
        input_mode: InputMode::Mouse,
        original_position: Vec3::ZERO,
        new_position: Vec3::ZERO,
    };

    pub fn node_index(&self) -> Option<usize> {
        let Some(selected_node) = self.selected_node else {
            return None;
        };

        Some(selected_node.index)
    }

    pub fn axis(&self) -> Option<Axis3d> {
        if let Some(edge_node) = &self.edge_node {
            return Some(edge_node.axis);
        }

        None
    }

    pub fn input_mode(&self) -> &InputMode {
        &self.input_mode
    }

    pub fn original_pos(&self) -> &Vec3 {
        &self.original_position
    }

    pub fn new_pos(&self) -> &Vec3 {
        &self.new_position
    }
}

impl Default for EditBaseContext {
    fn default() -> Self {
        EditBaseContext::DEFAULT
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
struct NodeEntity {
    index: usize,
    entity: Entity,
}

impl NodeEntity {
    const fn new(index: usize, entity: Entity) -> Self {
        Self { index, entity }
    }
}

#[derive(Component)]
struct NodeEdgeConnection {
    index: usize,
    prev: NodeEdge,
    next: NodeEdge,
}

#[derive(Clone)]
struct NodeEdge {
    index: usize,
    entity: Entity,
    axis: Axis3d,
    limit_nodes: [NodeEntity; 2],
}
