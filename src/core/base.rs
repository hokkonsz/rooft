use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology, VertexAttributeValues},
};

use crate::{assets::AppAssets, core::ElementList};

const HALF_SIZE_DEFAULT: Vec3 = Vec3::new(15000. * 0.5, 300. * 0.5, 10000. * 0.5);
const MIN_DISTANCE: f32 = 300.;

const VERTEX_PER_NODE: u32 = 6;
const INDEX_PER_TRIANGLE: usize = 3;

#[derive(Component, Clone, Copy)]
pub enum BaseShape {
    Rectangle,
    L,
    N,
}

impl BaseShape {
    pub const fn vertices(&self, point: BasePoint) -> &[usize] {
        match self {
            BaseShape::L => match point {
                BasePoint::A => &[6, 7, 17, 23, 30, 31],
                BasePoint::B => &[4, 5, 16, 22, 28, 29],
                BasePoint::C => &[2, 3, 15, 21, 26, 27],
                _ => &[],
            },
            BaseShape::N => match point {
                BasePoint::A => &[4, 5, 20, 28, 40, 41],
                BasePoint::B => &[6, 7, 21, 29, 38, 39],
                BasePoint::C => &[14, 15, 22, 30, 36, 37],
                BasePoint::D => &[12, 13, 23, 31, 34, 35],
            },
            _ => &[],
        }
    }

    pub fn name(&self) -> String {
        match self {
            BaseShape::Rectangle => String::from("Base Rectangle"),
            BaseShape::L => String::from("Base-L"),
            BaseShape::N => String::from("Base-N"),
        }
    }
}

pub enum BasePoint {
    A,
    B,
    C,
    D,
}

#[derive(Event)]
pub struct OnReshapeBase(pub BaseShape);

pub fn on_reshape_base(
    trigger: Trigger<OnReshapeBase>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    mut elements: ResMut<ElementList>,
    assets: Res<AppAssets>,
) {
    let id = commands
        .spawn((
            trigger.0.clone(),
            Mesh3d(meshes.add(create_base_mesh(trigger.0))),
            MeshMaterial3d(assets.materials.matcaps.gray.clone()),
            Transform::from_xyz(0.0, 150., 0.0),
        ))
        .id();

    let name = format!("{} {}", trigger.0.name(), id.index());
    commands.entity(id).insert(Name::from(name.clone()));

    elements.list.push((id, name));
}

fn create_base_mesh(shape: BaseShape) -> Mesh {
    match shape {
        BaseShape::Rectangle => base_rectangle(),
        BaseShape::L => base_l(),
        BaseShape::N => base_n(),
    }
}

fn base_rectangle() -> Mesh {
    // Calculate half size (mm)
    let x = HALF_SIZE_DEFAULT.x;
    let z = HALF_SIZE_DEFAULT.z;

    Base::builder()
        .start_point(-x, -z)
        .move_x_to(x)
        .move_z_to(z)
        .move_x_to(-x)
        .build()
        .mesh
}

fn base_l() -> Mesh {
    // Calculate half size (mm)
    let x = HALF_SIZE_DEFAULT.x;
    let z = HALF_SIZE_DEFAULT.z;

    Base::builder()
        .start_point(-x, -z)
        .move_z_to(z)
        .move_x_to(0.)
        .move_z_to(0.)
        .move_x_to(x)
        .move_z_to(-z)
        .build()
        .mesh
}

fn base_n() -> Mesh {
    // Calculate half size (mm)
    let x = HALF_SIZE_DEFAULT.x;
    let z = HALF_SIZE_DEFAULT.z;

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
        .build()
        .mesh
}

// TODO! Remove OnResizeBase and introduce new event which allows user
// to edit the shape nodes.

#[derive(Event)]
pub struct OnResizeBase(pub Vec2);

impl From<[f32; 2]> for OnResizeBase {
    fn from(a: [f32; 2]) -> Self {
        Self(Vec2::new(a[0], a[1]))
    }
}

pub fn on_resize_base(
    trigger: Trigger<OnResizeBase>,
    base: Single<(&Mesh3d, &BaseShape), With<BaseShape>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let (mesh, shape) = base.into_inner();

    let Some(mesh) = meshes.get_mut(mesh) else {
        return;
    };

    // Calculate half size (mm)
    let (x, z) = (trigger.0.x * 0.5, trigger.0.y * 0.5);

    if let Some(VertexAttributeValues::Float32x3(positions)) =
        mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
    {
        for (index, position) in positions.iter_mut().enumerate() {
            // Named Vertices
            match shape {
                BaseShape::L => {
                    if shape.vertices(BasePoint::A).contains(&index) {
                        *position = [0., position[1], z];
                        continue;
                    }

                    if shape.vertices(BasePoint::B).contains(&index) {
                        *position = [0., position[1], 0.];
                        continue;
                    }

                    if shape.vertices(BasePoint::C).contains(&index) {
                        *position = [x, position[1], 0.];
                        continue;
                    }
                }
                BaseShape::N => {
                    if shape.vertices(BasePoint::A).contains(&index) {
                        let p0 = x / 3.;
                        *position = [-p0, position[1], z];
                        continue;
                    }

                    if shape.vertices(BasePoint::B).contains(&index) {
                        let p0 = x / 3.;
                        let p1 = z / 3.;
                        *position = [-p0, position[1], p1];
                        continue;
                    }

                    if shape.vertices(BasePoint::C).contains(&index) {
                        let p0 = x / 3.;
                        let p1 = z / 3.;
                        *position = [p0, position[1], p1];
                        continue;
                    }

                    if shape.vertices(BasePoint::D).contains(&index) {
                        let p0 = x / 3.;
                        *position = [p0, position[1], z];
                        continue;
                    }
                }
                _ => (),
            }

            // Normal Vertices
            *position = [
                position[0].signum() * x,
                position[1],
                position[2].signum() * z,
            ];
        }
    }
}

#[inline]
const fn min_distance(distance: f32) -> f32 {
    if distance.abs() < MIN_DISTANCE {
        distance.signum() * MIN_DISTANCE
    } else {
        distance
    }
}

struct BaseMeshNormals(Vec<[f32; 3]>);

impl BaseMeshNormals {
    fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    // const NORMAL_X_POS: [f32; 3] = [1., 0., 0.];
    // const NORMAL_X_NEG: [f32; 3] = [-1., 0., 0.];
    // const NORMAL_Y_POS: [f32; 3] = [0., 1., 0.];
    // const NORMAL_Y_NEG: [f32; 3] = [0., -1., 0.];
    // const NORMAL_Z_POS: [f32; 3] = [0., 0., 1.];
    // const NORMAL_Z_NEG: [f32; 3] = [0., 0., -1.];
}

struct BaseMeshIndices(Vec<u32>);

impl BaseMeshIndices {
    const OFFSET_TOP_SIDE1: u32 = 1;
    const OFFSET_TOP_SIDE2: u32 = 2;
    const OFFSET_BOTTOM: u32 = 3;
    const OFFSET_BOTTOM_SIDE1: u32 = 4;
    const OFFSET_BOTTOM_SIDE2: u32 = 5;

    fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    fn add_top_triangle(&mut self, index0: usize, index1: usize, index2: usize) {
        self.0.extend_from_slice(&[
            index0 as u32 * VERTEX_PER_NODE,
            index1 as u32 * VERTEX_PER_NODE,
            index2 as u32 * VERTEX_PER_NODE,
        ]);
    }

    fn add_bottom_triangle(&mut self, index0: usize, index1: usize, index2: usize) {
        self.0.extend_from_slice(&[
            index0 as u32 * VERTEX_PER_NODE + Self::OFFSET_BOTTOM,
            index1 as u32 * VERTEX_PER_NODE + Self::OFFSET_BOTTOM,
            index2 as u32 * VERTEX_PER_NODE + Self::OFFSET_BOTTOM,
        ]);
    }

    fn add_side1_triangle(&mut self, index0: usize, index1: usize, index2: usize) {
        self.0.extend_from_slice(&[
            index0 as u32 * VERTEX_PER_NODE + Self::OFFSET_TOP_SIDE1,
            index1 as u32 * VERTEX_PER_NODE + Self::OFFSET_TOP_SIDE1,
            index2 as u32 * VERTEX_PER_NODE + Self::OFFSET_BOTTOM_SIDE1,
        ]);
    }

    fn add_side2_triangle(&mut self, index0: usize, index1: usize, index2: usize) {
        self.0.extend_from_slice(&[
            index0 as u32 * VERTEX_PER_NODE + Self::OFFSET_TOP_SIDE2,
            index1 as u32 * VERTEX_PER_NODE + Self::OFFSET_BOTTOM_SIDE2,
            index2 as u32 * VERTEX_PER_NODE + Self::OFFSET_BOTTOM_SIDE2,
        ]);
    }
}

#[derive(Resource)]
struct Base {
    nodes: NodeList,
    convex_turn: Turn,
    mesh: Mesh,
}

impl Base {
    fn builder() -> BaseBuilder<Empty> {
        BaseBuilder::<Empty> { geometry: Empty }
    }
}

trait Geometry {}

struct BaseBuilder<G: Geometry> {
    geometry: G,
}

struct Empty;

impl Geometry for Empty {}

impl BaseBuilder<Empty> {
    fn start_point(self, x: f32, z: f32) -> BaseBuilder<Point> {
        BaseBuilder {
            geometry: Point {
                node: Position::new(x, z),
            },
        }
    }
}

struct Point {
    node: Position,
}

impl Geometry for Point {}

impl BaseBuilder<Point> {
    fn move_x_to(self, x: f32) -> BaseBuilder<Line<X>> {
        let x = self.geometry.node.x + min_distance(x - self.geometry.node.x);
        let z = self.geometry.node.z;

        BaseBuilder {
            geometry: Line::from_position_pair(self.geometry.node, Position::new(x, z)),
        }
    }

    fn move_x_with(self, distance: f32) -> BaseBuilder<Line<X>> {
        let x = self.geometry.node.x + min_distance(distance);
        let z = self.geometry.node.z;

        BaseBuilder {
            geometry: Line::from_position_pair(self.geometry.node, Position::new(x, z)),
        }
    }

    fn move_z_to(self, z: f32) -> BaseBuilder<Line<Z>> {
        let x = self.geometry.node.x;
        let z = self.geometry.node.z + min_distance(z - self.geometry.node.z);

        BaseBuilder {
            geometry: Line::from_position_pair(self.geometry.node, Position::new(x, z)),
        }
    }

    fn move_z_with(self, distance: f32) -> BaseBuilder<Line<Z>> {
        let x = self.geometry.node.x;
        let z = self.geometry.node.z + min_distance(distance);

        BaseBuilder {
            geometry: Line::from_position_pair(self.geometry.node, Position::new(x, z)),
        }
    }
}

trait PrevMove {}

struct X;
impl PrevMove for X {}

struct Z;
impl PrevMove for Z {}

struct Line<P: PrevMove> {
    nodes: NodeList,
    prev_move: std::marker::PhantomData<P>,
}

impl<P: PrevMove> Line<P> {
    fn from_position_pair(position0: Position, position1: Position) -> Self {
        let mut nodes = NodeList::with_capacity(2);

        nodes.push(Node::new(position0, 1, 1));
        nodes.push(Node::new(position1, 0, 0));

        Self {
            nodes,
            prev_move: std::marker::PhantomData,
        }
    }
}

impl<P: PrevMove> Geometry for Line<P> {}

impl BaseBuilder<Line<X>> {
    fn move_z_to(self, z: f32) -> BaseBuilder<Triangle<Z>> {
        let middle = self.geometry.nodes.len() - 1;
        let x = self.geometry.nodes[middle].position.x;
        let z = self.geometry.nodes[middle].position.z
            + min_distance(z - self.geometry.nodes[middle].position.z);

        BaseBuilder {
            geometry: Triangle::from_line(self.geometry, Position::new(x, z)),
        }
    }

    fn move_z_with(self, distance: f32) -> BaseBuilder<Triangle<Z>> {
        let middle = self.geometry.nodes.len() - 1;
        let x = self.geometry.nodes[middle].position.x;
        let z = self.geometry.nodes[middle].position.z + min_distance(distance);

        BaseBuilder {
            geometry: Triangle::from_line(self.geometry, Position::new(x, z)),
        }
    }
}

impl BaseBuilder<Line<Z>> {
    fn move_x_to(self, x: f32) -> BaseBuilder<Triangle<X>> {
        let middle = self.geometry.nodes.len() - 1;
        let x = self.geometry.nodes[middle].position.x
            + min_distance(x - self.geometry.nodes[middle].position.x);
        let z = self.geometry.nodes[middle].position.z;

        BaseBuilder {
            geometry: Triangle::from_line(self.geometry, Position::new(x, z)),
        }
    }

    fn move_x_with(self, distance: f32) -> BaseBuilder<Triangle<X>> {
        let middle = self.geometry.nodes.len() - 1;
        let x = self.geometry.nodes[middle].position.x + min_distance(distance);
        let z = self.geometry.nodes[middle].position.z;

        BaseBuilder {
            geometry: Triangle::from_line(self.geometry, Position::new(x, z)),
        }
    }
}

struct Triangle<P: PrevMove> {
    nodes: NodeList,
    prev_move: std::marker::PhantomData<P>,
}

impl<To: PrevMove> Triangle<To> {
    fn from_line<From: PrevMove>(line: Line<From>, position: Position) -> Self {
        let mut nodes = line.nodes;

        nodes.push(Node::new(position, 1, 0));
        nodes[1].connection.next = 2;
        nodes[0].connection.prev = 2;

        let p_1 = &nodes[0].position;
        let p_2 = &nodes[1].position;
        let p_3 = &nodes[2].position;

        let v_12 = *p_2 - *p_1;
        let v_13 = *p_3 - *p_1;

        if v_12.cross(v_13) > 0. {
            nodes[1].turn = Turn::Clockwise;
        } else {
            nodes[1].turn = Turn::CounterClockwise;
        }

        Self {
            nodes,
            prev_move: std::marker::PhantomData,
        }
    }
}

impl<P: PrevMove> Geometry for Triangle<P> {}

impl BaseBuilder<Triangle<X>> {
    fn move_z_to(self, z: f32) -> BaseBuilder<Polygon<Z>> {
        let middle = self.geometry.nodes.len() - 1;
        let x = self.geometry.nodes[middle].position.x;
        let z = self.geometry.nodes[middle].position.z
            + min_distance(z - self.geometry.nodes[middle].position.z);

        BaseBuilder {
            geometry: Polygon::from_triangle(self.geometry, Position::new(x, z)),
        }
    }

    fn move_z_with(self, distance: f32) -> BaseBuilder<Polygon<Z>> {
        let middle = self.geometry.nodes.len() - 1;
        let x = self.geometry.nodes[middle].position.x;
        let z = self.geometry.nodes[middle].position.z + min_distance(distance);

        BaseBuilder {
            geometry: Polygon::from_triangle(self.geometry, Position::new(x, z)),
        }
    }
}

impl BaseBuilder<Triangle<Z>> {
    fn move_x_to(self, x: f32) -> BaseBuilder<Polygon<X>> {
        let middle = self.geometry.nodes.len() - 1;
        let x = self.geometry.nodes[middle].position.x
            + min_distance(x - self.geometry.nodes[middle].position.x);
        let z = self.geometry.nodes[middle].position.z;

        BaseBuilder {
            geometry: Polygon::from_triangle(self.geometry, Position::new(x, z)),
        }
    }

    fn move_x_with(self, distance: f32) -> BaseBuilder<Polygon<X>> {
        let middle = self.geometry.nodes.len() - 1;
        let x = self.geometry.nodes[middle].position.x + min_distance(distance);
        let z = self.geometry.nodes[middle].position.z;

        BaseBuilder {
            geometry: Polygon::from_triangle(self.geometry, Position::new(x, z)),
        }
    }
}

struct Polygon<P: PrevMove> {
    nodes: NodeList,
    cw_list: std::collections::VecDeque<usize>,
    ccw_list: std::collections::VecDeque<usize>,
    prev_move: std::marker::PhantomData<P>,
}

impl<To: PrevMove> Polygon<To> {
    fn from_triangle<From: PrevMove>(triangle: Triangle<From>, position: Position) -> Self {
        let mut polygon = Self {
            nodes: triangle.nodes,
            cw_list: std::collections::VecDeque::new(),
            ccw_list: std::collections::VecDeque::new(),
            prev_move: std::marker::PhantomData,
        };

        polygon.nodes.push(Node::new(position, 2, 0));
        polygon.nodes[2].connection.next = 3;
        polygon.nodes[0].connection.prev = 3;

        if polygon.nodes[1].turn == Turn::Clockwise {
            polygon.cw_list.push_back(1);
        } else {
            polygon.ccw_list.push_back(1);
        }

        polygon.calc_node_turn(2);

        polygon
    }
}

impl<P: PrevMove> Polygon<P> {
    fn insert_node(&mut self, position: Position) {
        let new_i = self.nodes.len();

        self.nodes.push(Node::new(position, new_i - 1, 0));
        self.nodes[new_i - 1].connection.next = new_i;
        self.nodes[0].connection.prev = new_i;

        self.calc_node_turn(self.nodes[new_i].connection.prev);
    }

    fn calc_node_turn(&mut self, index: usize) -> Turn {
        let p_1 = &self.nodes[self.nodes[index].connection.prev].position;
        let p_2 = &self.nodes[index].position;
        let p_3 = &self.nodes[self.nodes[index].connection.next].position;

        let v_12 = *p_2 - *p_1;
        let v_13 = *p_3 - *p_1;

        if v_12.cross(v_13) > 0. {
            if self.nodes[index].turn == Turn::Undefined {
                self.cw_list.push_back(index);
            }
            self.nodes[index].turn = Turn::Clockwise
        } else {
            if self.nodes[index].turn == Turn::Undefined {
                self.ccw_list.push_back(index);
            }
            self.nodes[index].turn = Turn::CounterClockwise
        }

        self.nodes[index].turn
    }
}

impl<P: PrevMove> Geometry for Polygon<P> {}

impl BaseBuilder<Polygon<X>> {
    fn move_z_to(mut self, z: f32) -> BaseBuilder<Polygon<Z>> {
        let middle = self.geometry.nodes.len() - 1;
        let x = self.geometry.nodes[middle].position.x;
        let z = self.geometry.nodes[middle].position.z
            + min_distance(z - self.geometry.nodes[middle].position.z);

        self.geometry.insert_node(Position::new(x, z));

        BaseBuilder {
            geometry: Polygon {
                nodes: self.geometry.nodes,
                cw_list: self.geometry.cw_list,
                ccw_list: self.geometry.ccw_list,
                prev_move: std::marker::PhantomData,
            },
        }
    }

    fn move_z_with(mut self, distance: f32) -> BaseBuilder<Polygon<Z>> {
        let middle = self.geometry.nodes.len() - 1;
        let x = self.geometry.nodes[middle].position.x;
        let z = self.geometry.nodes[middle].position.z + min_distance(distance);

        self.geometry.insert_node(Position::new(x, z));

        BaseBuilder {
            geometry: Polygon {
                nodes: self.geometry.nodes,
                cw_list: self.geometry.cw_list,
                ccw_list: self.geometry.ccw_list,
                prev_move: std::marker::PhantomData,
            },
        }
    }
}

impl BaseBuilder<Polygon<Z>> {
    fn move_x_to(mut self, x: f32) -> BaseBuilder<Polygon<X>> {
        let middle = self.geometry.nodes.len() - 1;
        let x = self.geometry.nodes[middle].position.x
            + min_distance(x - self.geometry.nodes[middle].position.x);
        let z = self.geometry.nodes[middle].position.z;

        self.geometry.insert_node(Position::new(x, z));

        BaseBuilder {
            geometry: Polygon {
                nodes: self.geometry.nodes,
                cw_list: self.geometry.cw_list,
                ccw_list: self.geometry.ccw_list,
                prev_move: std::marker::PhantomData,
            },
        }
    }

    fn move_x_with(mut self, distance: f32) -> BaseBuilder<Polygon<X>> {
        let middle = self.geometry.nodes.len() - 1;
        let x = self.geometry.nodes[middle].position.x + min_distance(distance);
        let z = self.geometry.nodes[middle].position.z;

        self.geometry.insert_node(Position::new(x, z));

        BaseBuilder {
            geometry: Polygon {
                nodes: self.geometry.nodes,
                cw_list: self.geometry.cw_list,
                ccw_list: self.geometry.ccw_list,
                prev_move: std::marker::PhantomData,
            },
        }
    }
}

impl<P: PrevMove> BaseBuilder<Polygon<P>> {
    fn build(mut self) -> Base {
        let y = HALF_SIZE_DEFAULT.y;
        let nodes_len = self.geometry.nodes.len();
        let mut nodes_temp = NodeList(self.geometry.nodes.clone());

        let mut mesh_vertices = Vec::with_capacity(nodes_len * VERTEX_PER_NODE as usize);
        // TODO! Add mesh normals
        // let mut mesh_normals: BaseMeshNormals::with_capacity(nodes_len * VERTEX_PER_NODE as usize);
        let mut mesh_indices = BaseMeshIndices::with_capacity(
            2 * (nodes_len - 2) * INDEX_PER_TRIANGLE // top + bottom
            + 2 * nodes_len * INDEX_PER_TRIANGLE, // sides
        );

        // Caclulate turn of first/last node
        self.geometry.calc_node_turn(0);
        self.geometry.calc_node_turn(nodes_len - 1);

        // If there are more cw turns than ccw, then the nodes are connected
        // in clockwise order. In this case each cw turn is convex, while
        // counter clockwise turns start out as reflexes.
        let (convex_turn, mut convex_list) =
            if self.geometry.cw_list.len() > self.geometry.ccw_list.len() {
                (Turn::Clockwise, self.geometry.cw_list)
            } else {
                (Turn::CounterClockwise, self.geometry.ccw_list)
            };

        // Each node point represent 6 vertex
        // Top (Y+) vertices building up 3 triangles (top + 2 side)
        // Bottom (Y-) vertices building up 3 triangles (bottom + 2 side)
        for (
            curr_i,
            Node {
                position: Position { x, z },
                connection: Connection { next, .. },
                ..
            },
        ) in self.geometry.nodes.iter().enumerate()
        {
            mesh_vertices.extend_from_slice(&[
                [*x, y, *z],  // top
                [*x, y, *z],  // top side 1
                [*x, y, *z],  // top side 2
                [*x, -y, *z], // bottom
                [*x, -y, *z], // bottom side 1
                [*x, -y, *z], // bottom side 2
            ]);

            let next_i = *next;

            if convex_turn == Turn::Clockwise {
                mesh_indices.add_side1_triangle(curr_i, next_i, next_i);
                mesh_indices.add_side2_triangle(curr_i, next_i, curr_i);
            } else {
                mesh_indices.add_side1_triangle(next_i, curr_i, next_i);
                mesh_indices.add_side2_triangle(curr_i, curr_i, next_i);
            }
        }

        let (mut triangle_empty, mut n);
        let (mut start, mut end, mut count) = (0, nodes_len, nodes_len);
        while count > 3 {
            let Some(curr_i) = convex_list.pop_front() else {
                n = start;
                for _ in 0..count {
                    nodes_temp.is_turn_changed(n);
                    n = nodes_temp[n].connection.next;
                }

                continue;
            };

            let prev_i = nodes_temp[curr_i].connection.prev;
            let next_i = nodes_temp[curr_i].connection.next;

            let p_1 = nodes_temp[prev_i].position;
            let p_2 = nodes_temp[curr_i].position;
            let p_3 = nodes_temp[next_i].position;

            let v_12 = p_2 - p_1;
            let v_23 = p_3 - p_2;
            let v_31 = p_1 - p_3;

            triangle_empty = true;
            n = start;
            for _ in 0..count {
                if n == prev_i || n == curr_i || n == next_i {
                    n = nodes_temp[n].connection.next;
                    continue;
                }

                let p_n = nodes_temp[n].position;

                let v_1n = p_n - p_1;
                let v_2n = p_n - p_2;
                let v_3n = p_n - p_3;

                // Check n-th node position is inside the triangle defined by p_1-3
                if convex_turn == Turn::Clockwise {
                    if v_12.cross(v_1n) >= 0. && v_23.cross(v_2n) >= 0. && v_31.cross(v_3n) >= 0. {
                        triangle_empty = false;
                        break;
                    }
                } else {
                    if v_12.cross(v_1n) <= 0. && v_23.cross(v_2n) <= 0. && v_31.cross(v_3n) <= 0. {
                        triangle_empty = false;
                        break;
                    }
                }

                n = nodes_temp[n].connection.next;
            }

            if triangle_empty {
                if convex_turn == Turn::Clockwise {
                    mesh_indices.add_top_triangle(prev_i, next_i, curr_i);
                    mesh_indices.add_bottom_triangle(prev_i, curr_i, next_i);
                } else {
                    mesh_indices.add_top_triangle(prev_i, curr_i, next_i);
                    mesh_indices.add_bottom_triangle(prev_i, next_i, curr_i);
                }

                nodes_temp[prev_i].connection.next = nodes_temp[curr_i].connection.next;
                nodes_temp[next_i].connection.prev = nodes_temp[curr_i].connection.prev;

                if nodes_temp[prev_i].turn != convex_turn && nodes_temp.is_turn_changed(prev_i) {
                    convex_list.push_back(prev_i);
                }

                if nodes_temp[next_i].turn != convex_turn && nodes_temp.is_turn_changed(next_i) {
                    convex_list.push_back(next_i);
                }

                if curr_i == start {
                    start = nodes_temp[curr_i].connection.next;
                } else if curr_i == end {
                    end = nodes_temp[curr_i].connection.prev;
                }

                count -= 1;
            } else {
                convex_list.push_back(curr_i);
            }
        }

        let start_prev_i = nodes_temp[start].connection.prev;
        let start_next_i = nodes_temp[start].connection.next;

        if convex_turn == Turn::Clockwise {
            mesh_indices.add_top_triangle(start, start_prev_i, start_next_i);
            mesh_indices.add_bottom_triangle(start, start_next_i, start_prev_i);
        } else {
            mesh_indices.add_top_triangle(start, start_next_i, start_prev_i);
            mesh_indices.add_bottom_triangle(start, start_prev_i, start_next_i);
        }

        if convex_turn == Turn::Clockwise {
            info!(
                "Base mesh is built in clockwise order, with {} nodes.",
                nodes_len
            )
        } else {
            info!(
                "Base mesh is built in counter clockwise order, with {} nodes.",
                nodes_len
            )
        }

        println!("TEst2: {}, {}", mesh_indices.0.len(), mesh_vertices.len());

        Base {
            nodes: self.geometry.nodes,
            convex_turn,
            mesh: Mesh::new(
                PrimitiveTopology::TriangleList,
                RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
            )
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_vertices)
            .with_inserted_indices(Indices::U32(mesh_indices.0)),
        }
    }
}

/// Describes the [Node]'s connection turn
///
/// ```
/// Clockwise        Counter Clockwise
/// (Right)          (Left)
///
/// 0--->1           1<---0
///      |           |
///      v           v
///      2           2
/// ```
#[derive(Debug, PartialEq, Clone, Copy)]
enum Turn {
    Undefined,
    Clockwise,
    CounterClockwise,
}

impl std::ops::Not for Turn {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Turn::Undefined => Turn::Undefined,
            Turn::Clockwise => Turn::CounterClockwise,
            Turn::CounterClockwise => Turn::Clockwise,
        }
    }
}

#[derive(Deref, DerefMut)]
struct NodeList(Vec<Node>);

impl NodeList {
    fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    fn is_turn_changed(&mut self, index: usize) -> bool {
        let p_1 = &self[self[index].connection.prev].position;
        let p_2 = &self[index].position;
        let p_3 = &self[self[index].connection.next].position;

        let v_12 = *p_2 - *p_1;
        let v_13 = *p_3 - *p_1;

        let cross = v_12.cross(v_13);

        if cross > 0. && self[index].turn == Turn::CounterClockwise {
            self[index].turn = Turn::Clockwise;
            return true;
        }

        if cross < 0. && self[index].turn == Turn::Clockwise {
            self[index].turn = Turn::CounterClockwise;
            return true;
        }

        false
    }
}

#[derive(Debug, Clone, Copy)]
struct Node {
    connection: Connection,
    position: Position,
    turn: Turn,
}

impl Node {
    fn new(position: Position, prev: usize, next: usize) -> Self {
        Self {
            connection: Connection { prev, next },
            position,
            turn: Turn::Undefined,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Connection {
    prev: usize,
    next: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Position {
    x: f32,
    z: f32,
}

impl Position {
    const fn new(x: f32, z: f32) -> Self {
        Self { x, z }
    }
}

impl std::ops::Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            z: self.z + rhs.z,
            ..self
        }
    }
}

impl std::ops::Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            z: self.z - rhs.z,
            ..self
        }
    }
}

impl std::ops::Mul for Position {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            z: self.z * rhs.z,
            ..self
        }
    }
}

impl std::ops::Div for Position {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            z: self.z / rhs.z,
            ..self
        }
    }
}

trait CrossProduct<Rhs = Self> {
    type Output;

    fn cross(self, rhs: Rhs) -> Self::Output;
}

impl CrossProduct for Position {
    type Output = f32;

    fn cross(self, rhs: Self) -> Self::Output {
        self.x * rhs.z - self.z * rhs.x
    }
}

#[test]
fn test_base_builder() {
    let x = 10000.;
    let z = 1234.;

    // -x,-z      x, -z
    // 0----------3
    // |          |
    // |     x    |
    // |          |
    // 1----------2
    // -x,z       x, z

    let base_ccw = Base::builder()
        .start_point(-x, -z)
        .move_z_to(z)
        .move_x_to(x)
        .move_z_to(-z);

    assert_eq!(
        base_ccw
            .geometry
            .nodes
            .iter()
            .map(|node| node.position)
            .collect::<Vec<Position>>(),
        vec![
            Position::new(-x, -z),
            Position::new(-x, z),
            Position::new(x, z),
            Position::new(x, -z),
        ]
    );
    assert_eq!(base_ccw.geometry.cw_list.len(), 0);
    assert_eq!(base_ccw.geometry.ccw_list.len(), 2);

    let x = MIN_DISTANCE * 0.5;
    let z = MIN_DISTANCE * 0.5;

    // -x,-z      x, -z
    // 0----------1
    // |          |
    // |     x    |
    // |          |
    // 3----------2
    // -x,z       x, z

    // Since the move distance is less than MIN_DISTANCE it should correct
    // the new node to have at least the minimum distance
    let mut base_cw = Base::builder()
        .start_point(-x, -z)
        .move_x_to(0.)
        .move_z_with(z)
        .move_x_with(-x);

    base_cw.geometry.calc_node_turn(0);
    base_cw.geometry.calc_node_turn(3);

    assert_eq!(
        base_cw
            .geometry
            .nodes
            .iter()
            .map(|node| node.position)
            .collect::<Vec<Position>>(),
        vec![
            Position::new(-x, -z),
            Position::new(x, -z),
            Position::new(x, z),
            Position::new(-x, z),
        ]
    );
    assert_eq!(base_cw.geometry.cw_list.len(), 4);
    assert_eq!(base_cw.geometry.ccw_list.len(), 0);
}
